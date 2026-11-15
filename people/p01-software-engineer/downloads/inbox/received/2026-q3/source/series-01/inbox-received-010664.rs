//! Synthetic incoming material for BETA-R4; retained as unreviewed inbox evidence.

pub const SOURCE_ID: &str = "p01-src-010664";
pub const EVENT: &str = "INBOX-010664";

pub struct InboxMaterial {
    pub status: &'static str,
    pub observed_p95_ms: u16,
    pub accepted: bool,
}

pub fn received_material() -> InboxMaterial {
    InboxMaterial { status: "unreviewed-inbox", observed_p95_ms: 188, accepted: false }
}
