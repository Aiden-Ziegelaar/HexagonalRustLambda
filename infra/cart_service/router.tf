locals {
    router_fragment = {
        "/cart/{username}" = {
            "get" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.cart_get_lambda.lambda_arn}/invocations"
                }
            }
            "delete" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.cart_clear_http_lambda.lambda_arn}/invocations"
                }
            }
        },
        "/cart/{username}/item" = {
            "post" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.cart_add_item_lambda.lambda_arn}/invocations"
                }
            }
        }
        "/cart/{username}/item/{product_id}" = {

            "delete" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.cart_remove_item_lambda.lambda_arn}/invocations"
                }
            }
            "patch" = {
                "x-amazon-apigateway-integration" = {
                    "httpMethod" = "POST"
                    "type" = "aws_proxy"
                    "uri" = "arn:aws:apigateway:ap-southeast-2:lambda:path/2015-03-31/functions/${module.cart_update_item_lambda.lambda_arn}/invocations"
                }
            }
        }
    }
}