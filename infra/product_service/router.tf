locals {
    router_fragment = {
        "/product" = {
            "get" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_get_lambda.lambda_arn}/invocations"
                }
            }
            "post" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_create_lambda.lambda_arn}/invocations"
                }
            }
            "put" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_update_lambda.lambda_arn}/invocations"
                }
            }
            "delete" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_delete_lambda.lambda_arn}/invocations"
                }
            }
        }
    }
}