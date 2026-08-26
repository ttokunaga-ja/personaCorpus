export const review = Object.freeze({
  artifact_id: "p03-full-008661",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-15",
  evidence_question: "Which source fact would cause the reviewer to accept the control conclusion?",
  evidence_instrument: "data-classification note",
  review_observation: "the evidence could be followed without an external system",
  challenge_performed: "a late evidence submission was separated from the original decision",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dependency reference is not meaningful unless the receiving artifact explains why it was consulted.",
  decision_rule: "Approve when the evidence is specific, attributable, and dated; otherwise classify the item as needing corroboration.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
