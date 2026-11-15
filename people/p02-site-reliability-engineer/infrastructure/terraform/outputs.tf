output "checkout_observability_profile" {
  description = "Synthetic monitoring profile selected for Checkout SVC-17."
  value       = terraform_data.checkout_svc17_observability.output.monitoring_profile
}

output "failover_record_name" {
  description = "Non-routable documentation record for the failover policy."
  value       = terraform_data.dns_failover_policy.output.record_name
}

output "capacity_plan" {
  description = "Synthetic capacity reservation inputs for regional review."
  value       = terraform_data.capacity_reservations.output
}

output "incident_annotation" {
  description = "Incident tag set retained with this operational model."
  value       = terraform_data.incident_tags.output
}
