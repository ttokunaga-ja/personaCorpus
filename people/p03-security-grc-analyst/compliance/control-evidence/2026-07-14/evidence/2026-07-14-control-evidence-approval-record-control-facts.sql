-- Synthetic AC-27 foundation query: p03-full-002318
-- spine: org.cobalt-harbor, control.approval-dual, rel.ac27.evidence-184, date.2026-07-14, term.no-kio-claim
-- Checksum normalization and action-owner acknowledgement were reviewed without accessing a live system.
-- Approval separation records preparer and reviewer roles for the synthetic exception disposition.
-- canonical value: AC-27 requires preparer and reviewer role separation for each exception disposition.
SELECT 'p03-full-002318' AS artifact_id, 'p03-src-002318' AS source_id, 'Cobalt Harbor Systems' AS organization, 'AC-27' AS control_id, 'CASE-260713-184' AS case_id, 'RISK-031' AS risk_id, 184 AS evidence_set, 'normalization' AS review_stage, 'EV-184-15' AS evidence_card, 'PRIV-15' AS review_lane, 9 AS verification_pass, 2318 AS working_paper_sequence, 48 AS reviewed_roles, 47 AS timely_attestations, 'Medium' AS residual_risk;
