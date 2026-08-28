-- Orchid Cellworks synthetic laboratory table for OCW-LAB-ZM4SX3FV.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abzm4sx3fv (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abzm4sx3fv VALUES (1, 'OCW-B16-007', 6.877, 0.57, 95.2, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (2, 'OCW-B16-003', 6.412, 0.70, 92.7, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (3, 'OCW-B16-016', 6.412, 0.70, 93.0, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (4, 'OCW-B16-010', 6.688, 0.14, 97.1, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (5, 'OCW-B16-014', 6.758, 0.14, 97.2, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (6, 'OCW-B16-011', 6.640, 0.43, 98.2, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (7, 'OCW-B16-008', 7.199, 0.28, 94.3, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (8, 'OCW-B16-016', 6.653, 0.14, 95.8, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (9, 'OCW-B16-005', 6.764, 0.70, 98.0, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (10, 'OCW-B16-005', 7.149, 0.71, 98.5, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (11, 'OCW-B16-003', 6.564, 0.57, 96.5, 'PASS');
INSERT INTO assay_abzm4sx3fv VALUES (12, 'OCW-B16-002', 6.731, 0.71, 96.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abzm4sx3fv WHERE status = 'PASS';
