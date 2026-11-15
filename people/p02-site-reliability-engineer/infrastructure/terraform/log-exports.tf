resource "terraform_data" "log_export_policy" {
  input = {
    source_services = [local.checkout_service_name, local.identity_service_name]
    destination     = "sre-log-archive.example"
    filter          = "exclude-payment-and-session-content"
    retention       = "14d"
    encryption_note = "configured-by-approved-provider-implementation"
  }
}
