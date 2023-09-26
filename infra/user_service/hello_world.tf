module "hello_world_lambda" {
    source = "../lambda_common"
    app_name = var.app_name
    lambda_name = "UserHelloWorld"
    additional_policy_arns = []
    bootstrap_folder_name = "user_hello_world"
    dynamo_table_name = var.dynamo_table_name
    architectures = var.architectures
    api_gateway_execution_arn = var.api_gateway_execution_arn
}