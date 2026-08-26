export const review = Object.freeze({
  artifact_id: "p03-full-002522",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-13",
  evidence_question: "Which responsibility boundary must be visible before the record is approved?",
  evidence_instrument: "service inventory snapshot",
  review_observation: "the source record separated assertion from corroboration",
  challenge_performed: "a compensating action was checked for accountable ownership",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer.",
  decision_rule: "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
