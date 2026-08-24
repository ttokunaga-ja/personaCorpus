// Ephemeral desktop working copy for p01-src-002094; synthetic only.
#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkingObservation { issue_id: &'static str, service: &'static str, latency_ms: u16, state: &'static str }

fn within_alpha_reference(item: &WorkingObservation) -> bool { item.latency_ms <= 200 }

fn main() {
    let item = WorkingObservation { issue_id: "ALW-3094", service: "svc-edge-api", latency_ms: 186, state: "review-pending" };
    println!("{} {}", item.issue_id, within_alpha_reference(&item));
}
