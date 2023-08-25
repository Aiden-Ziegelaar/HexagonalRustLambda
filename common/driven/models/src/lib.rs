use std::fmt;
use std::sync::OnceLock;

pub static REPOSITORY: OnceLock<dynamo_db_repository::DynamoDBSingleTableRepository> = OnceLock::new();

pub struct ModelRepositoryError {}

impl fmt::Display for ModelRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to perform model repository operation")
    }
}

pub mod models;
