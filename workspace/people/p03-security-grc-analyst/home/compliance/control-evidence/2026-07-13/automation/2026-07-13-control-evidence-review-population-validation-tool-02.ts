export const review = Object.freeze({
  artifact_id: "p03-full-009977",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-13",
  evidence_question: "Which source fact would cause the reviewer to accept the control conclusion?",
  evidence_instrument: "evidence freshness check",
  review_observation: "the accountable role was preserved through handover",
  challenge_performed: "a stale approval was compared with the current ownership assignment",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained with the decision record; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was corroborated by a compensating observation; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A repeated control statement does not establish operation when no counterexample was considered.",
  decision_rule: "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
