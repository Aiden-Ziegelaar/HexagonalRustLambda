module "user_service" {
    source = "./user_service"
    app_name = local.app_name
    dynamo_table_name = aws_dynamodb_table.dynamodb_single_table.name
    dynamo_policy_arn = aws_iam_policy.dynamodb_single_table_access_policy.arn
}