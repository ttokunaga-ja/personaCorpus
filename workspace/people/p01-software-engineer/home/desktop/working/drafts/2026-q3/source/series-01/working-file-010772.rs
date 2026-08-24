// Synthetic ephemeral desktop working copy: p01-src-010772.
struct WorkingSample { issue: &'static str, service: &'static str, latency_ms: u16, state: &'static str }
fn within_alpha_reference(sample: &WorkingSample) -> bool { sample.latency_ms <= 200 }
fn main() {
    let sample = WorkingSample { issue: "ALW-12772", service: "svc-decision-store", latency_ms: 183, state: "draft" };
    println!("{} {} {} {}", sample.issue, sample.service, sample.state, within_alpha_reference(&sample));
}
