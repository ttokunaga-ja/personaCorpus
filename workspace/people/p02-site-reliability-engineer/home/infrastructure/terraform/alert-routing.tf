# Routes are labels for a future integration; no contact addresses or paging keys are stored here.
resource "terraform_data" "alert_routing" {
  input = {
    service          = local.checkout_service_name
    critical_route   = "sre-primary-synthetic"
    warning_route    = "sre-business-hours-synthetic"
    escalation_after = "15m"
    dedupe_key       = "checkout-svc17-region"
  }
}
