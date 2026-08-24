//! Synthetic incoming material for ALPHA-R7; retained as unreviewed inbox evidence.

pub const SOURCE_ID: &str = "p01-src-006293";
pub const EVENT: &str = "INBOX-006293";

pub struct InboxMaterial {
    pub status: &'static str,
    pub observed_p95_ms: u16,
    pub accepted: bool,
}

pub fn received_material() -> InboxMaterial {
    InboxMaterial { status: "unreviewed-inbox", observed_p95_ms: 187, accepted: false }
}
