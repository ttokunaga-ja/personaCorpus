// Review period: 2026 Q3. This policy exception handling validation tool record was assessed using risk treatment and residual uncertainty; the reviewer accepted the retained basis.
export const reviewPeriod = "2026 Q3";
export const review = Object.freeze({
  artifact_id: "p03-full-008922",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "What observable evidence demonstrates that the stated control objective is met?",
  evidence_instrument: "reviewer observation log",
  review_observation: "the review showed how contradictory evidence was resolved",
  challenge_performed: "a missing date was treated as a traceability concern",
  review_profile: "the evidence set was collected before the approval meeting; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance with a documented caveat; the comparison used the current control objective; the evidence was retained in the review register; the exception path was considered and recorded; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is the scheduled review; the disposition was communicated to the independent reviewer; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A file name is not a substitute for the source facts that support the conclusion.",
  decision_rule: "Use the dependency as context rather than proof; the local artifact must state its own conclusion and basis.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
