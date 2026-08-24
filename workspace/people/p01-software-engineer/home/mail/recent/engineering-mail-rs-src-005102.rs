//! Synthetic attachment referenced by MSG-2026Q3-005102; no network or credentials.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailEvidence {
    pub message_id: &'static str,
    pub project: &'static str,
    pub subject: &'static str,
    pub design_budget_ms: u16,
}

pub fn reviewed_mail_evidence() -> MailEvidence {
    MailEvidence {
        message_id: "MSG-2026Q3-005102",
        project: "BETA-R4",
        subject: "staging rollout observation",
        design_budget_ms: 225,
    }
}
