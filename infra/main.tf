module "user_service" {
    source = "./user_service"
    app_name = local.app_name
    dynamo_table_name = aws_dynamodb_table.dynamodb_single_table.name
    dynamo_policy_arn = aws_iam_policy.dynamodb_single_table_access_policy.arn
    architectures = var.architectures
    api_gateway_id = aws_api_gateway_rest_api.main_api.id
    api_gateway_execution_arn = "${aws_api_gateway_rest_api.main_api.execution_arn}/*"
    event_bus_arn = aws_cloudwatch_event_bus.core_event_bus.arn
    event_bus_policy_arn = aws_iam_policy.event_bus_policy.arn
}