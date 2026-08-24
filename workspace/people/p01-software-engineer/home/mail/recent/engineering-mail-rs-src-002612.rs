//! Synthetic attachment referenced by MSG-2026Q3-002612; no network or credentials.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailEvidence {
    pub message_id: &'static str,
    pub project: &'static str,
    pub subject: &'static str,
    pub design_budget_ms: u16,
}

pub fn reviewed_mail_evidence() -> MailEvidence {
    MailEvidence {
        message_id: "MSG-2026Q3-002612",
        project: "BETA-R4",
        subject: "Beta R4 schema freeze note",
        design_budget_ms: 225,
    }
}
