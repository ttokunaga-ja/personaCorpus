export const review = Object.freeze({
  artifact_id: "p03-full-004227",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "What observable evidence demonstrates that the stated control objective is met?",
  evidence_instrument: "training completion record",
  review_observation: "the owner and custodian were named consistently",
  challenge_performed: "a recertification statement was tested for an actual review event",
  review_profile: "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled a routine operating condition; the source fact supported acceptance without qualification; the comparison used the prior review conclusion; the evidence was retained in the review register; the exception path was not invoked; the reviewer assessed a handover impact; the decision was suitable for routine monitoring; the source chronology was complete; the control claim was independently corroborated; the review boundary excluded unavailable material; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A repeated control statement does not establish operation when no counterexample was considered.",
  decision_rule: "Reject unsupported claims and retain a concise explanation of the missing fact for the next review cycle.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
