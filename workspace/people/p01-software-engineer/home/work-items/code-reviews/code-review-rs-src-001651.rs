/// Synthetic code-review evidence for CR-B4-001651.
pub const REVIEW_ID: &str = "CR-B4-001651";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewEvidence {
    pub service: &'static str,
    pub focus: &'static str,
    pub checks_executed: u16,
    pub checks_passed: u16,
}

pub fn evidence() -> ReviewEvidence {
    ReviewEvidence { service: "svc-release-coordinator", focus: "idempotent event handling", checks_executed: 20, checks_passed: 19 }
}
