pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-007213"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-13"),
    ("evidence_question", "What decision can be made from the retained source material without external access?"),
    ("evidence_instrument", "control-to-evidence map"),
    ("review_observation", "the review distinguished a planned activity from completed operation"),
    ("challenge_performed", "a local conclusion was checked for reliance on external access"),
    ("review_profile", "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A dated narrative without a named custodian is insufficient because responsibility cannot be tested."),
    ("decision_rule", "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition."),
    ("disposition", "escalated for corroboration with the decision basis retained"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
