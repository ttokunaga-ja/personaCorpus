/// Synthetic evidence for CR-B4-005427.
pub const REVIEW_ID: &str = "CR-B4-005427";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (18, 16)
}

pub fn review_focus() -> &'static str {
    "contract version visibility"
}
