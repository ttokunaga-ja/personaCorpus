/// Synthetic code-review evidence for CR-B4-001639.
pub const REVIEW_ID: &str = "CR-B4-001639";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewEvidence {
    pub service: &'static str,
    pub focus: &'static str,
    pub checks_executed: u16,
    pub checks_passed: u16,
}

pub fn evidence() -> ReviewEvidence {
    ReviewEvidence { service: "svc-observability", focus: "rollback observability", checks_executed: 39, checks_passed: 38 }
}
