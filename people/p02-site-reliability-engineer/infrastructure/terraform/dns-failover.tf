# .example records document intent and cannot resolve on the public Internet.
resource "terraform_data" "dns_failover_policy" {
  input = {
    record_name       = "checkout.failover.example"
    health_check_path = "/synthetic/ready"
    primary_target    = "checkout-primary-lb.example"
    secondary_target  = "checkout-secondary-lb.example"
    failover_trigger  = "three-consecutive-unhealthy-checks"
  }
}
