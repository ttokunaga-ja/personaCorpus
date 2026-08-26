pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-001909"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-15"),
    ("evidence_question", "What decision can be made from the retained source material without external access?"),
    ("evidence_instrument", "outcome verification note"),
    ("review_observation", "the source record separated assertion from corroboration"),
    ("challenge_performed", "a process narrative was checked for a named decision owner"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A complete-looking register is not reliable if the decision rule and disposition are absent."),
    ("decision_rule", "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
