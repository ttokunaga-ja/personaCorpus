// Review period: 2026 Q3. This audit request request response validation tool record was assessed using control boundary and operating ownership; the reviewer documented an ownership check.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-008428"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "How should a conflicting source be handled before the disposition is recorded?"),
    ("evidence_instrument", "sampling rationale"),
    ("review_observation", "the rationale accounted for an incomplete sample"),
    ("challenge_performed", "a policy reference was checked against the operating record"),
    ("review_profile", "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer."),
    ("decision_rule", "Treat the control as effective only when the counterexample has been considered and the exception path is explicit."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
