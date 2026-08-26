pub fn review_summary() -> Vec<(&'static str, &'static str)> {
    vec![
    ("artifact_id", "p03-full-005928"),
    ("organization", "Cobalt Harbor"),
    ("risk_reference", "RISK-031"),
    ("review_date", "2026-07-14"),
    ("evidence_question", "What decision can be made from the retained source material without external access?"),
    ("evidence_instrument", "contract obligation extract"),
    ("review_observation", "the rationale accounted for an incomplete sample"),
    ("challenge_performed", "a stale approval was compared with the current ownership assignment"),
    ("review_profile", "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact"),
    ("counterexample", "A repeated control statement does not establish operation when no counterexample was considered."),
    ("decision_rule", "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access."),
    ("disposition", "accepted with a retained review note"),
    ("operating_boundary", "No Kio capability is used or required for this offline review.")
    ]
}
