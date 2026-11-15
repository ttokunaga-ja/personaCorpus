resource "terraform_data" "monitoring_workspaces" {
  input = {
    workspace_name = "northstar-sre-production-simulated"
    services       = [local.checkout_service_name, local.identity_service_name]
    retention      = "30d-synthetic-metrics"
    dashboard      = "checkout-svc17-overview.example"
  }
}
