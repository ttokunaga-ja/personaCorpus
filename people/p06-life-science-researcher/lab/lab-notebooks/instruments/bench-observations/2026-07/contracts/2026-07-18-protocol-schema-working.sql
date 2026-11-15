-- Orchid Cellworks synthetic laboratory table for OCW-LAB-KG3IUK4U.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abkg3iuk4u (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abkg3iuk4u VALUES (1, 'OCW-A24-023', 7.121, 0.57, 94.9, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (2, 'OCW-A24-024', 6.702, 0.14, 93.8, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (3, 'OCW-A24-007', 6.568, 0.28, 98.6, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (4, 'OCW-A24-016', 6.386, 0.14, 92.1, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (5, 'OCW-A24-009', 6.779, 0.28, 98.5, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (6, 'OCW-A24-016', 7.151, 0.70, 94.8, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (7, 'OCW-A24-010', 6.533, 0.28, 98.0, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (8, 'OCW-A24-007', 6.418, 0.71, 99.3, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (9, 'OCW-A24-015', 6.975, 0.71, 98.6, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (10, 'OCW-A24-012', 6.765, 0.00, 92.1, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (11, 'OCW-A24-010', 7.080, 0.70, 96.4, 'PASS');
INSERT INTO assay_abkg3iuk4u VALUES (12, 'OCW-A24-002', 6.402, 0.28, 93.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abkg3iuk4u WHERE status = 'PASS';
