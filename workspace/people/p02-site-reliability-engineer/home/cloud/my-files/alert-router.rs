//! Deterministic alert classification for exported alert records.
//! This tool emits a routing decision only; it sends no notifications.

#[derive(Debug, PartialEq, Eq)]
enum Severity { Critical, Warning, Info }

#[derive(Debug, PartialEq, Eq)]
struct Alert<'a> { service: &'a str, signal: &'a str, severity: Severity }

fn route(alert: &Alert<'_>) -> &'static str {
    match (&alert.severity, alert.service, alert.signal) {
        (Severity::Critical, "Checkout SVC-17", "availability") => "checkout-primary-oncall",
        (Severity::Critical, _, _) => "platform-incident-commander",
        (Severity::Warning, "Checkout SVC-17", _) => "checkout-secondary-oncall",
        (Severity::Warning, _, _) => "service-owner-review",
        (Severity::Info, _, _) => "observability-digest",
    }
}

fn main() {
    let alert = Alert { service: "Checkout SVC-17", signal: "availability", severity: Severity::Critical };
    println!("Northstar Transit Cloud route: {}", route(&alert));
    let _example_warning = Severity::Warning;
    let _example_info = Severity::Info;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkout_availability_is_prioritized() {
        let alert = Alert { service: "Checkout SVC-17", signal: "availability", severity: Severity::Critical };
        assert_eq!(route(&alert), "checkout-primary-oncall");
    }
}
