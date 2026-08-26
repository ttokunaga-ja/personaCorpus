export const review = Object.freeze({
  artifact_id: "p03-full-003430",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-13",
  evidence_question: "What minimal trace proves that the control remains attributable to its owner?",
  evidence_instrument: "risk treatment worksheet",
  review_observation: "the control boundary was explained in practical terms",
  challenge_performed: "an unassigned exception was tested against the escalation route",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A file name is not a substitute for the source facts that support the conclusion.",
  decision_rule: "Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
