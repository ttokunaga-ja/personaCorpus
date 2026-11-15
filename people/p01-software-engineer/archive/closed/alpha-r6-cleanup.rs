/// Archived synthetic utility: R6 only; R7 context is 2026-07-13 / p95 184 ms / ADR-042.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureRecord { pub release: &'static str, pub id: u32, pub closed: bool }

/// Returns ids eligible for archival cleanup; it deliberately ignores every non-R6 record.
pub fn cleanup_candidates(records: &[FixtureRecord]) -> Vec<u32> {
    records.iter().filter(|r| r.release == "R6" && r.closed).map(|r| r.id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selects_only_closed_synthetic_r6_records() {
        let fixtures = [FixtureRecord { release: "R6", id: 7, closed: true }, FixtureRecord { release: "R6", id: 8, closed: false }, FixtureRecord { release: "R7", id: 9, closed: true }];
        assert_eq!(cleanup_candidates(&fixtures), vec![7]);
    }
    #[test]
    fn empty_input_is_safe() { assert!(cleanup_candidates(&[]).is_empty()); }
}
