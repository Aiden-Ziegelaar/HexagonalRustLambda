use std::collections::HashMap;

use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::operation::update_item::UpdateItemError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use sdk_credential_meta_repository::SdkCredentialsMetaRepository;
use tokio::sync::OnceCell;

pub static AWS_DYNAMO_DB_REPOSITORY: OnceCell<DynamoDBSingleTableRepository> =
    OnceCell::const_new();

pub enum GSIs {
    GSI1,
    GSI2,
}

pub struct DynamoDBSingleTableRepository {
    pub client: Client,
    pub table_name: String,
}

impl DynamoDBSingleTableRepository {
    pub fn new(
        sdk_credential_meta_repository: &SdkCredentialsMetaRepository,
    ) -> DynamoDBSingleTableRepository {
        let table_name = std::env::var("DYNAMO_TABLE_NAME")
            .expect("DYNAMO_TABLE_NAME environment variable not set");
        DynamoDBSingleTableRepository {
            client: Client::new(&sdk_credential_meta_repository.sdk_config),
            table_name,
        }
    }

    pub async fn get_item_primary(
        &self,
        p_key: String,
        s_key: String,
    ) -> Result<aws_sdk_dynamodb::operation::get_item::GetItemOutput, GetItemError> {
        let p_key_att = AttributeValue::S(p_key);
        let s_key_att = AttributeValue::S(s_key);
        self.client
            .get_item()
            .table_name(self.table_name.clone())
            .key("Pkey", p_key_att)
            .key("Skey", s_key_att)
            .send()
            .await
            .map_err(|e| e.into_service_error())
    }

    pub async fn get_item_index(
        &self,
        p_key: String,
        s_key: String,
        index: GSIs,
    ) -> Result<aws_sdk_dynamodb::operation::query::QueryOutput, QueryError> {
        let p_key_att = AttributeValue::S(p_key);
        let s_key_att = AttributeValue::S(s_key);
        self.client
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
            .expression_attribute_values("Pkey", p_key_att)
            .expression_attribute_values("Skey", s_key_att)
            .send()
            .await
            .map_err(|e| e.into_service_error())
    }

    pub async fn delete_item(
        &self,
        p_key: String,
        s_key: String,
    ) -> Result<aws_sdk_dynamodb::operation::delete_item::DeleteItemOutput, DeleteItemError> {
        let p_key_att = AttributeValue::S(p_key);
        let s_key_att = AttributeValue::S(s_key);
        self.client
            .delete_item()
            .table_name(self.table_name.clone())
            .key("Pkey", p_key_att)
            .key("Skey", s_key_att)
            .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
            .send()
            .await
            .map_err(|e| e.into_service_error())
    }

    pub async fn put_new_item(
        &self,
        payload: HashMap<String, AttributeValue>,
    ) -> Result<aws_sdk_dynamodb::operation::put_item::PutItemOutput, PutItemError> {
        self.client
            .put_item()
            .table_name(self.table_name.clone())
            .set_item(Option::Some(payload))
            .condition_expression("attribute_not_exists(Pkey) AND attribute_not_exists(Skey)")
            .send()
            .await
            .map_err(|e| e.into_service_error())
    }

    pub async fn update_item(
        &self,
        p_key: String,
        s_key: String,
        update_expression: String,
        attr_names: Option<HashMap<String, String>>,
        attr_values: Option<HashMap<String, AttributeValue>>,
    ) -> Result<aws_sdk_dynamodb::operation::update_item::UpdateItemOutput, UpdateItemError> {
        let p_key_att = AttributeValue::S(p_key);
        let s_key_att = AttributeValue::S(s_key);
        self.client
            .update_item()
            .table_name(self.table_name.clone())
            .key("Pkey", p_key_att)
            .key("Skey", s_key_att)
            .update_expression(update_expression)
            .condition_expression("attribute_exists(Pkey) AND attribute_exists(Skey)")
            .set_expression_attribute_names(attr_names)
            .set_expression_attribute_values(attr_values)
            .return_values(aws_sdk_dynamodb::types::ReturnValue::AllNew)
            .send()
            .await
            .map_err(|e| {
                println!("error: {:?}", e);
                e.into_service_error()
            })
    }
}
