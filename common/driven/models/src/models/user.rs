use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use aws_sdk_dynamodb::types::error::ConditionalCheckFailedException;
use error::HexagonalError;
use aws_sdk_dynamodb::types::AttributeValue;
use persistance_repository::{DynamoDBSingleTableRepository, AWS_DYNAMO_DB_REPOSITORY};
use serde::{Deserialize, Serialize};

// First we define our model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub first: String,
    pub last: String,
    pub email: String,
    pub username: String,
    #[serde(default = "default_time")]
    pub created_at: String,
    #[serde(default = "default_time")]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MutableUser {
    pub email: String,
    pub first: Option<String>,
    pub last: Option<String>,
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
        item.insert(
            "Pkey".to_string(),
            AttributeValue::S("user_".to_string() + &self.email.clone()),
        );
        item.insert("Skey".to_string(), AttributeValue::S("-".to_string()));
        item.insert(
            "GSI1Pkey".to_string(),
            AttributeValue::S("user_".to_string() + &self.username.clone()),
        );
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
        item.insert(
            "Pkey".to_string(),
            AttributeValue::S("user_username_".to_string() + &self.username.clone()),
        );
        item.insert("Skey".to_string(), AttributeValue::S("-".to_string()));
        item.insert("email".to_string(), AttributeValue::S(self.email.clone()));
        item
    }
}

impl MutableUser {
    pub fn into_attr_map(&self) -> HashMap<String, AttributeValue> {
        let mut attribute_values = HashMap::new();
        if Option::is_some(&self.first.clone()) {
            attribute_values.insert(
                ":first".to_string(),
                AttributeValue::S(self.first.clone().unwrap()),
            );
        }
        if Option::is_some(&self.last.clone()) {
            attribute_values.insert(
                ":last".to_string(),
                AttributeValue::S(self.last.clone().unwrap()),
            );
        }
        attribute_values.insert(":updated_at".to_string(), AttributeValue::S(default_time()));
        attribute_values
    }
}

// Conceptually the call signatures of these functions are our "ports" and the implementations are our "adaptors"
pub async fn user_get_by_email(email: String) -> Result<Option<User>, HexagonalError> {
    let result = AWS_DYNAMO_DB_REPOSITORY
        .get_or_init(DynamoDBSingleTableRepository::new)
        .await
        .get_item_primary(format!("user_{}", email.to_string()), "-".to_string())
        .await;

    match result {
        Ok(x) => {
            match x.item {
                Some(y) => Ok(Some(User::from_attr_map(y))),
                None => Ok(None),
            }
        },
        Err(err) => Err(HexagonalError {
            error: error::HexagonalErrorCode::NotFound,
            message: "Unable to fetch user, error in get call".to_string(),
            trace: err.to_string()
        }),
    }
}

pub async fn user_get_by_username(username: String) -> Result<Vec<User>, HexagonalError> {
    let result = AWS_DYNAMO_DB_REPOSITORY
        .get_or_init(DynamoDBSingleTableRepository::new)
        .await
        .get_item_index(
            username.to_string(),
            "-".to_string(),
            persistance_repository::GSIs::GSI1,
        )
        .await;

    match result {
        Ok(x) => {
            match x.items {
                Some(y) => {
                    let mut users = Vec::new();
                    for user in y {
                        users.push(User::from_attr_map(user));
                    }
                    Ok(users)
                },
                None => Err(HexagonalError {
                    error: error::HexagonalErrorCode::NotFound,
                    message: "Unable to fetch user, does not exist".to_string(),
                    trace: "".to_string()
                })
            }
        },
        Err(err) => Err(HexagonalError {
            error: error::HexagonalErrorCode::NotFound,
            message: "Unable to fetch user, error in get call".to_string(),
            trace: err.to_string()
        }),
    }        
}


pub async fn user_create(user: User) -> Result<User, HexagonalError> {
    let repository = AWS_DYNAMO_DB_REPOSITORY
        .get_or_init(DynamoDBSingleTableRepository::new)
        .await;
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

    repository
        .client
        .transact_write_items()
        .transact_items(user_transact)
        .transact_items(username_transact)
        .send()
        .await
        .map_err(|err| {
            let err_trace = err.to_string();
            if err.into_source().unwrap().downcast_ref::<ConditionalCheckFailedException>().is_some() {
                HexagonalError {
                    error: error::HexagonalErrorCode::Conflict,
                    message: "Unable to create user, user already exists".to_string(),
                    trace:  err_trace
                }
            } else {
                HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "Unable to create user, error in transaction write call".to_string(),
                    trace:  err_trace
                }
            }
        })
        .map(|_| {
            user
        })
}

