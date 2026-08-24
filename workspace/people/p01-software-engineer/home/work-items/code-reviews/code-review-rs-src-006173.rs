/// Synthetic evidence for CR-A7-006173.
pub const REVIEW_ID: &str = "CR-A7-006173";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (39, 37)
}

pub fn review_focus() -> &'static str {
    "decision audit retention"
}
