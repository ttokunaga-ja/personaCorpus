export const review = Object.freeze({
  artifact_id: "p03-full-002441",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "Which source fact would cause the reviewer to accept the control conclusion?",
  evidence_instrument: "retention schedule extract",
  review_observation: "the exception route was visible to an independent reviewer",
  challenge_performed: "a retained file was assessed for evidence freshness",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dated narrative without a named custodian is insufficient because responsibility cannot be tested.",
  decision_rule: "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
