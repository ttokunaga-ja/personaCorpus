/// Synthetic evidence for CR-B4-006507.
pub const REVIEW_ID: &str = "CR-B4-006507";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (25, 23)
}

pub fn review_focus() -> &'static str {
    "contract version visibility"
}
