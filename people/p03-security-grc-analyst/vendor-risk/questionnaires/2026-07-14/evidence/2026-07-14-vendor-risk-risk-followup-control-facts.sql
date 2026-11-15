-- Synthetic Cobalt Harbor Systems supplier assurance foundation record.
-- Content spine trace: org.cobalt-harbor: Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | risk.risk-031: RISK-031: incomplete quarterly privileged-access review evidence could prevent AC-27 operating-effectiveness confirmation; inherent High and residual Medium after bounded remediation. | date.2026-07-14: Evidence normalization, checksum confirmation, and action-owner acknowledgement.
CREATE TABLE IF NOT EXISTS vendor_risk_control_fact_001483 (
  record_id TEXT PRIMARY KEY,
  control_id TEXT NOT NULL,
  case_id TEXT NOT NULL,
  risk_id TEXT NOT NULL,
  evidence_set INTEGER NOT NULL,
  residual_risk TEXT NOT NULL,
  source_note TEXT NOT NULL
);
INSERT INTO vendor_risk_control_fact_001483
  (record_id, control_id, case_id, risk_id, evidence_set, residual_risk, source_note)
VALUES
  ('VRF-001483', 'AC-27', 'CASE-260713-184', 'RISK-031', 184, 'Medium', 'Synthetic defensive supplier assurance record; action ACT-260713-013 due 2026-07-31.');
