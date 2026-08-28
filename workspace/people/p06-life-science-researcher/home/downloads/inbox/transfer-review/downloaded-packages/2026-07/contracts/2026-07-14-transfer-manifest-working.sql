-- Orchid Cellworks synthetic laboratory table for OCW-LAB-YR44CR6U.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abyr44cr6u (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abyr44cr6u VALUES (1, 'OCW-A24-022', 7.045, 0.71, 98.4, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (2, 'OCW-A24-019', 6.975, 0.70, 96.0, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (3, 'OCW-A24-007', 6.581, 0.56, 97.8, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (4, 'OCW-A24-013', 6.814, 0.28, 92.4, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (5, 'OCW-A24-021', 6.575, 0.42, 99.7, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (6, 'OCW-A24-014', 6.792, 0.56, 97.1, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (7, 'OCW-A24-021', 6.616, 0.56, 99.3, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (8, 'OCW-A24-003', 6.522, 0.71, 99.4, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (9, 'OCW-A24-007', 7.164, 0.28, 99.8, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (10, 'OCW-A24-013', 6.564, 0.57, 96.9, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (11, 'OCW-A24-017', 6.750, 0.42, 98.4, 'PASS');
INSERT INTO assay_abyr44cr6u VALUES (12, 'OCW-A24-009', 6.731, 0.71, 93.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abyr44cr6u WHERE status = 'PASS';
