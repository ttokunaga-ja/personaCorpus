pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-009071"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-13"),
    ("evidence_question", "Which source fact would cause the reviewer to accept the control conclusion?"),
    ("evidence_instrument", "service inventory snapshot"),
    ("review_observation", "the reviewer could locate the controlling decision without inference"),
    ("challenge_performed", "a missing date was treated as a traceability concern"),
    ("review_profile", "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer."),
    ("decision_rule", "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