pub async fn user_update_by_email(user: MutableUser) -> Result<User, HexagonalError> {

    let mut update_expression = "SET ".to_string();
    let mut attr_names = HashMap::new();

    match user.first {
        Some(_) => {
            update_expression.push_str("#first_key = :first, ");
            attr_names.insert("#first_key".to_string(), "first".to_string());
        }
        None => {}
    }

    match user.last {
        Some(_) => {
            update_expression.push_str("#last_key = :last, ");
            attr_names.insert("#last_key".to_string(), "last".to_string());
        }
        None => {}
    }

    update_expression.push_str("updated_at = :updated_at");

    let result = AWS_DYNAMO_DB_REPOSITORY
        .get_or_init(DynamoDBSingleTableRepository::new)
        .await
        .update_item(
            format!("user_{}", user.email.to_string()),
            "-".to_string(),
            update_expression.to_string(),
            Some(attr_names),
            Some(user.into_attr_map()),
        )
        .await;

    match result {
        Ok(x) => {
            match x.attributes {
                Some(y) => Ok(User::from_attr_map(y)),
                None => Err(HexagonalError {
                    error: error::HexagonalErrorCode::Unkown,
                    message: "User attributes were not returned".to_string(),
                    trace: "".to_string()
                }),
            }
        },
        Err(err) => Err(
            if err.is_conditional_check_failed_exception() {
                HexagonalError{
                    error: error::HexagonalErrorCode::NotFound,
                    message: "Unable to update user, does not exist".to_string(),
                    trace: err.to_string()
                }
            } else {
                HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "Unable to update user, error in put call".to_string(),
                    trace: err.to_string()
                }
        }),
    }
}

pub async fn user_update_username_by_email(
    email: String,
    new_username: String,
) -> Result<(), HexagonalError> {
    let repository = AWS_DYNAMO_DB_REPOSITORY
        .get_or_init(DynamoDBSingleTableRepository::new)
        .await;

    let mut username_item = HashMap::new();
    username_item.insert(
        "Pkey".to_string(),
        AttributeValue::S("user_username_".to_string() + &new_username.clone()),
    );
    username_item.insert("Skey".to_string(), AttributeValue::S("-".to_string()));
    username_item.insert("email".to_string(), AttributeValue::S(email.clone()));
    let pkey_unique = "attribute_not_exists(Pkey) AND attribute_not_exists(Skey)";
    let username_put = aws_sdk_dynamodb::types::Put::builder()
        .table_name(repository.table_name.clone())
        .set_item(Some(username_item))
        .condition_expression(pkey_unique.to_string())
        .build();
    let username_put_action = aws_sdk_dynamodb::types::TransactWriteItem::builder()
        .put(username_put)
        .build();

    let update_user_expression = "SET username = :username, updated_at = :updated_at";
    let mut attribute_values = HashMap::new();
    attribute_values.insert(
        ":username".to_string(),
        AttributeValue::S(new_username.clone()),
    );
    attribute_values.insert(":updated_at".to_string(), AttributeValue::S(default_time()));
    let username_update = aws_sdk_dynamodb::types::Update::builder()
        .table_name(repository.table_name.clone())
        .update_expression(update_user_expression.to_string())
        .key(
            "Pkey",
            AttributeValue::S("user_".to_string() + &email.clone()),
        )
        .key("Skey", AttributeValue::S("-".to_string()))
        .set_expression_attribute_values(Some(attribute_values))
        .build();
    let username_update_action = aws_sdk_dynamodb::types::TransactWriteItem::builder()
        .update(username_update)
        .build();

    let username_delete = aws_sdk_dynamodb::types::Delete::builder()
        .table_name(repository.table_name.clone())
        .key(
            "Pkey",
            AttributeValue::S("user_username_".to_string() + &new_username.clone()),
        )
        .key("Skey", AttributeValue::S("-".to_string()))
        .build();
    let username_delete_action = aws_sdk_dynamodb::types::TransactWriteItem::builder()
        .delete(username_delete)
        .build();

    repository
        .client
        .transact_write_items()
        .transact_items(username_put_action)
        .transact_items(username_update_action)
        .transact_items(username_delete_action)
        .send()
        .await
        .map_err(|err| {
            let err_trace = err.to_string();
            if err.into_source().unwrap().downcast_ref::<ConditionalCheckFailedException>().is_some() {
                HexagonalError {
                    error: error::HexagonalErrorCode::Conflict,
                    message: "Unable to change username, username is already taken".to_string(),
                    trace:  err_trace
                }
            } else {
                HexagonalError {
                    error: error::HexagonalErrorCode::AdaptorError,
                    message: "Unable to update username, error in transaction write call".to_string(),
                    trace:  err_trace
                }
            }
        })
        .map(|_| ())
}

pub async fn user_delete_by_email(email: String) -> Result<User, HexagonalError> {
    let result = AWS_DYNAMO_DB_REPOSITORY
        .get_or_init(DynamoDBSingleTableRepository::new)
        .await
        .delete_item(email.to_string(), "-".to_string())
        .await
        .map_err(|_| HexagonalError {
            error: error::HexagonalErrorCode::AdaptorError,
            message: "Unable to delete user, error in delete call".to_string(),
            trace: "".to_string()
        });
    match result {
        Ok(x) => {
            match x.attributes {
                Some(y) => Ok(User::from_attr_map(y)),
                None => Err(HexagonalError {
                    error: error::HexagonalErrorCode::NotFound,
                    message: "Unable to delete user, does not exist".to_string(),
                    trace: "".to_string()
                }),
            }
        },
        Err(err) => Err(HexagonalError {
            error: error::HexagonalErrorCode::AdaptorError,
            message: "Unable to delete user, error in delete call".to_string(),
            trace: err.to_string()
        }),
    }
}
