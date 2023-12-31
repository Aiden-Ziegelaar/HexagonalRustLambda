module "product_get_lambda" {
    source = "../lambda_http_common"
    app_name = var.app_name
    lambda_name = "ProductGetLambda"
    additional_policy_arns = [var.dynamo_policy_arn, var.event_bus_policy_arn]
    bootstrap_folder_name = "product_get"
    dynamo_table_name = var.dynamo_table_name
    architectures = var.architectures
    api_gateway_execution_arn = var.api_gateway_execution_arn
    env_vars = {
        "EVENT_BUS_NAME" = var.event_bus_arn
    }
}