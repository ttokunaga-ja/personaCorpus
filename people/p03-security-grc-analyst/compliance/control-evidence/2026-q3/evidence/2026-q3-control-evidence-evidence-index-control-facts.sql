-- Synthetic AC-27 foundation query: p03-full-001883
-- spine: org.cobalt-harbor, control.approval-dual, rel.ac27.evidence-184, period.2026-q3, term.no-kio-claim
-- Quarterly governance planning preserves the completed July review and schedules evidence retention follow-through.
-- Evidence indexing links bounded evidence references to the AC-27 review and retention disposition.
-- canonical value: AC-27 requires preparer and reviewer role separation for each exception disposition.
SELECT 'p03-full-001883' AS artifact_id, 'p03-src-001883' AS source_id, 'Cobalt Harbor Systems' AS organization, 'AC-27' AS control_id, 'CASE-260713-184' AS case_id, 'RISK-031' AS risk_id, 184 AS evidence_set, 'governance-planning' AS review_stage, 'EV-184-12' AS evidence_card, 'PRIV-12' AS review_lane, 3 AS verification_pass, 1883 AS working_paper_sequence, 48 AS reviewed_roles, 47 AS timely_attestations, 'Medium' AS residual_risk;
