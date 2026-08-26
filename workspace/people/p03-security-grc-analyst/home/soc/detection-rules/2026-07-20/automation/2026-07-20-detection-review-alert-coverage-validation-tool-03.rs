pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-005661"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "What observable evidence demonstrates that the stated control objective is met?"),
    ("evidence_instrument", "control-to-evidence map"),
    ("review_observation", "the local conclusion did not depend on a portal or live endpoint"),
    ("challenge_performed", "a handoff record was assessed for continuity of evidence custody"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained in the review register; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer."),
    ("decision_rule", "Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
