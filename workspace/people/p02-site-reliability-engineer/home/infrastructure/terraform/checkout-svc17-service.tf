# Desired service contract. Workload deployment is managed by the Kubernetes scope.
resource "terraform_data" "checkout_svc17_service" {
  input = merge(local.common_tags, {
    service          = local.checkout_service_name
    tier             = "critical-path"
    primary_region   = var.primary_region
    secondary_region = var.secondary_region
    dependency       = local.identity_service_name
  })
}
