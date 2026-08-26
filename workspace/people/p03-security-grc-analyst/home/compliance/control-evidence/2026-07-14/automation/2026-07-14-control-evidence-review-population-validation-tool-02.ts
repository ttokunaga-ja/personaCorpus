export const review = Object.freeze({
  artifact_id: "p03-full-004977",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "Which responsibility boundary must be visible before the record is approved?",
  evidence_instrument: "quality review annotation",
  review_observation: "the sample included an adverse operating condition",
  challenge_performed: "an ambiguous dependency was clarified before disposition",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A file name is not a substitute for the source facts that support the conclusion.",
  decision_rule: "Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
