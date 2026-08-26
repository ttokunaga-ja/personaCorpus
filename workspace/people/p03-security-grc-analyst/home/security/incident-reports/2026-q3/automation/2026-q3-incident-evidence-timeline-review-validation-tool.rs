// Review period: 2026 Q3. This incident evidence timeline review validation tool record was assessed using closure evidence and accountable disposition; the reviewer recorded a bounded exception.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-008921"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "How does the review distinguish complete evidence from an unsupported assertion?"),
    ("evidence_instrument", "policy-to-practice comparison"),
    ("review_observation", "the source record separated assertion from corroboration"),
    ("challenge_performed", "a local conclusion was checked for reliance on external access"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A file name is not a substitute for the source facts that support the conclusion."),
    ("decision_rule", "Treat the control as effective only when the counterexample has been considered and the exception path is explicit."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
