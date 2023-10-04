resource "aws_api_gateway_rest_api" "main_api" {

    body = jsonencode({
        openapi = "3.0.1"
        info = {
        title   = "example"
        version = "1.0"
        }
        paths = merge(
            module.user_service.router_fragment,
            module.product_service.router_fragment
        )
    })

    name = "${local.app_name}-api"
}

resource "aws_api_gateway_deployment" "main_deployment" {
    rest_api_id = aws_api_gateway_rest_api.main_api.id

    triggers = {
        router_changes = sha1(aws_api_gateway_rest_api.main_api.body)
    }

    lifecycle {
        create_before_destroy = true
    }
}

resource "aws_api_gateway_stage" "main_stage" {
    deployment_id = aws_api_gateway_deployment.main_deployment.id
    rest_api_id   = aws_api_gateway_rest_api.main_api.id
    stage_name    = "main"
}