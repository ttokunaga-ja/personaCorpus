pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-005099"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "Which review question exposes a gap between policy language and operating evidence?"),
    ("evidence_instrument", "boundary decision record"),
    ("review_observation", "the owner and custodian were named consistently"),
    ("challenge_performed", "a control objective was tested for a measurable outcome"),
    ("review_profile", "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A complete-looking register is not reliable if the decision rule and disposition are absent."),
    ("decision_rule", "Treat the control as effective only when the counterexample has been considered and the exception path is explicit."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
