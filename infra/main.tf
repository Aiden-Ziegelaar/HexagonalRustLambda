resource "aws_dynamodb_table" "basic-dynamodb-table" {
    name           = "Storefront"
    billing_mode   = "PAY_PER_REQUEST"
    hash_key       = "Pkey"
    range_key      = "SKey"

    attribute {
        name = "Pkey"
        type = "S"
    }

    attribute {
        name = "Skey"
        type = "S"
    }

    attribute {
        name = "GSI1Pkey"
        type = "S"
    }

    attribute {
        name = "GSI1Skey"
        type = "S"
    }

    attribute {
        name = "GSI2Pkey"
        type = "S"
    }

    attribute {
        name = "GSI2Skey"
        type = "S"
    }

    ttl {
        attribute_name = "TimeToExist"
        enabled        = false
    }

    global_secondary_index {
        name               = "GSI1"
        hash_key           = "GSI1Pkey"
        range_key          = "GSI1Skey"
        projection_type    = "ALL"
    }

    global_secondary_index {
        name               = "GSI2"
        hash_key           = "GSI2Pkey"
        range_key          = "GSI2key"
        projection_type    = "ALL"
    }

    tags = {
        Name        = "dynamodb-table-1"
        Environment = "production"
    }
}