use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ModelRepositoryError;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use dynamo_db_repository::{DynamoDBSingleTableRepository, AWS_DYNAMO_DB_REPOSITORY};

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

    pub fn into_attr_map_unique_username(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("Pkey".to_string(), AttributeValue::S("user_username_".to_string() + &self.username.clone()));
        item.insert("Skey".to_string(), AttributeValue::S("-".to_string()));
        item.insert("email".to_string(), AttributeValue::S(self.email.clone()));
        item
    }
}

// Conceptually the call signatures of these functions are our "ports" and the implementations are our "adaptors"
pub async fn user_get_by_email(email: String) -> Result<User, ModelRepositoryError> {
    let result = AWS_DYNAMO_DB_REPOSITORY.get_or_init(DynamoDBSingleTableRepository::new).await
        .get_item_primary(email.to_string(), "-".to_string())
        .await
        .map_err(|_| ModelRepositoryError {})?;
    Ok(User::from_attr_map(result.item.unwrap()))
}

pub async fn user_get_by_username(username: String) -> Result<Vec<User>, ModelRepositoryError> {
    let result = AWS_DYNAMO_DB_REPOSITORY.get_or_init(DynamoDBSingleTableRepository::new).await
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
    let repository = AWS_DYNAMO_DB_REPOSITORY.get_or_init(DynamoDBSingleTableRepository::new).await;
    let user_model: HashMap<String, AttributeValue> = user.into_attr_map();
    let username_model: HashMap<String, AttributeValue> = user.into_attr_map_unique_username();

    let pkey_unique = "attribute_not_exists(Pkey) AND attribute_not_exists(Skey)";

    let user_put = aws_sdk_dynamodb::types::Put::builder()
        .table_name(repository.table_name.clone())
        .set_item(Some(user_model))
        .condition_expression(pkey_unique.to_string())
        .build();

    let username_put = aws_sdk_dynamodb::types::Put::builder()
        .table_name(repository.table_name.clone())
        .set_item(Some(username_model))
        .condition_expression(pkey_unique.to_string())
        .build();

    let user_transact = aws_sdk_dynamodb::types::TransactWriteItem::builder()
        .put(user_put)
        .build();

    let username_transact = aws_sdk_dynamodb::types::TransactWriteItem::builder()
        .put(username_put)
        .build();

        repository.client
        .transact_write_items()
        .transact_items(user_transact)
        .transact_items(username_transact)
        .send().await.map_err(|err| {
            println!("Error: {:?}", err);
            ModelRepositoryError {}
        }).map(|_| ())
}

pub async fn username_update_by_email(email: String, user: User) -> Result<User, ModelRepositoryError> {
    let update_expression =
        "SET first = :first, last = :last, updated_at = :updated_at";
    let mut attribute_values = HashMap::new();
    attribute_values.insert(":first".to_string(), AttributeValue::S(user.first.clone()));
    attribute_values.insert(":last".to_string(), AttributeValue::S(user.last.clone()));
    attribute_values.insert(
        ":updated_at".to_string(),
        AttributeValue::S(user.updated_at.clone()),
    );
    AWS_DYNAMO_DB_REPOSITORY.get_or_init(DynamoDBSingleTableRepository::new).await
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
    let result = AWS_DYNAMO_DB_REPOSITORY.get_or_init(DynamoDBSingleTableRepository::new).await
        .delete_item(email.to_string(), "-".to_string())
        .await
        .map_err(|_| ModelRepositoryError {})?;
    Ok(User::from_attr_map(result.attributes.unwrap()))
}
