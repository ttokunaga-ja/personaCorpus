export const review = Object.freeze({
  artifact_id: "p03-full-003541",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-13",
  evidence_question: "How should a conflicting source be handled before the disposition is recorded?",
  evidence_instrument: "asset accountability list",
  review_observation: "the evidence could be followed without an external system",
  challenge_performed: "an incomplete register was challenged for its missing decision rule",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A complete-looking register is not reliable if the decision rule and disposition are absent.",
  decision_rule: "Treat the control as effective only when the counterexample has been considered and the exception path is explicit.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
