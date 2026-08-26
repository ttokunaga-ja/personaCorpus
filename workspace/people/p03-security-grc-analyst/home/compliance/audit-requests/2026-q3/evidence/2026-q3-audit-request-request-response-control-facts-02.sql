-- Review period: 2026 Q3. This audit request request response control facts sql record was assessed using risk treatment and residual uncertainty; the reviewer set a follow-up assurance trigger.
-- Offline, static assurance review record.
SELECT
  'p03-full-004548' AS artifact_id,
  'p03-src-004548' AS source_record,
  'p03-primary-07' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'Which review question exposes a gap between policy language and operating evidence?' AS evidence_question,
  'risk owner retained decision record for this review.' AS source_facts,
  'A single approval cannot settle a conflicting record; the discrepancy must remain visible to the reviewer.' AS counterexample,
  'boundary decision record' AS evidence_instrument,
  'the review preserved a clear separation of duties' AS review_observation,
  'a recertification statement was tested for an actual review event' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the accountable owner confirmed the operating boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance without qualification", "the comparison used the prior review conclusion", "the evidence was retained in the review register", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for targeted follow-up", "the source chronology was complete", "the control claim was independently corroborated", "the review boundary excluded unavailable material", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Record an approval only after the reviewer can reproduce the conclusion from the retained facts without network access.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
