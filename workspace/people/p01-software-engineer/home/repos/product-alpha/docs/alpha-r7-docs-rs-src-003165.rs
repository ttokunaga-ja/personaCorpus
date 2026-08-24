/// Synthetic Product Alpha R7 documentation helper for evidence capture.
pub const ARTIFACT_ID: &str = "p01-full-003165";
pub const SERVICE: &str = "svc-event-bus";
pub fn release_summary() -> (&'static str, &'static str, u16, u16) { ("ALPHA-R7", "ADR-042", 184, 200) }
#[cfg(test)] mod tests { use super::*; #[test] fn observed_p95_is_within_slo() { assert!(release_summary().2 <= release_summary().3); } }
