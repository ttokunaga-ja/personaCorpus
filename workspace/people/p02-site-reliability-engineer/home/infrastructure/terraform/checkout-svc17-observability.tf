# Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13
# SRE checkpoint: error budget 2.4%; incident reference INC-260713.
# This module models desired operational metadata only; it creates no cloud infrastructure.
resource "terraform_data" "checkout_svc17_observability" {
  input = {
    service            = local.checkout_service_name
    monitoring_profile = local.monitoring_profile
    slo_target         = var.checkout_availability_slo
    error_budget       = "2.4%"
    incident_reference = "INC-260713"
  }

  lifecycle {
    precondition {
      condition     = var.checkout_availability_slo >= 0.95 && var.checkout_availability_slo < 1
      error_message = "Checkout availability SLO must be a decimal between 0.95 and 1.00."
    }
  }
}
