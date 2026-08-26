export const review = Object.freeze({
  artifact_id: "p03-full-008769",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-15",
  evidence_question: "Which responsibility boundary must be visible before the record is approved?",
  evidence_instrument: "exception decision sheet",
  review_observation: "the reviewer identified the fact that would reverse the outcome",
  challenge_performed: "a generic assurance phrase was rejected pending source facts",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A dated narrative without a named custodian is insufficient because responsibility cannot be tested.",
  decision_rule: "Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
