-- Orchid Cellworks synthetic laboratory table for OCW-LAB-FM7LOGPW.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abfm7logpw (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abfm7logpw VALUES (1, 'OCW-B16-008', 6.542, 0.28, 98.1, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (2, 'OCW-B16-008', 6.975, 0.70, 97.5, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (3, 'OCW-B16-013', 6.975, 0.71, 99.7, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (4, 'OCW-B16-014', 6.444, 0.14, 93.0, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (5, 'OCW-B16-006', 6.975, 0.71, 93.3, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (6, 'OCW-B16-002', 6.592, 0.71, 93.6, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (7, 'OCW-B16-010', 6.717, 0.28, 92.4, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (8, 'OCW-B16-008', 6.996, 0.42, 95.9, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (9, 'OCW-B16-006', 6.399, 0.42, 99.5, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (10, 'OCW-B16-005', 7.256, 0.70, 94.2, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (11, 'OCW-B16-001', 7.122, 0.14, 92.1, 'PASS');
INSERT INTO assay_abfm7logpw VALUES (12, 'OCW-B16-008', 7.129, 0.43, 96.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abfm7logpw WHERE status = 'PASS';
