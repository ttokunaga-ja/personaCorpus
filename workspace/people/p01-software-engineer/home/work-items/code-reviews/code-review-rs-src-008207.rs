/// Synthetic evidence for CR-A7-008207.
pub const REVIEW_ID: &str = "CR-A7-008207";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (14, 12)
}

pub fn review_focus() -> &'static str {
    "contract version visibility"
}
