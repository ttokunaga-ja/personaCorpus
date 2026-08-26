// Review period: 2026 Q3. This reference evidence template validation tool record was assessed using retention basis and follow-up condition; the reviewer recorded a bounded exception.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-008539"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "How does the review distinguish complete evidence from an unsupported assertion?"),
    ("evidence_instrument", "handover checklist"),
    ("review_observation", "the evidence sequence exposed a material dependency"),
    ("challenge_performed", "a scope boundary was compared with the evidence collection boundary"),
    ("review_profile", "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence."),
    ("decision_rule", "Reject unsupported claims and retain a concise explanation of the missing fact for the next review cycle."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
