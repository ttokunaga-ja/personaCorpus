export const review = Object.freeze({
  artifact_id: "p03-full-009145",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "What minimal trace proves that the control remains attributable to its owner?",
  evidence_instrument: "control-to-evidence map",
  review_observation: "the conclusion matched the scope of the stated control",
  challenge_performed: "a single reviewer conclusion was checked for independent challenge",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A generic assurance claim is rejected when it cannot be traced to an owner, date, and retained evidence.",
  decision_rule: "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
