export const review = Object.freeze({
  artifact_id: "p03-full-002706",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "What minimal trace proves that the control remains attributable to its owner?",
  evidence_instrument: "boundary decision record",
  review_observation: "the review basis could be repeated by a successor",
  challenge_performed: "a generic assurance phrase was rejected pending source facts",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained in the review register; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A file name is not a substitute for the source facts that support the conclusion.",
  decision_rule: "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
