use std::fmt;
use async_once::AsyncOnce;

#[macro_use]
extern crate lazy_static;

lazy_static!{
    static ref REPOSITORY: AsyncOnce<dynamo_db_repository::DynamoDBSingleTableRepository> = AsyncOnce::new(async {
        let now = std::time::SystemTime::now();
        let output = dynamo_db_repository::DynamoDBSingleTableRepository::new().await;
        println!("DynamoDBSingleTableRepository::new() took {:?}", now.elapsed().unwrap().as_millis());
        output
    });
}

pub struct ModelRepositoryError {}

impl fmt::Display for ModelRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to perform model repository operation")
    }
}

pub mod models;
