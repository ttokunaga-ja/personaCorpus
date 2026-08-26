// Review period: 2026 Q3. This policy retention validation tool record was assessed using evidence custody and reviewer independence; the reviewer retained a challenge response.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-008397"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "What minimal trace proves that the control remains attributable to its owner?"),
    ("evidence_instrument", "quality review annotation"),
    ("review_observation", "the owner acknowledged a limitation without weakening traceability"),
    ("challenge_performed", "a broad claim was narrowed to the observable control action"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A dated narrative without a named custodian is insufficient because responsibility cannot be tested."),
    ("decision_rule", "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
