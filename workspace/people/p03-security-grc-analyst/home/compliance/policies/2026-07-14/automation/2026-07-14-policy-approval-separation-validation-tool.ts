export const review = Object.freeze({
  artifact_id: "p03-full-008623",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "How does the review distinguish complete evidence from an unsupported assertion?",
  evidence_instrument: "control-to-evidence map",
  review_observation: "the owner acknowledged a limitation without weakening traceability",
  challenge_performed: "an exception rationale was tested for a documented decision rule",
  review_profile: "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence.",
  decision_rule: "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
