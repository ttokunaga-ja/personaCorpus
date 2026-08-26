pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-005433"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-14"),
    ("evidence_question", "What observable evidence demonstrates that the stated control objective is met?"),
    ("evidence_instrument", "policy-to-practice comparison"),
    ("review_observation", "the disposition stated both confidence and residual uncertainty"),
    ("challenge_performed", "an incomplete register was challenged for its missing decision rule"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A dependency reference is not meaningful unless the receiving artifact explains why it was consulted."),
    ("decision_rule", "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
