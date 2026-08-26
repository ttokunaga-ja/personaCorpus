export const review = Object.freeze({
  artifact_id: "p03-full-002668",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "How should a conflicting source be handled before the disposition is recorded?",
  evidence_instrument: "retention schedule extract",
  review_observation: "the exception was linked to a compensating action",
  challenge_performed: "a duplicated statement was not accepted as independent corroboration",
  review_profile: "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer.",
  decision_rule: "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
