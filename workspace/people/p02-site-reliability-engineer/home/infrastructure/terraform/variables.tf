variable "environment" {
  description = "Synthetic deployment environment label."
  type        = string
  default     = "production-simulated"
}

variable "primary_region" {
  description = "Primary synthetic region identifier."
  type        = string
  default     = "northstar-a"
}

variable "secondary_region" {
  description = "Secondary synthetic region identifier."
  type        = string
  default     = "northstar-b"
}

variable "checkout_availability_slo" {
  description = "Monthly availability objective for Checkout SVC-17."
  type        = number
  default     = 0.976
}

variable "planned_checkout_capacity" {
  description = "Reserved synthetic checkout workload units per region."
  type        = number
  default     = 120
}

variable "maintenance_window_utc" {
  description = "Approved recurring maintenance window, expressed in UTC."
  type        = string
  default     = "Sun 02:00-03:00 UTC"
}
