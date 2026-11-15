#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchitectureReview {
    pub review_id: &'static str,
    pub subject: &'static str,
    pub design_budget_ms: u16,
}

pub fn review_000903() -> ArchitectureReview {
    ArchitectureReview {
        review_id: "ADR-B4-108",
        subject: "replay safety",
        design_budget_ms: 225,
    }
}

pub fn within_budget(elapsed_ms: u16) -> bool {
    elapsed_ms <= review_000903().design_budget_ms
}
