export const review = Object.freeze({
  artifact_id: "p03-full-009528",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "What minimal trace proves that the control remains attributable to its owner?",
  evidence_instrument: "approval trail summary",
  review_observation: "the verification date was visible beside the source fact",
  challenge_performed: "a single reviewer conclusion was checked for independent challenge",
  review_profile: "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A complete-looking register is not reliable if the decision rule and disposition are absent.",
  decision_rule: "Treat the control as effective only when the counterexample has been considered and the exception path is explicit.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
