export const review = Object.freeze({
  artifact_id: "p03-full-008090",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-13",
  evidence_question: "How does the review distinguish complete evidence from an unsupported assertion?",
  evidence_instrument: "boundary decision record",
  review_observation: "the owner acknowledged a limitation without weakening traceability",
  challenge_performed: "a contradictory note was reconciled against the retained source",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained in the review register; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence.",
  decision_rule: "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
