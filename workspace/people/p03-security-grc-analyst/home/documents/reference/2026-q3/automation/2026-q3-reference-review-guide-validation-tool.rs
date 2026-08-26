// Review period: 2026 Q3. This reference review guide validation tool record was assessed using closure evidence and accountable disposition; the reviewer confirmed the review conclusion.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-003394"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "What decision can be made from the retained source material without external access?"),
    ("evidence_instrument", "exception decision sheet"),
    ("review_observation", "the exception was linked to a compensating action"),
    ("challenge_performed", "a claimed control was tested against a negative sample"),
    ("review_profile", "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence."),
    ("decision_rule", "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
