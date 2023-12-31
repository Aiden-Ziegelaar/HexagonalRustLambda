locals {
    router_fragment = {
        "/product" = {
            "post" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_create_lambda.lambda_arn}/invocations"
                }
            }
            "get" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_batch_get_lambda.lambda_arn}/invocations"
                }
            }
        }
        "/product/{id}" = {
            "put" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_update_lambda.lambda_arn}/invocations"
                    "requestParameters": {
                        "integration.request.path.id": "method.request.path.id"
                    }
                }
            }
            "delete" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_delete_lambda.lambda_arn}/invocations"
                    "requestParameters": {
                        "integration.request.path.id": "method.request.path.id"
                    }
                }
            }
            "get" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.product_get_lambda.lambda_arn}/invocations"
                    "requestParameters": {
                        "integration.request.path.id": "method.request.path.id"
                    }
                }
            }
        }
    }
}