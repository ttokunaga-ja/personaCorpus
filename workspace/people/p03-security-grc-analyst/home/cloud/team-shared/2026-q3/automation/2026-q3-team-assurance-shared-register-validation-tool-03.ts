// Review period: 2026 Q3. This team assurance shared register validation tool record was assessed using access attestation and compensating action; the reviewer set a follow-up assurance trigger.
export const reviewPeriod = "2026 Q3";
export const review = Object.freeze({
  artifact_id: "p03-full-008501",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "Which review question exposes a gap between policy language and operating evidence?",
  evidence_instrument: "sampling rationale",
  review_observation: "the decision retained its escalation threshold",
  challenge_performed: "an absent relationship was made explicit before acceptance",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance without qualification; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.",
  decision_rule: "Escalate conflicting observations to the designated reviewer rather than resolving them through filename or identifier changes.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
