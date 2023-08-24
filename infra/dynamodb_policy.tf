resource "aws_iam_policy" "dynamodb_single_table_access_policy" {
    name        = "${local.app_name}-DynamoDBSingleTableAccessPolicy"
    description = "DynamoDB Single Table Access Policy for app ${local.app_name}"
    policy      = data.aws_iam_policy_document.dynamodb_single_table_access_policy_document.json
}

data "aws_iam_policy_document" "dynamodb_single_table_access_policy_document" {
    statement {
        sid = "AllowDynamoDBMetaOperations"

        effect = "Allow"

        actions = [
            "dynamodb:List*",
            "dynamodb:DescribeReservedCapacity*",
            "dynamodb:DescribeLimits",
            "dynamodb:DescribeTimeToLive"
        ]

        resources = [
            "*",
        ]
    }

    statement {
        sid = "AllowDynamoDBTableOperations"

        effect = "Allow"

        actions = [
            "dynamodb:BatchGet*",
            "dynamodb:DescribeStream",
            "dynamodb:DescribeTable",
            "dynamodb:Get*",
            "dynamodb:Query",
            "dynamodb:Scan",
            "dynamodb:BatchWrite*",
            "dynamodb:CreateTable",
            "dynamodb:Delete*",
            "dynamodb:Update*",
            "dynamodb:PutItem"      
        ]

        resources = [
            aws_dynamodb_table.dynamodb_single_table.arn,
        ]
    }
}