pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-004673"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-15"),
    ("evidence_question", "How should a conflicting source be handled before the disposition is recorded?"),
    ("evidence_instrument", "signed ownership matrix"),
    ("review_observation", "the rationale accounted for an incomplete sample"),
    ("challenge_performed", "an old exception was assessed for continued relevance"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion."),
    ("decision_rule", "Treat the control as effective only when the counterexample has been considered and the exception path is explicit."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
