/// Synthetic evidence for CR-A7-008630.
pub const REVIEW_ID: &str = "CR-A7-008630";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (31, 29)
}

pub fn review_focus() -> &'static str {
    "review-thread ownership"
}
