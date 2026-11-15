/// Synthetic Product Alpha R7 documentation helper for runbook navigation.
pub const ARTIFACT_ID: &str = "p01-full-004032";
pub const SERVICE: &str = "svc-edge-api";
pub fn release_summary() -> (&'static str, &'static str, u16, u16) { ("ALPHA-R7", "ADR-042", 184, 200) }
#[cfg(test)] mod tests { use super::*; #[test] fn observed_p95_is_within_slo() { assert!(release_summary().2 <= release_summary().3); } }
