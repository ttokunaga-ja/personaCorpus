pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-009034"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-13"),
    ("evidence_question", "Which source fact would cause the reviewer to accept the control conclusion?"),
    ("evidence_instrument", "evidence custody note"),
    ("review_observation", "the record explained why an exception remained open"),
    ("challenge_performed", "a retained file was assessed for evidence freshness"),
    ("review_profile", "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A dependency reference is not meaningful unless the receiving artifact explains why it was consulted."),
    ("decision_rule", "Use the dependency as context rather than proof; the local artifact must state its own conclusion and basis."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
