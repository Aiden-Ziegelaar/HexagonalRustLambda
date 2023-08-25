resource "aws_iam_role" "lambda_role" {
    name               = "${var.app_name}-${var.lambda_name}"
    assume_role_policy = data.aws_iam_policy_document.lambda_assume_role.json
}

resource "aws_iam_role_policy_attachment" "lambda_role_policy_attachment" {
    count       = length(var.additional_policy_arns)
    role        = aws_iam_role.lambda_role.name
    policy_arn  = var.additional_policy_arns[count.index]
}

resource "aws_iam_role_policy_attachment" "basic_execution_role_policy_attachment" {
    role        = aws_iam_role.lambda_role.name
    policy_arn  = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/../../target/lambda/${var.bootstrap_folder_name}/bootstrap"
  output_path = "${path.module}/../../target/archive/${var.bootstrap_folder_name}.zip"
}

resource "aws_lambda_function" "lambda" {
  filename      = data.archive_file.lambda_archive.output_path
  function_name = "${var.app_name}-${var.lambda_name}"
  role          = aws_iam_role.lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.lambda_archive.output_base64sha256

  runtime = "provided.al2"

  architectures = var.architectures

  //memory_size = 1024

  environment {
    variables = merge(
      var.env_vars,
      {
        DYNAMO_TABLE_NAME = var.dynamo_table_name
      }
    )
  }
}
