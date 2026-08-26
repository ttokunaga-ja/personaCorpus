-- Review period: 2026 Q3. This siem export event category control facts sql record was assessed using exception ageing and escalation criteria; the reviewer recorded a bounded exception.
-- Offline, static assurance review record.
SELECT
  'p03-full-008148' AS artifact_id,
  'p03-src-008148' AS source_record,
  'p03-primary-09' AS scope,
  'Cobalt Harbor' AS organization,
  '2026-07-20' AS review_date,
  'RISK-031' AS risk_reference,
  'AC-27 relates to evidence set 184' AS relationship,
  'no upstream dependency' AS dependency_context,
  'How does the review distinguish complete evidence from an unsupported assertion?' AS evidence_question,
  'service owner retained contract evidence for this review.' AS source_facts,
  'A dependency reference is not meaningful unless the receiving artifact explains why it was consulted.' AS counterexample,
  'recertification record' AS evidence_instrument,
  'the conclusion matched the scope of the stated control' AS review_observation,
  'an incomplete register was challenged for its missing decision rule' AS challenge_performed,
  '["the evidence set was collected before the approval meeting", "the accountable owner confirmed the operating boundary", "the reviewer sampled an adverse operating condition", "the source fact supported acceptance without qualification", "the comparison used the current control objective", "the evidence was retained in the review register", "the exception path was considered and recorded", "the reviewer assessed a handover impact", "the decision was suitable for targeted follow-up", "the source chronology contained a reconciled late entry", "the control claim was corroborated by a compensating observation", "the review boundary identified unavailable material as a limitation", "the next assurance trigger is completion of the follow-up action", "the disposition was communicated to the owner", "the retained rationale supports repeat review", "the record remains a local offline assurance artifact"]' AS review_profile,
  'Preserve a bounded decision: cite the source, explain the rule, and state whether the result is accepted or escalated.' AS decision_rule,
  'accepted with a retained review note' AS disposition,
  'No Kio capability is used or required for this offline review.' AS operating_boundary;
