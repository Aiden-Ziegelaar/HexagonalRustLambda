module "user_update_lambda" {
    source = "../lambda_common"
    app_name = var.app_name
    lambda_name = "UserUpdateLambda"
    additional_policy_arns = [var.dynamo_policy_arn]
    bootstrap_folder_name = "user_update"
    dynamo_table_name = var.dynamo_table_name
    architectures = var.architectures
    api_gateway_execution_arn = var.api_gateway_execution_arn
}