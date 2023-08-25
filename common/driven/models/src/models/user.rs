use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{ModelRepositoryError, REPOSITORY};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};

// First we define our model

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    first: String,
    last: String,
    email: String,
    username: String,
    #[serde(default = "default_time")]
    created_at: String,
    #[serde(default = "default_time")]
    updated_at: String,
}

fn default_time() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs().to_string()
}

// Adaptor Transform Traits
impl User {
    pub fn into_attr_map(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("Pkey".to_string(), AttributeValue::S("user_".to_string() + &self.email.clone()));
        item.insert("Skey".to_string(), AttributeValue::S("-".to_string()));
        item.insert("GSI1Pkey".to_string(), AttributeValue::S("user_".to_string() + &self.username.clone()));
        item.insert("GSI1Skey".to_string(), AttributeValue::S("-".to_string()));
        item.insert("first".to_string(), AttributeValue::S(self.first.clone()));
        item.insert("last".to_string(), AttributeValue::S(self.last.clone()));
        item.insert("email".to_string(), AttributeValue::S(self.email.clone()));
        item.insert(
            "username".to_string(),
            AttributeValue::S(self.username.clone()),
        );
        item.insert(
            "created_at".to_string(),
            AttributeValue::S(self.created_at.clone()),
        );
        item.insert(
            "updated_at".to_string(),
            AttributeValue::S(self.updated_at.clone()),
        );
        item
    }

    pub fn from_attr_map(attr: HashMap<String, AttributeValue>) -> User {
        User {
            first: attr.get("first").unwrap().as_s().unwrap().to_string(),
            last: attr.get("last").unwrap().as_s().unwrap().to_string(),
            email: attr.get("email").unwrap().as_s().unwrap().to_string(),
            username: attr.get("username").unwrap().as_s().unwrap().to_string(),
            created_at: attr.get("created_at").unwrap().as_s().unwrap().to_string(),
            updated_at: attr.get("updated_at").unwrap().as_s().unwrap().to_string(),
        }
    }
}

// Conceptually the call signatures of these functions are our "ports" and the implementations are our "adaptors"
pub async fn user_get_by_email(email: String) -> Result<User, ModelRepositoryError> {
    let result = REPOSITORY.get().unwrap()
        .get_item_primary(email.to_string(), "-".to_string())
        .await
        .map_err(|_| ModelRepositoryError {})?;
    Ok(User::from_attr_map(result.item.unwrap()))
}

pub async fn user_get_by_username(username: String) -> Result<Vec<User>, ModelRepositoryError> {
    let result = REPOSITORY.get().unwrap()
        .get_item_index(
            username.to_string(),
            "-".to_string(),
            dynamo_db_repository::GSIs::GSI1,
        )
        .await
        .map_err(|_| ModelRepositoryError {})?;
    Ok(result
        .items
        .unwrap()
        .iter()
        .map(|x| User::from_attr_map(x.clone()))
        .collect())
}

pub async fn user_create(user: User) -> Result<(), ModelRepositoryError> {
    REPOSITORY.get().unwrap()
        .put_new_item(user.into_attr_map())
        .await
        .map_err(|_| ModelRepositoryError {}).map(|_| ())
}

pub async fn user_update_by_email(email: String, user: User) -> Result<User, ModelRepositoryError> {
    let update_expression =
        "SET first = :first, last = :last, username = :username, updated_at = :updated_at";
    let mut attribute_values = HashMap::new();
    attribute_values.insert(":first".to_string(), AttributeValue::S(user.first.clone()));
    attribute_values.insert(":last".to_string(), AttributeValue::S(user.last.clone()));
    attribute_values.insert(
        ":username".to_string(),
        AttributeValue::S(user.username.clone()),
    );
    attribute_values.insert(
        ":updated_at".to_string(),
        AttributeValue::S(user.updated_at.clone()),
    );
    REPOSITORY.get().unwrap()
        .update_item(
            email.to_string(),
            "-".to_string(),
            update_expression.to_string(),
            None,
            Some(attribute_values),
        )
        .await
        .map_err(|_| ModelRepositoryError {})
        .map(|x| User::from_attr_map(x.attributes.unwrap()))
}

pub async fn user_delete_by_email(email: String) -> Result<User, ModelRepositoryError> {
    let result = REPOSITORY.get().unwrap()
        .delete_item(email.to_string(), "-".to_string())
        .await
        .map_err(|_| ModelRepositoryError {})?;
    Ok(User::from_attr_map(result.attributes.unwrap()))
}
