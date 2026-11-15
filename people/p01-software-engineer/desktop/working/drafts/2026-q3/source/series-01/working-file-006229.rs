// Synthetic ephemeral desktop working copy: p01-src-006229.
struct WorkingSample { issue: &'static str, service: &'static str, latency_ms: u16, state: &'static str }
fn within_alpha_reference(sample: &WorkingSample) -> bool { sample.latency_ms <= 200 }
fn main() {
    let sample = WorkingSample { issue: "ALW-8229", service: "svc-release-coordinator", latency_ms: 197, state: "working" };
    println!("{} {} {} {}", sample.issue, sample.service, sample.state, within_alpha_reference(&sample));
}
