data "aws_iam_policy_document" "event_bus_policy" {
    statement {
        sid = "AllowAddingEventsToBus"

        effect = "Allow"

        actions = ["events:PutEvents"]
        resources = [
            aws_cloudwatch_event_bus.core_event_bus.arn
        ]
    }
}

resource "aws_iam_policy" "event_bus_policy" {
    name = "${local.app_name}_event_bus_policy"
    policy = data.aws_iam_policy_document.event_bus_policy.json
}