export const review = Object.freeze({
  artifact_id: "p03-full-007368",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-15",
  evidence_question: "What observable evidence demonstrates that the stated control objective is met?",
  evidence_instrument: "exception ageing report",
  review_observation: "the source chronology did not conceal a late correction",
  challenge_performed: "a handoff record was assessed for continuity of evidence custody",
  review_profile: "the evidence set was collected before the approval meeting; the accountable owner confirmed the operating boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was corroborated by a compensating observation; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A repeated control statement does not establish operation when no counterexample was considered.",
  decision_rule: "Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
