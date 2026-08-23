resource "terraform_data" "incident_tags" {
  input = {
    incident_reference = "INC-260713"
    service            = local.checkout_service_name
    severity           = "synthetic-sev2"
    review_status      = "follow-up-tracked"
    annotations = merge(local.common_tags, {
      event_date = "2026-07-13"
      domain     = "checkout-observability"
    })
  }
}
