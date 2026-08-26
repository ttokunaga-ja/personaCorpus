export const review = Object.freeze({
  artifact_id: "p03-full-002861",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "What minimal trace proves that the control remains attributable to its owner?",
  evidence_instrument: "process walkthrough note",
  review_observation: "the review basis could be repeated by a successor",
  challenge_performed: "a policy reference was checked against the operating record",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dependency reference is not meaningful unless the receiving artifact explains why it was consulted.",
  decision_rule: "Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
