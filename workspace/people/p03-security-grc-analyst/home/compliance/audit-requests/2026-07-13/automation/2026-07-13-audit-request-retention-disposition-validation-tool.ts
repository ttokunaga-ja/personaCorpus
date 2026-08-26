export const review = Object.freeze({
  artifact_id: "p03-full-002175",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-13",
  evidence_question: "What decision can be made from the retained source material without external access?",
  evidence_instrument: "risk treatment worksheet",
  review_observation: "the exception route was visible to an independent reviewer",
  challenge_performed: "a summary was checked against the underlying decision minute",
  review_profile: "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.",
  decision_rule: "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
