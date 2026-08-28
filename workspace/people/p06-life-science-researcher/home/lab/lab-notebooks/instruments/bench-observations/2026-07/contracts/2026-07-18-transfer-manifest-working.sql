-- Orchid Cellworks synthetic laboratory table for OCW-LAB-UPV6MAGK.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abupv6magk (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abupv6magk VALUES (1, 'OCW-A24-007', 6.673, 0.28, 95.5, 'PASS');
INSERT INTO assay_abupv6magk VALUES (2, 'OCW-A24-018', 7.129, 0.43, 95.9, 'PASS');
INSERT INTO assay_abupv6magk VALUES (3, 'OCW-A24-021', 6.961, 0.28, 98.5, 'PASS');
INSERT INTO assay_abupv6magk VALUES (4, 'OCW-A24-008', 6.834, 0.70, 95.7, 'PASS');
INSERT INTO assay_abupv6magk VALUES (5, 'OCW-A24-003', 6.564, 0.57, 92.3, 'PASS');
INSERT INTO assay_abupv6magk VALUES (6, 'OCW-A24-020', 6.588, 0.70, 92.7, 'PASS');
INSERT INTO assay_abupv6magk VALUES (7, 'OCW-A24-001', 6.807, 0.14, 93.1, 'PASS');
INSERT INTO assay_abupv6magk VALUES (8, 'OCW-A24-001', 6.658, 0.70, 92.2, 'PASS');
INSERT INTO assay_abupv6magk VALUES (9, 'OCW-A24-022', 7.115, 0.70, 93.5, 'PASS');
INSERT INTO assay_abupv6magk VALUES (10, 'OCW-A24-012', 6.399, 0.42, 92.0, 'PASS');
INSERT INTO assay_abupv6magk VALUES (11, 'OCW-A24-012', 6.453, 0.71, 96.1, 'PASS');
INSERT INTO assay_abupv6magk VALUES (12, 'OCW-A24-013', 6.522, 0.71, 97.4, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abupv6magk WHERE status = 'PASS';
