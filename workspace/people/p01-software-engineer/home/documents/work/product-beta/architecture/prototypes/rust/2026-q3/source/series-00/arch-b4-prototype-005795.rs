#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BetaReview {
    pub id: &'static str,
    pub component: &'static str,
    pub budget_ms: u16,
}

pub fn beta_review_005795() -> BetaReview {
    BetaReview { id: "ADR-B4-025", component: "review evidence sink", budget_ms: 225 }
}

pub fn accepts_elapsed(elapsed_ms: u16) -> bool {
    elapsed_ms <= beta_review_005795().budget_ms
}
