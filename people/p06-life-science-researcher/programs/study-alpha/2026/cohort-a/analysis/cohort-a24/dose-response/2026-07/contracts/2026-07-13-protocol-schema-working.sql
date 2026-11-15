-- Orchid Cellworks synthetic laboratory table for OCW-LAB-IOQPB5OB.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abioqpb5ob (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abioqpb5ob VALUES (1, 'OCW-A24-016', 7.220, 0.00, 95.5, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (2, 'OCW-A24-009', 6.540, 0.42, 99.7, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (3, 'OCW-A24-010', 6.421, 0.14, 92.3, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (4, 'OCW-A24-023', 6.618, 0.14, 94.9, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (5, 'OCW-A24-001', 6.529, 0.57, 98.0, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (6, 'OCW-A24-009', 6.491, 0.14, 99.9, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (7, 'OCW-A24-014', 6.658, 0.70, 93.4, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (8, 'OCW-A24-020', 7.178, 0.14, 92.1, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (9, 'OCW-A24-022', 6.453, 0.71, 95.8, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (10, 'OCW-A24-008', 6.731, 0.71, 99.2, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (11, 'OCW-A24-008', 6.897, 0.56, 95.8, 'PASS');
INSERT INTO assay_abioqpb5ob VALUES (12, 'OCW-A24-012', 6.710, 0.43, 97.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abioqpb5ob WHERE status = 'PASS';
