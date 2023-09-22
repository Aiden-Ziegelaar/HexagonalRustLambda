module "user_service" {
    source = "./user_service"
    app_name = local.app_name
    dynamo_table_name = aws_dynamodb_table.dynamodb_single_table.name
    dynamo_policy_arn = aws_iam_policy.dynamodb_single_table_access_policy.arn
    architectures = var.architectures
    api_gateway_id = aws_api_gateway_rest_api.main_api.id
    api_gateway_execution_arn = "${aws_api_gateway_rest_api.main_api.execution_arn}/*"
}