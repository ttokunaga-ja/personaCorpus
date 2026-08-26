pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-008879"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "What observable evidence demonstrates that the stated control objective is met?"),
    ("evidence_instrument", "handover checklist"),
    ("review_observation", "the evidence could be followed without an external system"),
    ("challenge_performed", "a single reviewer conclusion was checked for independent challenge"),
    ("review_profile", "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A file name is not a substitute for the source facts that support the conclusion."),
    ("decision_rule", "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
