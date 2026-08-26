pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-006834"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-14"),
    ("evidence_question", "What decision can be made from the retained source material without external access?"),
    ("evidence_instrument", "outcome verification note"),
    ("review_observation", "the review distinguished a planned activity from completed operation"),
    ("challenge_performed", "a missing date was treated as a traceability concern"),
    ("review_profile", "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion."),
    ("decision_rule", "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
