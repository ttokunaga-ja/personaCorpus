// Review period: 2026 Q3. This audit request retention disposition validation tool record was assessed using control boundary and operating ownership; the reviewer documented an ownership check.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-002099"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "How should a conflicting source be handled before the disposition is recorded?"),
    ("evidence_instrument", "handover checklist"),
    ("review_observation", "the owner and custodian were named consistently"),
    ("challenge_performed", "an ambiguous dependency was clarified before disposition"),
    ("review_profile", "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A dated narrative without a named custodian is insufficient because responsibility cannot be tested."),
    ("decision_rule", "Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
