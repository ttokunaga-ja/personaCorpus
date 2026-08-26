pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-002174"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-13"),
    ("evidence_question", "What observable evidence demonstrates that the stated control objective is met?"),
    ("evidence_instrument", "data-classification note"),
    ("review_observation", "the sample included an adverse operating condition"),
    ("challenge_performed", "a conflicting approval was escalated instead of silently merged"),
    ("review_profile", "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence."),
    ("decision_rule", "Use the dependency as context rather than proof; the local artifact must state its own conclusion and basis."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
