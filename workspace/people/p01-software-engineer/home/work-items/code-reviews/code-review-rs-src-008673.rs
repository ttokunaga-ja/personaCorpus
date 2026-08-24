/// Synthetic evidence for CR-B4-008673.
pub const REVIEW_ID: &str = "CR-B4-008673";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (16, 14)
}

pub fn review_focus() -> &'static str {
    "decision audit retention"
}
