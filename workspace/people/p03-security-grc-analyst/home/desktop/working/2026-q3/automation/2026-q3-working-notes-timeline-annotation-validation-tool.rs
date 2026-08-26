// Review period: 2026 Q3. This working notes timeline annotation validation tool record was assessed using access attestation and compensating action; the reviewer identified a corroboration need.
pub const REVIEW_PERIOD: &str = "2026 Q3";
pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-007973"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-20"),
    ("evidence_question", "Which source fact would cause the reviewer to accept the control conclusion?"),
    ("evidence_instrument", "exception ageing report"),
    ("review_observation", "the control boundary was explained in practical terms"),
    ("challenge_performed", "a broad claim was narrowed to the observable control action"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion."),
    ("decision_rule", "Use the dependency as context rather than proof; the local artifact must state its own conclusion and basis."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
