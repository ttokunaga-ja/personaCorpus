pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-005472"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-14"),
    ("evidence_question", "What minimal trace proves that the control remains attributable to its owner?"),
    ("evidence_instrument", "exception decision sheet"),
    ("review_observation", "the evidence could be followed without an external system"),
    ("challenge_performed", "an incomplete register was challenged for its missing decision rule"),
    ("review_profile", "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer."),
    ("decision_rule", "Approve when the evidence is specific, attributable, and dated; otherwise classify the item as needing corroboration."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
