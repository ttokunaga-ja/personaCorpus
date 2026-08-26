pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-009940"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-13"),
    ("evidence_question", "How should a conflicting source be handled before the disposition is recorded?"),
    ("evidence_instrument", "asset accountability list"),
    ("review_observation", "the source chronology did not conceal a late correction"),
    ("challenge_performed", "a corrective action was tested for closure evidence"),
    ("review_profile", "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A repeated control statement does not establish operation when no counterexample was considered."),
    ("decision_rule", "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
