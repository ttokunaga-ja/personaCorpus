/// Synthetic evidence for CR-A7-004627.
pub const REVIEW_ID: &str = "CR-A7-004627";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (30, 28)
}

pub fn review_focus() -> &'static str {
    "contract version visibility"
}
