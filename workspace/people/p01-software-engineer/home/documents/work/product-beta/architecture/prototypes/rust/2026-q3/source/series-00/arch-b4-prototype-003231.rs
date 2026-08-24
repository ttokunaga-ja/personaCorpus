#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BetaReview {
    pub id: &'static str,
    pub component: &'static str,
    pub budget_ms: u16,
}

pub fn beta_review_003231() -> BetaReview {
    BetaReview { id: "ADR-B4-048", component: "event distributor", budget_ms: 225 }
}

pub fn accepts_elapsed(elapsed_ms: u16) -> bool {
    elapsed_ms <= beta_review_003231().budget_ms
}
