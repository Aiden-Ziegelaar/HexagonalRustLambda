resource "aws_cloudwatch_event_bus" "core_event_bus" {
  name = "${local.app_name}_core-event-bus"
}