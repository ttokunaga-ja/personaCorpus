-- Cobalt Harbor Systems | Aegis Assurance | synthetic defensive record
-- Working file: 2026-07-15-personal-grc-evidence-digest-control-facts-sql-s009119.sql | Topic: evidence digest | Focus: residual-risk statement
-- Purpose: prepare a personal evidence note | Conclusion: For 2026-07-15, this working file prepare a personal evidence note focused on residual-risk statement; it supports a Medium residual-risk conclusion without asserting live-system access.
-- Ledger date anchors: 2026-07-15
-- Positive spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform | action.act-260713-011 = Publish an AC-27 reviewer-completion register; completed 2026-07-15 | date.2026-07-15 = Control owner validates the remediation plan and retention disposition
-- Spine: org.cobalt-harbor; action.act-260713-011; rel.risk-031.treatment; date.2026-07-15; term.no-kio-claim
CREATE TABLE ac27_foundation_p03_full_009119 (
  artifact_id TEXT PRIMARY KEY, review_period TEXT NOT NULL, control_id TEXT NOT NULL,
  case_id TEXT NOT NULL, evidence_set INTEGER NOT NULL, timely_attestations INTEGER NOT NULL,
  review_population INTEGER NOT NULL, residual_risk TEXT NOT NULL, action_id TEXT NOT NULL
);
INSERT INTO ac27_foundation_p03_full_009119 VALUES ('p03-full-009119', '2026-07-15', 'AC-27', 'CASE-260713-184', 184, 47, 48, 'Medium', 'ACT-260713-011');
