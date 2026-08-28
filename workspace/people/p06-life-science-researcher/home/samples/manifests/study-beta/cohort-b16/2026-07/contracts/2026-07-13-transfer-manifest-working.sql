-- Orchid Cellworks synthetic laboratory table for OCW-LAB-I7RVDMXB.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abi7rvdmxb (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abi7rvdmxb VALUES (1, 'OCW-B16-003', 6.450, 0.00, 92.7, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (2, 'OCW-B16-012', 6.668, 0.57, 99.5, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (3, 'OCW-B16-008', 6.421, 0.14, 96.6, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (4, 'OCW-B16-012', 7.024, 0.28, 93.3, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (5, 'OCW-B16-009', 7.094, 0.28, 97.5, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (6, 'OCW-B16-006', 6.660, 0.00, 96.3, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (7, 'OCW-B16-012', 6.715, 0.42, 93.2, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (8, 'OCW-B16-003', 7.192, 0.14, 95.2, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (9, 'OCW-B16-002', 6.989, 0.43, 93.0, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (10, 'OCW-B16-013', 6.728, 0.70, 99.7, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (11, 'OCW-B16-010', 6.933, 0.14, 94.9, 'PASS');
INSERT INTO assay_abi7rvdmxb VALUES (12, 'OCW-B16-009', 7.010, 0.70, 97.5, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abi7rvdmxb WHERE status = 'PASS';
