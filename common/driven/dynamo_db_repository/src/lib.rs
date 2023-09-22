use std::collections::HashMap;
use std::fmt;


use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use sdk_credential_meta_repository::{SdkCredentialsMetaRepository, AWS_CREDENTIAL_REPOSITORY};
use tokio::sync::OnceCell;

pub static AWS_DYNAMO_DB_REPOSITORY: OnceCell<DynamoDBSingleTableRepository> = OnceCell::const_new();

#[derive(Debug, Clone)]
pub struct RepositoryError;

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to perform repository operation")
    }
}

pub enum GSIs {
    GSI1,
    GSI2,
}

pub struct DynamoDBSingleTableRepository {
    pub client: Client,
    pub table_name: String,
}


impl DynamoDBSingleTableRepository {
    pub async fn new() -> DynamoDBSingleTableRepository {
        let table_name =
            std::env::var("DYNAMO_TABLE_NAME").expect("DYNAMO_TABLE_NAME environment variable not set");
        DynamoDBSingleTableRepository {
            client: Client::new(&AWS_CREDENTIAL_REPOSITORY.get_or_init(
                SdkCredentialsMetaRepository::new
            ).await.sdk_config.clone()),
            table_name,
        }
    }

    pub async fn get_item_primary(
        &self,
        p_key: String,
        s_key: String,
    ) -> Result<aws_sdk_dynamodb::operation::get_item::GetItemOutput, RepositoryError> {
        let p_key_att = AttributeValue::S(p_key);
        let s_key_att = AttributeValue::S(s_key);
        let result = self
            .client
            .get_item()
            .table_name(self.table_name.clone())
            .key("p_key", p_key_att)
            .key("s_key", s_key_att)
            .send()
            .await;
        if result.is_err() {
            println!("Error: {:?}", result);
        }
        result.map_err(|_| RepositoryError)
    }

    pub async fn get_item_index(
        &self,
        p_key: String,
        s_key: String,
        index: GSIs,
    ) -> Result<aws_sdk_dynamodb::operation::query::QueryOutput, RepositoryError> {
        let p_key_att = AttributeValue::S(p_key);
        let s_key_att = AttributeValue::S(s_key);
        let result = self
            .client
            .query()
            .table_name(self.table_name.clone())
            .index_name(match index {
                GSIs::GSI1 => "GSI1",
                GSIs::GSI2 => "GSI2",
            })
            .key_condition_expression(match index {
                GSIs::GSI1 => "GSI1Pkey = :p_key AND GSI1Skey = :s_key",
                GSIs::GSI2 => "GSI2Pkey = :p_key AND GSI2Skey = :s_key",
            })
            .expression_attribute_values("p_key", p_key_att)
            .expression_attribute_values("s_key", s_key_att)
            .send()
            .await;
        if result.is_err() {
            println!("Error: {:?}", result);
        }
        result.map_err(|_| RepositoryError)
    }

    pub async fn delete_item(
        &self,
        p_key: String,
        s_key: String,
    ) -> Result<aws_sdk_dynamodb::operation::delete_item::DeleteItemOutput, RepositoryError> {
        let p_key_att = AttributeValue::S(p_key);
        let s_key_att = AttributeValue::S(s_key);
        let result = self
            .client
            .delete_item()
            .table_name(self.table_name.clone())
            .key("p_key", p_key_att)
            .key("s_key", s_key_att)
            .send()
            .await;
        if result.is_err() {
            println!("Error: {:?}", result);
        }
        result.map_err(|_| RepositoryError)
    }

    pub async fn put_new_item(
        &self,
        payload: HashMap<String, AttributeValue>,
    ) -> Result<aws_sdk_dynamodb::operation::put_item::PutItemOutput, RepositoryError> {
        let result = self
            .client
            .put_item()
            .table_name(self.table_name.clone())
            .set_item(Option::Some(payload))
            .condition_expression("attribute_not_exists(Pkey) AND attribute_not_exists(Skey)")
            .send()
            .await;
        if result.is_err() {
            println!("Error: {:?}", result);
        }
        result.map_err(|_| RepositoryError)
    }

    pub async fn update_item(
        &self,
        p_key: String,
        s_key: String,
        update_expression: String,
        attr_names: Option<HashMap<String, String>>,
        attr_values: Option<HashMap<String, AttributeValue>>,
    ) -> Result<aws_sdk_dynamodb::operation::update_item::UpdateItemOutput, RepositoryError> {
        let p_key_att = AttributeValue::S(p_key);
        let s_key_att = AttributeValue::S(s_key);
        let result = self
            .client
            .update_item()
            .table_name(self.table_name.clone())
            .key("p_key", p_key_att)
            .key("s_key", s_key_att)
            .update_expression(update_expression)
            .set_expression_attribute_names(attr_names)
            .set_expression_attribute_values(attr_values)
            .send()
            .await;
        if result.is_err() {
            println!("Error: {:?}", result);
        }
        result.map_err(|_| RepositoryError)
    }
}
