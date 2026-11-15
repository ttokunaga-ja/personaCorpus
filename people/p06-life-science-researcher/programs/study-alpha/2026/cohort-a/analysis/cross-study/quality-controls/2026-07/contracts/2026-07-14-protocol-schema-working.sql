-- Orchid Cellworks synthetic laboratory table for OCW-LAB-SK6HL7UU.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_absk6hl7uu (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_absk6hl7uu VALUES (1, 'OCW-A24-016', 6.575, 0.42, 96.5, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (2, 'OCW-A24-018', 6.513, 0.14, 92.6, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (3, 'OCW-A24-024', 6.520, 0.00, 92.5, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (4, 'OCW-A24-005', 6.428, 0.28, 98.5, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (5, 'OCW-A24-008', 6.772, 0.14, 99.8, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (6, 'OCW-A24-006', 6.660, 0.00, 92.2, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (7, 'OCW-A24-012', 6.843, 0.57, 97.8, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (8, 'OCW-A24-001', 6.507, 0.28, 98.8, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (9, 'OCW-A24-005', 6.765, 0.00, 95.1, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (10, 'OCW-A24-021', 6.814, 0.28, 97.7, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (11, 'OCW-A24-007', 6.808, 0.57, 97.7, 'PASS');
INSERT INTO assay_absk6hl7uu VALUES (12, 'OCW-A24-004', 6.482, 0.70, 99.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_absk6hl7uu WHERE status = 'PASS';
