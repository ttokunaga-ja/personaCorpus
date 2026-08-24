/// Synthetic evidence for CR-A7-009430.
pub const REVIEW_ID: &str = "CR-A7-009430";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (19, 17)
}

pub fn review_focus() -> &'static str {
    "review-thread ownership"
}
