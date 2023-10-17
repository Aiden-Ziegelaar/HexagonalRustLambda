variable "app_name" {
    type = string
    nullable = false
}

variable "lambda_name" {
    type = string
    nullable = false
}

variable "additional_policy_arns" {
    type = list(string)
    default = []
    nullable = false
}

variable "bootstrap_folder_name" {
    type = string
    nullable = false
}

variable "dynamo_table_name" {
    type = string
    nullable = false
}

variable "env_vars" {
    type = map(string)
    default = {}
    nullable = false
}

variable "architectures" {
    type = list(string)
    nullable = false
}

variable "eventbridge_rule_arn" {
    type = string
    nullable = false
}
