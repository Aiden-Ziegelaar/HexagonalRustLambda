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
    pub name: String,
    pub price_cents: i32,
    pub description: String,
}

impl MutableProduct {
    pub fn into_attr_map(&self) -> std::collections::HashMap<String, AttributeValue> {
        let mut attr_map = std::collections::HashMap::new();
        attr_map.insert("name".to_string(), AttributeValue::S(self.name.to_string()));
        attr_map.insert("price_cents".to_string(), AttributeValue::N(self.price_cents.to_string()));
        attr_map.insert("description".to_string(), AttributeValue::S(self.description.to_string()));
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