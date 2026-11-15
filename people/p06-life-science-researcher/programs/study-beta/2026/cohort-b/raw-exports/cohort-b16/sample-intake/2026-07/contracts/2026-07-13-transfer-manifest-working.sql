-- Orchid Cellworks synthetic laboratory table for OCW-LAB-YS43W5R6.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abys43w5r6 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abys43w5r6 VALUES (1, 'OCW-B16-004', 6.367, 0.28, 97.7, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (2, 'OCW-B16-010', 6.623, 0.70, 93.0, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (3, 'OCW-B16-009', 6.779, 0.28, 96.3, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (4, 'OCW-B16-015', 7.163, 0.43, 96.2, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (5, 'OCW-B16-003', 6.424, 0.57, 93.8, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (6, 'OCW-B16-002', 6.940, 0.00, 98.9, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (7, 'OCW-B16-012', 6.693, 0.70, 97.5, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (8, 'OCW-B16-016', 7.010, 0.70, 93.4, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (9, 'OCW-B16-015', 7.094, 0.28, 96.6, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (10, 'OCW-B16-008', 6.933, 0.56, 99.7, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (11, 'OCW-B16-001', 6.801, 0.71, 95.2, 'PASS');
INSERT INTO assay_abys43w5r6 VALUES (12, 'OCW-B16-007', 7.045, 0.71, 95.0, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abys43w5r6 WHERE status = 'PASS';
