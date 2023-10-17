module "cart_update_item_lambda" {
    source = "../lambda_http_common"
    app_name = var.app_name
    lambda_name = "CartUpdateItemLambda"
    additional_policy_arns = [var.dynamo_policy_arn, var.event_bus_policy_arn]
    bootstrap_folder_name = "cart_update_item"
    dynamo_table_name = var.dynamo_table_name
    architectures = var.architectures
    api_gateway_execution_arn = var.api_gateway_execution_arn
    env_vars = {
        "EVENT_BUS_NAME" = var.event_bus_arn
    }
}