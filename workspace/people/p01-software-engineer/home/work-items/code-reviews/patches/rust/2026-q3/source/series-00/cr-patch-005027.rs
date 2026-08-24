/// Synthetic evidence for CR-A7-005027.
pub const REVIEW_ID: &str = "CR-A7-005027";
pub const REVIEW_ELAPSED_MINUTES: u16 = 400;

pub fn passed_checks() -> (u16, u16) {
    (24, 22)
}

pub fn review_focus() -> &'static str {
    "contract version visibility"
}
