export const review = Object.freeze({
  artifact_id: "p03-full-002477",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "What minimal trace proves that the control remains attributable to its owner?",
  evidence_instrument: "exception ageing report",
  review_observation: "the reviewer recorded why an unavailable source was not relied upon",
  challenge_performed: "a conflicting approval was escalated instead of silently merged",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for targeted follow-up; the source chronology was complete; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is the scheduled review; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A file name is not a substitute for the source facts that support the conclusion.",
  decision_rule: "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
