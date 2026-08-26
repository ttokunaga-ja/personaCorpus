pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-006195"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-13"),
    ("evidence_question", "What minimal trace proves that the control remains attributable to its owner?"),
    ("evidence_instrument", "evidence custody note"),
    ("review_observation", "the approved scope was not expanded beyond the available evidence"),
    ("challenge_performed", "a compensating action was checked for accountable ownership"),
    ("review_profile", "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A dependency reference is not meaningful unless the receiving artifact explains why it was consulted."),
    ("decision_rule", "Treat the control as effective only when the counterexample has been considered and the exception path is explicit."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
