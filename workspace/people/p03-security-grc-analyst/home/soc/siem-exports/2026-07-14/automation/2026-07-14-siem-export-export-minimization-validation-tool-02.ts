export const review = Object.freeze({
  artifact_id: "p03-full-005473",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "What decision can be made from the retained source material without external access?",
  evidence_instrument: "sampling rationale",
  review_observation: "the accountable role was preserved through handover",
  challenge_performed: "a summary was checked against the underlying decision minute",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dependency reference is not meaningful unless the receiving artifact explains why it was consulted.",
  decision_rule: "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
