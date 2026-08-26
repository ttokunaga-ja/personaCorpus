-- Synthetic AC-27 foundation query: p03-full-004647
-- spine: org.cobalt-harbor, evidence.set-184, rel.ac27.evidence-184, date.2026-07-20, term.no-kio-claim
-- Seven-day closure follow-up confirmed the residual-risk status and checksum reconciliation.
-- Evidence indexing links bounded evidence references to the AC-27 review and retention disposition.
-- canonical value: Evidence set 184 contains 24 synthetic items: 12 access-review attestations, 6 approval records, 4 checksum manifests, and 2 exception records.
SELECT 'p03-full-004647' AS artifact_id, 'p03-src-004647' AS source_id, 'Cobalt Harbor Systems' AS organization, 'AC-27' AS control_id, 'CASE-260713-184' AS case_id, 'RISK-031' AS risk_id, 184 AS evidence_set, 'follow-up' AS review_stage, 'EV-184-16' AS evidence_card, 'PRIV-40' AS review_lane, 6 AS verification_pass, 4647 AS working_paper_sequence, 48 AS reviewed_roles, 47 AS timely_attestations, 'Medium' AS residual_risk;
