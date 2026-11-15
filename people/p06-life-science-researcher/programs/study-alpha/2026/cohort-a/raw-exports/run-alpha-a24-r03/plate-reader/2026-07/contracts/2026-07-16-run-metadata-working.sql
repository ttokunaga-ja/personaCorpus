-- Orchid Cellworks synthetic laboratory table for OCW-LAB-AABMG7PY.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abaabmg7py (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abaabmg7py VALUES (1, 'OCW-A24-005', 6.786, 0.28, 96.3, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (2, 'OCW-A24-018', 6.640, 0.43, 98.4, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (3, 'OCW-A24-012', 6.533, 0.28, 96.1, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (4, 'OCW-A24-012', 7.114, 0.71, 93.2, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (5, 'OCW-A24-011', 6.758, 0.14, 98.8, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (6, 'OCW-A24-016', 6.842, 0.14, 96.0, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (7, 'OCW-A24-014', 6.799, 0.70, 93.2, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (8, 'OCW-A24-007', 6.581, 0.56, 96.6, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (9, 'OCW-A24-008', 6.940, 0.00, 95.1, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (10, 'OCW-A24-010', 6.625, 0.00, 95.9, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (11, 'OCW-A24-018', 7.115, 0.70, 98.4, 'PASS');
INSERT INTO assay_abaabmg7py VALUES (12, 'OCW-A24-008', 6.498, 0.28, 96.4, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abaabmg7py WHERE status = 'PASS';
