resource "terraform_data" "maintenance_window" {
  input = {
    window_utc       = var.maintenance_window_utc
    change_class     = "standard-sre-maintenance"
    traffic_strategy = "drain-primary-then-validate-secondary"
    abort_condition  = "availability-slo-risk-detected"
  }
}
