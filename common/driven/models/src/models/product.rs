use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_dynamodb::types::{AttributeValue, KeysAndAttributes};
use error::HexagonalError;
use mockall::automock;
use persistance_repository::DynamoDBSingleTableRepository;
use serde::{Deserialize, Serialize};

use crate::{default_time, new_uuid, DynamoDbModel};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Product {
    #[serde(default = "new_uuid")]
    pub id: String,
    pub product_name: String,
    pub price_cents: i32,
    pub description: String,
    #[serde(default = "default_time")]
    pub created_at: String,
    #[serde(default = "default_time")]
    pub updated_at: String,
}

impl Product {
    pub fn new(product_name: String, price_cents: i32, description: String) -> Self {
        Self {
            id: new_uuid(),
            product_name,
            price_cents,
            description,
            created_at: default_time(),
            updated_at: default_time(),
        }
    }
}

impl DynamoDbModel for Product {
    fn from_attr_map(attr_map: std::collections::HashMap<String, AttributeValue>) -> Self {
        Product {
            id: attr_map.get("id").unwrap().as_s().unwrap().to_string(),
            product_name: attr_map
                .get("product_name")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string(),
            price_cents: attr_map
                .get("price_cents")
                .unwrap()
                .as_n()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            description: attr_map
                .get("description")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string(),
            created_at: attr_map
                .get("created_at")
                .unwrap()
                .as_n()
                .unwrap()
                .to_string(),
            updated_at: attr_map
                .get("updated_at")
                .unwrap()
                .as_n()
                .unwrap()
                .to_string(),
        }
    }

    fn into_attr_map(&self) -> std::collections::HashMap<String, AttributeValue> {
        let mut attr_map = std::collections::HashMap::new();
        attr_map.insert(
            "Pkey".to_string(),
            AttributeValue::S("PRODUCT#".to_string() + &self.id.to_string()),
        );
        attr_map.insert("Skey".to_string(), AttributeValue::S("-".to_string()));
        attr_map.insert("id".to_string(), AttributeValue::S(self.id.to_string()));
        attr_map.insert(
            "product_name".to_string(),
            AttributeValue::S(self.product_name.to_string()),
        );
        attr_map.insert(
            "price_cents".to_string(),
            AttributeValue::N(self.price_cents.to_string()),
        );
        attr_map.insert(
            "description".to_string(),
            AttributeValue::S(self.description.to_string()),
        );
        attr_map.insert(
            "created_at".to_string(),
            AttributeValue::N(self.created_at.to_string()),
        );
        attr_map.insert(
            "updated_at".to_string(),
            AttributeValue::N(self.updated_at.to_string()),
        );
        attr_map
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MutableProduct {
    pub product_name: Option<String>,
    pub price_cents: Option<i32>,
    pub description: Option<String>,
}

impl MutableProduct {
    pub fn into_attr_map(&self) -> std::collections::HashMap<String, AttributeValue> {
        let mut attr_map = std::collections::HashMap::new();
        if self.product_name.is_some() {
            attr_map.insert(
                ":product_name".to_string(),
                AttributeValue::S(self.product_name.clone().unwrap()),
            );
        }
        if self.price_cents.is_some() {
            attr_map.insert(
                ":price_cents".to_string(),
                AttributeValue::N(self.price_cents.unwrap().to_string()),
            );
        }
        if self.description.is_some() {
            attr_map.insert(
                ":description".to_string(),
                AttributeValue::S(self.description.clone().unwrap().to_string()),
            );
        }
        attr_map.insert(":updated_at".to_string(), AttributeValue::N(default_time()));
        attr_map
    }
}

// Conceptually the traits of the repository are our "ports" and the implementations are our "adaptors"
#[automock]
#[async_trait]
pub trait ProductRepositoryPort {
    async fn product_get_by_id(&self, id: &String) -> Result<Option<Product>, HexagonalError>;
    async fn product_get_by_ids(&self, id: &Vec<String>) -> Result<Vec<Product>, HexagonalError>;
    async fn product_create(&self, product: &Product) -> Result<Product, HexagonalError>;
    async fn product_update_by_id(
        &self,
        id: &String,
        product_update: &MutableProduct,
    ) -> Result<Product, HexagonalError>;
    async fn product_delete_by_id(&self, id: &String) -> Result<Product, HexagonalError>;
}

pub struct ProductRepositoryAdaptor<'a> {
    persistance_repository: &'a DynamoDBSingleTableRepository,
}

impl<'a> ProductRepositoryAdaptor<'a> {
    pub fn new(
        persistance_repository: &'a DynamoDBSingleTableRepository,
    ) -> ProductRepositoryAdaptor<'a> {
        ProductRepositoryAdaptor {
            persistance_repository,
        }
    }
}

