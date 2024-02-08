// Cart Access Patterns
// 1. Get Cart for user by user_id
// 2. Add Product to Cart user_id, product_id, quantity
// 3. Remove Product from Cart by user_id
// 4. Update quantity of Product in Cart by user_id, product_id, quantity
// 5. Delete Cart by user_id
// 6. Remove product from all carts by product_id

// Model:
// Pkey = CART#USER#<user_id>
// Skey = CART#PRODUCT#<product_id>
// GSI1-Pkey = CART#PRODUCT#<product_id>
// GSI1-Skey = CART#USER#<user_id>

// Assumptions:
// 1. A single cart does not exceed 1MB of data

use std::{collections::HashMap, error::Error};

use crate::{default_time, DynamoDbModel};
use async_trait::async_trait;
use aws_sdk_dynamodb::types::{AttributeValue, WriteRequest};
use error::HexagonalError;
use mockall::automock;
use persistance_repository::DynamoDBSingleTableRepository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub product_id: String,
    pub user_id: String,
    pub quantity: u32,
    #[serde(default = "default_time")]
    pub created_at: String,
    #[serde(default = "default_time")]
    pub updated_at: String,
}

impl CartItem {
    pub fn new(product_id: String, user_id: String, quantity: u32) -> Self {
        Self {
            product_id,
            user_id,
            quantity,
            created_at: default_time(),
            updated_at: default_time(),
        }
    }
}

impl DynamoDbModel for CartItem {
    fn from_attr_map(attr_map: std::collections::HashMap<String, AttributeValue>) -> Self {
        CartItem {
            product_id: attr_map
                .get("product_id")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string(),
            user_id: attr_map.get("user_id").unwrap().as_s().unwrap().to_string(),
            quantity: attr_map
                .get("quantity")
                .unwrap()
                .as_n()
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            created_at: attr_map
                .get("created_at")
                .unwrap()
                .as_n()
                .unwrap()
                .to_string(),
            updated_at: attr_map
                .get("updated_at")
                .unwrap()
                .as_n()
                .unwrap()
                .to_string(),
        }
    }

    fn into_attr_map(&self) -> std::collections::HashMap<String, AttributeValue> {
        let mut attr_map = std::collections::HashMap::new();
        attr_map.insert(
            "Pkey".to_string(),
            AttributeValue::S("CART#USER#".to_string() + &self.user_id.to_string()),
        );
        attr_map.insert(
            "Skey".to_string(),
            AttributeValue::S("CART#PRODUCT#".to_string() + &self.product_id.to_string()),
        );
        attr_map.insert(
            "GSI1Pkey".to_string(),
            AttributeValue::S("CART#PRODUCT#".to_string() + &self.product_id.to_string()),
        );
        attr_map.insert(
            "GSI1Skey".to_string(),
            AttributeValue::S("CART#USER#".to_string() + &self.user_id.to_string()),
        );
        attr_map.insert(
            "product_id".to_string(),
            AttributeValue::S(self.product_id.to_string()),
        );
        attr_map.insert(
            "user_id".to_string(),
            AttributeValue::S(self.user_id.to_string()),
        );
        attr_map.insert(
            "quantity".to_string(),
            AttributeValue::N(self.quantity.to_string()),
        );
        attr_map.insert(
            "created_at".to_string(),
            AttributeValue::N(self.created_at.to_string()),
        );
        attr_map.insert(
            "updated_at".to_string(),
            AttributeValue::N(self.updated_at.to_string()),
        );
        attr_map
    }
}

#[automock]
#[async_trait]
pub trait CartRepositoryPort {
    async fn cart_get_by_user_id(&self, user_id: &String) -> Result<Vec<CartItem>, HexagonalError>;
    async fn cart_add_item(&self, item: &CartItem) -> Result<CartItem, HexagonalError>;
    async fn cart_remove_item(
        &self,
        user_id: &String,
        product_id: &String,
    ) -> Result<CartItem, HexagonalError>;
    async fn cart_update_item(
        &self,
        user_id: &String,
        product_id: &String,
        quantity: u32,
    ) -> Result<CartItem, HexagonalError>;
    async fn cart_clear(&self, user_id: &String) -> Result<Vec<CartItem>, HexagonalError>;
    async fn cart_global_remove_product(
        &self,
        product_id: &String,
    ) -> Result<(), Vec<HexagonalError>>;
}

pub struct CartRepositoryAdaptor<'a> {
    persistance_repository: &'a DynamoDBSingleTableRepository,
}

impl<'a> CartRepositoryAdaptor<'a> {
    pub fn new(
        persistance_repository: &'a DynamoDBSingleTableRepository,
    ) -> CartRepositoryAdaptor<'a> {
        CartRepositoryAdaptor {
            persistance_repository,
        }
    }
}

