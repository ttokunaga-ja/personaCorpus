export const review = Object.freeze({
  artifact_id: "p03-full-005627",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "What minimal trace proves that the control remains attributable to its owner?",
  evidence_instrument: "asset accountability list",
  review_observation: "the owner and custodian were named consistently",
  challenge_performed: "a compensating action was checked for accountable ownership",
  review_profile: "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dated narrative without a named custodian is insufficient because responsibility cannot be tested.",
  decision_rule: "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