#[async_trait]
impl<'a> ProductRepositoryPort for ProductRepositoryAdaptor<'a> {
    async fn product_get_by_id(&self, id: &String) -> Result<Option<Product>, HexagonalError> {
        let result = self
            .persistance_repository
            .get_item_primary("PRODUCT#".to_string() + id, "-".to_string())
            .await;

        match result {
            Ok(result) => match result.item {
                Some(item) => Ok(Some(Product::from_attr_map(item))),
                None => Ok(None),
            },
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to get product".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn product_get_by_ids(&self, id: &Vec<String>) -> Result<Vec<Product>, HexagonalError> {
        let get_item_key_vec = id
            .iter()
            .map(|id| {
                let mut key = HashMap::new();
                key.insert(
                    "Pkey".to_string(),
                    AttributeValue::S("PRODUCT#".to_string() + id),
                );
                key.insert("Skey".to_string(), AttributeValue::S("-".to_string()));
                key
            })
            .collect::<Vec<HashMap<String, AttributeValue>>>();

        let keys_and_attributes = KeysAndAttributes::builder()
            .set_keys(Some(get_item_key_vec))
            .build();

        let mut request_items = HashMap::new();
        request_items.insert(
            self.persistance_repository.table_name.clone(),
            keys_and_attributes,
        );

        let result = self
            .persistance_repository
            .client
            .batch_get_item()
            .set_request_items(Some(request_items))
            .send()
            .await;

        match result {
            Ok(result) => match result.responses {
                Some(item) => Ok(item
                    .get(&self.persistance_repository.table_name)
                    .unwrap()
                    .iter()
                    .map(|item| Product::from_attr_map(item.clone()))
                    .collect()),
                None => Ok(Vec::new()),
            },
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to get product".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn product_create(&self, product: &Product) -> Result<Product, HexagonalError> {
        let result = self
            .persistance_repository
            .put_new_item(product.into_attr_map())
            .await;

        println!("result: {:?}", result);

        match result {
            Ok(_) => Ok(product.clone()),
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to create product".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn product_update_by_id(
        &self,
        id: &String,
        product_update: &MutableProduct,
    ) -> Result<Product, HexagonalError> {
        let mut update_expression = "SET ".to_string();
        let mut attr_names = HashMap::new();

        if product_update.product_name.is_some() {
            update_expression.push_str("#name_key = :product_name, ");
            attr_names.insert("#name_key".to_string(), "product_name".to_string());
        }

        if product_update.price_cents.is_some() {
            update_expression.push_str("#price_cents_key = :price_cents, ");
            attr_names.insert("#price_cents_key".to_string(), "price_cents".to_string());
        }

        if product_update.description.is_some() {
            update_expression.push_str("#description_key = :description, ");
            attr_names.insert("#description_key".to_string(), "description".to_string());
        }

        update_expression.push_str("updated_at = :updated_at");

        let result = self
            .persistance_repository
            .update_item(
                "PRODUCT#".to_string() + id,
                "-".to_string(),
                update_expression,
                Some(attr_names),
                Some(product_update.into_attr_map()),
            )
            .await;

        match result {
            Ok(output) => Ok(Product::from_attr_map(output.attributes.unwrap())),
            Err(e) => match e.is_conditional_check_failed_exception() {
                true => Err(HexagonalError {
                    error: error::HexagonalErrorCode::NotFound,
                    message: "Unable to update product, does not exist".to_string(),
                    trace: "".to_string(),
                }),
                false => Err(HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "Unable to update product".to_string(),
                    trace: e.to_string(),
                }),
            },
        }
    }

    async fn product_delete_by_id(&self, id: &String) -> Result<Product, HexagonalError> {
        let result = self
            .persistance_repository
            .delete_item("PRODUCT#".to_string() + id, "-".to_string())
            .await;

        match result {
            Ok(output) => match output.attributes {
                Some(attributes) => Ok(Product::from_attr_map(attributes)),
                None => Err(HexagonalError {
                    error: error::HexagonalErrorCode::NotFound,
                    message: "Unable to delete product, does not exist".to_string(),
                    trace: "".to_string(),
                }),
            },
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to delete product".to_string(),
                trace: e.to_string(),
            }),
        }
    }
}
