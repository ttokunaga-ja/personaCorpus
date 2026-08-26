export const review = Object.freeze({
  artifact_id: "p03-full-001874",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-15",
  evidence_question: "Which review question exposes a gap between policy language and operating evidence?",
  evidence_instrument: "evidence freshness check",
  review_observation: "the conclusion matched the scope of the stated control",
  challenge_performed: "a single reviewer conclusion was checked for independent challenge",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence.",
  decision_rule: "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
