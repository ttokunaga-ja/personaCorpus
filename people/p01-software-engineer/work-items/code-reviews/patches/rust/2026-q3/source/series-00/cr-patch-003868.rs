/// Synthetic code-review evidence for CR-A7-003868.
pub const REVIEW_ID: &str = "CR-A7-003868";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewEvidence {
    pub service: &'static str,
    pub focus: &'static str,
    pub checks_executed: u16,
    pub checks_passed: u16,
}

pub fn evidence() -> ReviewEvidence {
    ReviewEvidence { service: "svc-event-bus", focus: "latency budget accounting", checks_executed: 36, checks_passed: 35 }
}
