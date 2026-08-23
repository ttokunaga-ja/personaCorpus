# Identity remains separately deployable while sharing the checkout failover posture.
resource "terraform_data" "identity_service_contract" {
  input = merge(local.common_tags, {
    service              = local.identity_service_name
    session_mode         = "regional-read-through"
    failover_requirement = "preserve-authentication-availability"
    checkout_dependency  = local.checkout_service_name
  })
}
