export const review = Object.freeze({
  artifact_id: "p03-full-006572",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "Which review question exposes a gap between policy language and operating evidence?",
  evidence_instrument: "policy-to-practice comparison",
  review_observation: "the evidence could be followed without an external system",
  challenge_performed: "an incomplete register was challenged for its missing decision rule",
  review_profile: "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dated narrative without a named custodian is insufficient because responsibility cannot be tested.",
  decision_rule: "Approve when the evidence is specific, attributable, and dated; otherwise classify the item as needing corroboration.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
