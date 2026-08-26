export const review = Object.freeze({
  artifact_id: "p03-full-003201",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-15",
  evidence_question: "How should a conflicting source be handled before the disposition is recorded?",
  evidence_instrument: "training completion record",
  review_observation: "the accountable role was preserved through handover",
  challenge_performed: "a change record was tested for reviewer independence",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A complete-looking register is not reliable if the decision rule and disposition are absent.",
  decision_rule: "Reject unsupported claims and retain a concise explanation of the missing fact for the next review cycle.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
