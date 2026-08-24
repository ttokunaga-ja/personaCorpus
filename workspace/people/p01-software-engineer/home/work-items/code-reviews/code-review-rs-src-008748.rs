/// Synthetic evidence for CR-B4-008748.
pub const REVIEW_ID: &str = "CR-B4-008748";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (33, 31)
}

pub fn review_focus() -> &'static str {
    "decision audit retention"
}
