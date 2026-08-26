export const review = Object.freeze({
  artifact_id: "p03-full-001270",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-14",
  evidence_question: "How should a conflicting source be handled before the disposition is recorded?",
  evidence_instrument: "risk treatment worksheet",
  review_observation: "the accountable role was preserved through handover",
  challenge_performed: "a generic assurance phrase was rejected pending source facts",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.",
  decision_rule: "Treat the control as effective only when the counterexample has been considered and the exception path is explicit.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
