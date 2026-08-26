export const review = Object.freeze({
  artifact_id: "p03-full-007634",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-13",
  evidence_question: "Which responsibility boundary must be visible before the record is approved?",
  evidence_instrument: "versioned review register",
  review_observation: "the disposition stated both confidence and residual uncertainty",
  challenge_performed: "a broad claim was narrowed to the observable control action",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A file name is not a substitute for the source facts that support the conclusion.",
  decision_rule: "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
