export const review = Object.freeze({
  artifact_id: "p03-full-002561",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-13",
  evidence_question: "What decision can be made from the retained source material without external access?",
  evidence_instrument: "reviewer observation log",
  review_observation: "the evidence owner confirmed the record's completeness",
  challenge_performed: "a recertification statement was tested for an actual review event",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dependency reference is not meaningful unless the receiving artifact explains why it was consulted.",
  decision_rule: "Approve when the evidence is specific, attributable, and dated; otherwise classify the item as needing corroboration.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
