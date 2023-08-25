module "user_create_lambda" {
    source = "../lambda_common"
    app_name = var.app_name
    lambda_name = "UserCreateLambda"
    additional_policy_arns = [var.dynamo_policy_arn]
    bootstrap_folder_name = "user_create"
    dynamo_table_name = var.dynamo_table_name
    architectures = var.architectures
}