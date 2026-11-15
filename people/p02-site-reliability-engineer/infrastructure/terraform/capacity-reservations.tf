resource "terraform_data" "capacity_reservations" {
  input = {
    checkout_units_primary   = var.planned_checkout_capacity
    checkout_units_secondary = ceil(var.planned_checkout_capacity * 0.6)
    identity_units_each      = ceil(var.planned_checkout_capacity * 0.35)
    review_cadence           = "weekly-before-peak-window"
  }

  lifecycle {
    precondition {
      condition     = var.planned_checkout_capacity >= 20
      error_message = "Capacity plan must retain at least 20 synthetic workload units."
    }
  }
}
