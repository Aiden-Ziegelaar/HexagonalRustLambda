variable "dynamo_table_name" {
    type = string
    nullable = false
}

variable "dynamo_policy_arn" {
    type = string
    nullable = false
}

variable "app_name" {
    type = string
    nullable = false
}

variable "architectures" {
    type = list(string)
    nullable = false
}

variable "api_gateway_id" {
    type = string
    nullable = false
}

variable "api_gateway_execution_arn" {
    type = string
    nullable = false
}

variable "event_bus_arn" {
    type = string
    nullable = false
}

variable "event_bus_policy_arn" {
    type = string
    nullable = false
}