#[async_trait]
impl<'a> CartRepositoryPort for CartRepositoryAdaptor<'a> {
    async fn cart_get_by_user_id(&self, user_id: &String) -> Result<Vec<CartItem>, HexagonalError> {
        let query_expression = "Pkey = :pk AND begins_with(Skey, :sk)";

        let mut expression_attribute_values = std::collections::HashMap::new();
        expression_attribute_values.insert(
            ":pk".to_string(),
            AttributeValue::S("CART#USER#".to_string() + &user_id.to_string()),
        );
        expression_attribute_values.insert(
            ":sk".to_string(),
            AttributeValue::S("CART#PRODUCT#".to_string()),
        );

        let query_cart_items = self
            .persistance_repository
            .client
            .query()
            .table_name(self.persistance_repository.table_name.clone())
            .key_condition_expression(query_expression.to_string())
            .set_expression_attribute_values(Some(expression_attribute_values))
            .send()
            .await; // we're making the assumption here that a single cart does not excede 1MB of data

        match query_cart_items {
            Ok(result) => {
                let items = result.items.unwrap();
                let mut cart_items = Vec::new();
                for item in items {
                    cart_items.push(CartItem::from_attr_map(item));
                }
                Ok(cart_items)
            }
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to get cart items".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn cart_add_item(&self, item: &CartItem) -> Result<CartItem, HexagonalError> {
        let result = self
            .persistance_repository
            .put_new_item(item.into_attr_map())
            .await;

        match result {
            Ok(_) => Ok(item.clone()),
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to add item to cart".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn cart_remove_item(
        &self,
        user_id: &String,
        product_id: &String,
    ) -> Result<CartItem, HexagonalError> {
        let result = self
            .persistance_repository
            .delete_item(
                "CART#USER#".to_string() + &user_id.to_string(),
                "CART#PRODUCT#".to_string() + &product_id.to_string(),
            )
            .await;

        match result {
            Ok(value) => Ok(CartItem::from_attr_map(value.attributes.unwrap())),
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to remove item from cart".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn cart_update_item(
        &self,
        user_id: &String,
        product_id: &String,
        quantity: u32,
    ) -> Result<CartItem, HexagonalError> {
        let mut update_expression = "SET ".to_string();
        let mut attr_names = HashMap::new();

        update_expression.push_str("#quantity_key = :quantity, ");
        attr_names.insert("#quantity_key".to_string(), "quantity".to_string());

        update_expression.push_str("updated_at = :updated_at");

        let result = self
            .persistance_repository
            .update_item(
                "CART#USER#".to_string() + &user_id.to_string(),
                "CART#PRODUCT#".to_string() + &product_id.to_string(),
                update_expression,
                Some(attr_names),
                Some(
                    [
                        (
                            ":quantity".to_string(),
                            AttributeValue::N(quantity.to_string()),
                        ),
                        (":updated_at".to_string(), AttributeValue::N(default_time())),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                ),
            )
            .await;

        match result {
            Ok(value) => Ok(CartItem::from_attr_map(value.attributes.unwrap())),
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to update item in cart".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn cart_clear(&self, user_id: &String) -> Result<Vec<CartItem>, HexagonalError> {
        let query_expression = "Pkey = :pk AND begins_with(Skey, :sk)";

        let mut expression_attribute_values = std::collections::HashMap::new();
        expression_attribute_values.insert(
            ":pk".to_string(),
            AttributeValue::S("CART#USER#".to_string() + &user_id.to_string()),
        );
        expression_attribute_values.insert(
            ":sk".to_string(),
            AttributeValue::S("CART#PRODUCT#".to_string()),
        );

        let query_cart_items = self
            .persistance_repository
            .client
            .query()
            .table_name(self.persistance_repository.table_name.clone())
            .key_condition_expression(query_expression.to_string())
            .set_expression_attribute_values(Some(expression_attribute_values))
            .send()
            .await;

        match query_cart_items {
            Ok(result) => {
                let items = result.items.unwrap();
                let mut cart_items = Vec::new();
                for item in items {
                    cart_items.push(CartItem::from_attr_map(item));
                }

                let delete_requests: Vec<WriteRequest> = cart_items
                    .iter()
                    .map(|item| {
                        WriteRequest::builder()
                            .delete_request(
                                aws_sdk_dynamodb::types::DeleteRequest::builder()
                                    .key(
                                        "Pkey".to_string(),
                                        AttributeValue::S(
                                            "CART#USER#".to_string() + &user_id.to_string(),
                                        ),
                                    )
                                    .key(
                                        "Skey".to_string(),
                                        AttributeValue::S(
                                            "CART#PRODUCT#".to_string()
                                                + &item.product_id.to_string(),
                                        ),
                                    )
                                    .build(),
                            )
                            .build()
                    })
                    .collect();

                let delete_chunk_iterator = delete_requests.chunks(25); // need to chunk due to item limit of 25

                for delete_chunk in delete_chunk_iterator {
                    let result = self
                        .persistance_repository
                        .client
                        .batch_write_item()
                        .request_items(
                            self.persistance_repository.table_name.clone(),
                            delete_chunk.to_vec(),
                        )
                        .send()
                        .await;
                    if result.is_err() {
                        return Err(HexagonalError {
                            error: error::HexagonalErrorCode::AdaptorError,
                            message: "Unable to clear cart".to_string(),
                            trace: result.unwrap_err().to_string(),
                        });
                    }
                }

                Ok(cart_items)
            }
            Err(e) => Err(HexagonalError {
                error: error::HexagonalErrorCode::AdaptorError,
                message: "Unable to get cart items".to_string(),
                trace: e.to_string(),
            }),
        }
    }

    async fn cart_global_remove_product(
        &self,
        product_id: &String,
    ) -> Result<(), Vec<HexagonalError>> {
        println!("Removing product {} from all carts", product_id);
        let query_expression = "GSI1Pkey = :pk AND begins_with(GSI1Skey, :sk)";

        let mut errors = Vec::new();

        let mut expression_attribute_values = std::collections::HashMap::new();
        expression_attribute_values.insert(
            ":pk".to_string(),
            AttributeValue::S("CART#PRODUCT#".to_string() + &product_id.to_string()),
        );
        expression_attribute_values.insert(
            ":sk".to_string(),
            AttributeValue::S("CART#USER#".to_string()),
        );

        let mut pagination = true;

        let mut exclusive_start_key: Option<HashMap<String, AttributeValue>> = None;

        while pagination {
            let query_cart_items = self
                .persistance_repository
                .client
                .query()
                .table_name(self.persistance_repository.table_name.clone())
                .index_name("GSI1".to_string())
                .key_condition_expression(query_expression.to_string())
                .set_expression_attribute_values(Some(expression_attribute_values.clone()))
                .set_exclusive_start_key(exclusive_start_key.clone())
                .send()
                .await; // we're making the assumption here that a single cart does not excede 1MB of data

            println!("Query result: {:?}", query_cart_items);

            let delete_result = match query_cart_items {
                Ok(result) => {
                    exclusive_start_key = result.last_evaluated_key;

                    if exclusive_start_key.is_none() {
                        pagination = false;
                    }
                    let items = result.items.unwrap();
                    let mut cart_items = Vec::new();
                    for item in items {
                        cart_items.push(CartItem::from_attr_map(item));
                    }

                    let delete_requests: Vec<WriteRequest> = cart_items
                        .iter()
                        .map(|item| {
                            WriteRequest::builder()
                                .delete_request(
                                    aws_sdk_dynamodb::types::DeleteRequest::builder()
                                        .key(
                                            "Pkey".to_string(),
                                            AttributeValue::S(
                                                "CART#USER#".to_string()
                                                    + &item.user_id.to_string(),
                                            ),
                                        )
                                        .key(
                                            "Skey".to_string(),
                                            AttributeValue::S(
                                                "CART#PRODUCT#".to_string()
                                                    + &item.product_id.to_string(),
                                            ),
                                        )
                                        .build(),
                                )
                                .build()
                        })
                        .collect();

                    let delete_chunk_iterator = delete_requests.chunks(25); // need to chunk due to item limit of 25

                    for delete_chunk in delete_chunk_iterator {
                        let result = self
                            .persistance_repository
                            .client
                            .batch_write_item()
                            .request_items(
                                self.persistance_repository.table_name.clone(),
                                delete_chunk.to_vec(),
                            )
                            .send()
                            .await;

                        if result.is_err() {
                            errors.push(HexagonalError {
                                error: error::HexagonalErrorCode::AdaptorError,
                                message: "Unable to remove product from carts".to_string(),
                                trace: result.unwrap_err().to_string(),
                            });
                        }
                    }

                    Ok(())
                }
                Err(e) => {
                    pagination = false; // stop pagination if failure
                    println!("Error: {:?}", e.source());
                    Err(HexagonalError {
                        error: error::HexagonalErrorCode::AdaptorError,
                        message: "Unable to get cart items".to_string(),
                        trace: "".to_string(),
                    })
                }
            };

            if delete_result.is_err() {
                errors.push(delete_result.unwrap_err())
            }
        }
        if errors.len() > 0 {
            return Err(errors);
        }
        return Ok(());
    }
}
