pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-008695"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-15"),
    ("evidence_question", "Which source fact would cause the reviewer to accept the control conclusion?"),
    ("evidence_instrument", "service inventory snapshot"),
    ("review_observation", "the evidence sequence exposed a material dependency"),
    ("challenge_performed", "a retained file was assessed for evidence freshness"),
    ("review_profile", "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence."),
    ("decision_rule", "Approve when the evidence is specific, attributable, and dated; otherwise classify the item as needing corroboration."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
