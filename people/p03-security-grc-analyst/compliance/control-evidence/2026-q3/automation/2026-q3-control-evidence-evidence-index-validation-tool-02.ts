// Review period: 2026 Q3. This control evidence evidence index validation tool record was assessed using control boundary and operating ownership; the reviewer set a follow-up assurance trigger.
export const reviewPeriod = "2026 Q3";
export const review = Object.freeze({
  artifact_id: "p03-full-006959",
  organization: "Cobalt Harbor",
  risk_reference: "RISK-031",
  review_date: "2026-07-20",
  evidence_question: "Which review question exposes a gap between policy language and operating evidence?",
  evidence_instrument: "asset accountability list",
  review_observation: "the local conclusion did not depend on a portal or live endpoint",
  challenge_performed: "a corrective action was tested for closure evidence",
  review_profile: "the evidence set was collected during the follow-up review; the evidence custodian confirmed the record boundary; the reviewer sampled an adverse operating condition; the source fact supported acceptance with a documented caveat; the comparison used the prior review conclusion; the evidence was retained with the decision record; the exception path was not invoked; the reviewer found no handover impact; the decision was suitable for targeted follow-up; the source chronology contained a reconciled late entry; the control claim was independently corroborated; the review boundary identified unavailable material as a limitation; the next assurance trigger is completion of the follow-up action; the disposition was communicated to the owner; the retained rationale supports repeat review; the record remains a local offline assurance artifact",
  counterexample: "A complete-looking register is not reliable if the decision rule and disposition are absent.",
  decision_rule: "Approve when the evidence is specific, attributable, and dated; otherwise classify the item as needing corroboration.",
  disposition: "accepted with a retained review note",
  operating_boundary: "No Kio capability is used or required for this offline review."
});

export function reviewSummary() { return { ...review }; }
