pub mod models;
use std::time::{SystemTime, UNIX_EPOCH};

use aws_sdk_dynamodb::types::AttributeValue;

pub fn default_time() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs().to_string()
}

pub fn new_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

trait DynamoDbModel {
    fn into_attr_map(&self) -> std::collections::HashMap<String, AttributeValue>;

    fn from_attr_map(attr_map: std::collections::HashMap<String, AttributeValue>) -> Self;
}
