export const review = Object.freeze({
  artifact_id: "p03-full-001342",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "What minimal trace proves that the control remains attributable to its owner?",
  evidence_instrument: "sampling rationale",
  review_observation: "the decision retained its escalation threshold",
  challenge_performed: "a control objective was tested for a measurable outcome",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dated narrative without a named custodian is insufficient because responsibility cannot be tested.",
  decision_rule: "Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
