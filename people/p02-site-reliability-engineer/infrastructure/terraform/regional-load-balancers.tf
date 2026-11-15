resource "terraform_data" "regional_load_balancers" {
  input = {
    primary = {
      name     = "checkout-primary-lb.example"
      region   = var.primary_region
      strategy = "weighted-active"
    }
    secondary = {
      name     = "checkout-secondary-lb.example"
      region   = var.secondary_region
      strategy = "warm-standby"
    }
  }
}
