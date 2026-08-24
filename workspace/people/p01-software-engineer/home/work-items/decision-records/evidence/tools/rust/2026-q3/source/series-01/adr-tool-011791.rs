//! Synthetic engineering decision evaluator for p01-src-011791.
#[derive(Debug)]
struct LifecycleDecision { record_id: &'static str, adr_id: &'static str, state: &'static str }

const DECISION: LifecycleDecision = LifecycleDecision { record_id: "p01-src-011791", adr_id: "ADR-051", state: "reviewed" };

fn can_publish(decision: &LifecycleDecision, observed_p95_ms: u16) -> bool {
    matches!(decision.state, "reviewed" | "approved" | "released" | "closed") && observed_p95_ms <= 200
}

fn main() {
    println!("{} {} {}", DECISION.record_id, DECISION.adr_id, can_publish(&DECISION, 184));
}
