export const review = Object.freeze({
  artifact_id: "p03-full-004189",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "What observable evidence demonstrates that the stated control objective is met?",
  evidence_instrument: "policy-to-practice comparison",
  review_observation: "the exception route was visible to an independent reviewer",
  challenge_performed: "a handoff record was assessed for continuity of evidence custody",
  review_profile: "the evidence set was collected during the follow-up review; the accountable owner confirmed the operating boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer found no handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A status label alone is not evidence; it must be tied to a source fact and a reviewer conclusion.",
  decision_rule: "Accept only when the source fact, accountable role, and review conclusion agree; otherwise record a follow-up disposition.",
  disposition: "escalated for corroboration with the decision basis retained",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
