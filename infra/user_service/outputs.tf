output "router_hash" {
    value = filesha256("${path.module}/router.tf")
}

output "router_fragment" {
    value = local.router_fragment
}