// Aster Loop Product Alpha R7 architecture model: p01-src-004399
struct Review { record_id: &'static str, service: &'static str, observed_p95_ms: u16 }
fn within_slo(review: &Review) -> bool { review.observed_p95_ms <= 200 }
fn main() { let review = Review { record_id: "ARCH-A7-004399", service: "svc-release-coordinator", observed_p95_ms: 195 }; println!("{} {} {}", review.record_id, review.service, within_slo(&review)); }
