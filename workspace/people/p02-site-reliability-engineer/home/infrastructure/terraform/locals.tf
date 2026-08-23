locals {
  organization          = "Northstar Transit Cloud"
  checkout_service_name = "checkout-svc17"
  identity_service_name = "identity-gateway"
  monitoring_profile    = "checkout-svc17-standard"
  synthetic_zone_suffix = ".example"
  common_tags = {
    managed_by  = "terraform"
    owner       = "site-reliability-engineering"
    environment = var.environment
    data_class  = "synthetic-operational-metadata"
  }
}
