use std::collections::HashMap;

use async_trait::async_trait;
use error::HexagonalError;
use mockall::automock;
use persistance_repository::DynamoDBSingleTableRepository;
use serde::{Deserialize, Serialize};
use aws_sdk_dynamodb::types::AttributeValue;


use crate::{default_time, DynamoDbModel, new_uuid};



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Product {
    #[serde(default = "new_uuid")]
    pub id: String,
    pub name: String,
    pub price_cents: i32,
    pub description: String,
    #[serde(default = "default_time")]
    pub created_at: String,
    #[serde(default = "default_time")]
    pub updated_at: String,
}

impl Product {
    pub fn new(name: String, price_cents: i32, description: String) -> Self {
        Self {
            id: new_uuid(),
            name,
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
            name: attr_map.get("name").unwrap().as_s().unwrap().to_string(),
            price_cents: attr_map.get("price_cents").unwrap().as_n().unwrap().parse::<i32>().unwrap(),
            description: attr_map.get("description").unwrap().as_s().unwrap().to_string(),
            created_at: attr_map.get("created_at").unwrap().as_n().unwrap().to_string(),
            updated_at: attr_map.get("updated_at").unwrap().as_n().unwrap().to_string(),
        }
    }

    fn into_attr_map(&self) -> std::collections::HashMap<String, AttributeValue> {
        let mut attr_map = std::collections::HashMap::new();
        attr_map.insert(
            "Pkey".to_string(),
            AttributeValue::S("user_".to_string() + &self.id.to_string()),
        );
        attr_map.insert("Skey".to_string(), AttributeValue::S("-".to_string()));
        attr_map.insert("id".to_string(), AttributeValue::S(self.id.to_string()));
        attr_map.insert("name".to_string(), AttributeValue::S(self.name.to_string()));
        attr_map.insert("price_cents".to_string(), AttributeValue::N(self.price_cents.to_string()));
        attr_map.insert("description".to_string(), AttributeValue::S(self.description.to_string()));
        attr_map.insert("created_at".to_string(), AttributeValue::N(self.created_at.to_string()));
        attr_map.insert("updated_at".to_string(), AttributeValue::N(self.updated_at.to_string()));
        attr_map
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MutableProduct {
    pub name: Option<String>,
    pub price_cents: Option<i32>,
    pub description: Option<String>,
}

impl MutableProduct {
    pub fn into_attr_map(&self) -> std::collections::HashMap<String, AttributeValue> {
        let mut attr_map = std::collections::HashMap::new();
        if self.name.is_some() {
            attr_map.insert("name".to_string(), AttributeValue::S(self.name.clone().unwrap()));
        }
        if self.price_cents.is_some() {
            attr_map.insert("price_cents".to_string(), AttributeValue::N(self.price_cents.unwrap().to_string()));
        }
        if self.description.is_some() {
            attr_map.insert("description".to_string(), AttributeValue::S(self.description.clone().unwrap().to_string()));
        }
        attr_map.insert("updated_at".to_string(), AttributeValue::N(default_time()));
        attr_map
    }
}

// Conceptually the traits of the repository are our "ports" and the implementations are our "adaptors"
#[automock]
#[async_trait]
pub trait ProductRepositoryPort {
    async fn product_get_by_id(&self, id: &String) -> Result<Option<Product>, HexagonalError>;
    async fn product_create(&self, product: &Product) -> Result<Product, HexagonalError>;
    async fn product_update_by_id(&self, id: &String, product_update: &MutableProduct) -> Result<Product, HexagonalError>;
    async fn product_delete_by_id(&self, id: &String) -> Result<Product, HexagonalError>;
}

pub struct ProductRepositoryAdaptor<'a> {
    persistance_repository: &'a DynamoDBSingleTableRepository,
}

impl<'a> ProductRepositoryAdaptor<'a> {
    pub fn new(persistance_repository: &'a DynamoDBSingleTableRepository) -> ProductRepositoryAdaptor<'a> {
        ProductRepositoryAdaptor {
            persistance_repository,
        }
    }
}

#[async_trait]
impl<'a> ProductRepositoryPort for ProductRepositoryAdaptor<'a> {
    async fn product_get_by_id(&self, id: &String) -> Result<Option<Product>, HexagonalError> {
        let result = self.persistance_repository.get_item_primary("product_".to_string() + id, "-".to_string()).await;

        match result {
            Ok(result) => {
                match result.item {
                    Some(item) => Ok(Some(Product::from_attr_map(item))),
                    None => Ok(None),
                }
            },
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to get product".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn product_create(&self, product: &Product) -> Result<Product, HexagonalError> {
        let result = self.persistance_repository.put_new_item(product.into_attr_map()).await;

        match result {
            Ok(output) => Ok(Product::from_attr_map(output.attributes.unwrap())),
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to create product".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn product_update_by_id(&self, id: &String, product_update: &MutableProduct) -> Result<Product, HexagonalError> {

        let mut update_expression = "SET ".to_string();
        let mut attr_names = HashMap::new();

        if product_update.name.is_some() {
            update_expression.push_str("#name_key = :name, ");
            attr_names.insert("#name_key".to_string(), "name".to_string());
        }

        if product_update.price_cents.is_some() {
            update_expression.push_str("#price_cents_key = :price_cents, ");
            attr_names.insert("#price_cents".to_string(), "price_cents".to_string());
        }

        if product_update.description.is_some() {
            update_expression.push_str("#description_key = :description, ");
            attr_names.insert("#description".to_string(), "description".to_string());
        }

        update_expression.push_str("updated_at = :updated_at");

        let result = self.persistance_repository.update_item(
            "product_".to_string() + id, 
            "-".to_string(),
            update_expression,
            Some(attr_names),
            Some(product_update.into_attr_map())
        ).await;

        match result {
            Ok(output) => Ok(Product::from_attr_map(output.attributes.unwrap())),
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to update product".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn product_delete_by_id(&self, id: &String) -> Result<Product, HexagonalError> {
        let result = self.persistance_repository.delete_item("product_".to_string() + id, "-".to_string()).await;

        match result {
            Ok(output) => Ok(Product::from_attr_map(output.attributes.unwrap())),
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to delete product".to_string(),
                trace: e.to_string(),
            }),
        }
    }
}