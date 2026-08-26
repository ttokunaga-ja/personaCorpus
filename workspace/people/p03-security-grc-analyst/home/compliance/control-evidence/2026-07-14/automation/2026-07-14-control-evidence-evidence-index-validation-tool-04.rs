pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-009527"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-14"),
    ("evidence_question", "How does the review distinguish complete evidence from an unsupported assertion?"),
    ("evidence_instrument", "asset accountability list"),
    ("review_observation", "the exception was linked to a compensating action"),
    ("challenge_performed", "a broad claim was narrowed to the observable control action"),
    ("review_profile", "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer."),
    ("decision_rule", "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
