locals {
    router_fragment = {
        "/user" = {
            "post" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.user_create_lambda.lambda_arn}/invocations"
                }
            }
            "put" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.user_update_lambda.lambda_arn}/invocations"
                }
            }
        }
        "/user/hello_world" = {
            "get" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.hello_world_lambda.lambda_arn}/invocations"
                }
            }
        }
    }
}