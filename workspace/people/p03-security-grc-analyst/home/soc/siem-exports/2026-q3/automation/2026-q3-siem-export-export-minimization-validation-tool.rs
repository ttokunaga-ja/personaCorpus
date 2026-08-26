// Review period: 2026 Q3. This siem export export minimization validation tool record was assessed using evidence custody and reviewer independence; the reviewer recorded a bounded exception.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-009409"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "How does the review distinguish complete evidence from an unsupported assertion?"),
    ("evidence_instrument", "control narrative comparison"),
    ("review_observation", "the approval boundary matched the operating record"),
    ("challenge_performed", "a policy reference was checked against the operating record"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion."),
    ("decision_rule", "Use the dependency as context rather than proof; the local artifact must state its own conclusion and basis."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
