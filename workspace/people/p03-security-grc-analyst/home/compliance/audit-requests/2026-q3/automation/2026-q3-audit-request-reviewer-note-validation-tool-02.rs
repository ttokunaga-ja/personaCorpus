// Review period: 2026 Q3. This audit request reviewer note validation tool record was assessed using closure evidence and accountable disposition; the reviewer recorded a bounded exception.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-002062"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "How does the review distinguish complete evidence from an unsupported assertion?"),
    ("evidence_instrument", "service inventory snapshot"),
    ("review_observation", "the conclusion matched the scope of the stated control"),
    ("challenge_performed", "a missing date was treated as a traceability concern"),
    ("review_profile", "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A complete-looking register is not reliable if the decision rule and disposition are absent."),
    ("decision_rule", "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
