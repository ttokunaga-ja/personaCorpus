pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-006604"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "What observable evidence demonstrates that the stated control objective is met?"),
    ("evidence_instrument", "retention schedule extract"),
    ("review_observation", "the review distinguished a planned activity from completed operation"),
    ("challenge_performed", "a review result was compared with a counterexample"),
    ("review_profile", "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A complete-looking register is not reliable if the decision rule and disposition are absent."),
    ("decision_rule", "Approve when the evidence is specific, attributable, and dated; otherwise classify the item as needing corroboration."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
