export const review = Object.freeze({
  artifact_id: "p03-full-004495",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-15",
  evidence_question: "Which source fact would cause the reviewer to accept the control conclusion?",
  evidence_instrument: "exception ageing report",
  review_observation: "the record explained the relationship between policy and practice",
  challenge_performed: "a broad claim was narrowed to the observable control action",
  review_profile: "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dated narrative without a named custodian is insufficient because responsibility cannot be tested.",
  decision_rule: "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
