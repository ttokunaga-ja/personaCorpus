// Synthetic Product Alpha R7 architecture model: p01-src-000145
struct ArchitectureObservation { record_id: &'static str, service: &'static str, decision_id: &'static str, p95_ms: u16 }
impl ArchitectureObservation { fn meets_slo(&self) -> bool { self.p95_ms <= 200 } }
fn main() { let item = ArchitectureObservation { record_id: "ARCH-A7-000145", service: "svc-release-coordinator", decision_id: "ADR-146", p95_ms: 188 }; println!("{} {} {}", item.record_id, item.service, item.meets_slo()); }
