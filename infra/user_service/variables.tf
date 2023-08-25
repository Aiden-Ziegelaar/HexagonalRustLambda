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