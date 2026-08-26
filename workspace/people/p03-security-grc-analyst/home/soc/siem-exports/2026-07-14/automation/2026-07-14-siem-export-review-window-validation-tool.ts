export const review = Object.freeze({
  artifact_id: "p03-full-005507",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "Which responsibility boundary must be visible before the record is approved?",
  evidence_instrument: "control test worksheet",
  review_observation: "the exception route was visible to an independent reviewer",
  challenge_performed: "an ambiguous dependency was clarified before disposition",
  review_profile: "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.",
  decision_rule: "Treat the control as effective only when the counterexample has been considered and the exception path is explicit.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
