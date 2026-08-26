pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-002521"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-13"),
    ("evidence_question", "Which review question exposes a gap between policy language and operating evidence?"),
    ("evidence_instrument", "handover checklist"),
    ("review_observation", "the exception was linked to a compensating action"),
    ("challenge_performed", "a generic assurance phrase was rejected pending source facts"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence."),
    ("decision_rule", "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
