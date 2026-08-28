-- Orchid Cellworks synthetic laboratory table for OCW-LAB-EUWKAS2T.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abeuwkas2t (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abeuwkas2t VALUES (1, 'OCW-A24-019', 6.780, 0.43, 99.7, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (2, 'OCW-A24-018', 6.668, 0.57, 93.9, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (3, 'OCW-A24-009', 6.562, 0.14, 95.5, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (4, 'OCW-A24-010', 6.695, 0.00, 98.8, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (5, 'OCW-A24-012', 6.687, 0.56, 92.2, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (6, 'OCW-A24-011', 6.766, 0.71, 95.5, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (7, 'OCW-A24-024', 6.540, 0.42, 92.8, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (8, 'OCW-A24-021', 6.668, 0.57, 92.9, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (9, 'OCW-A24-007', 7.198, 0.43, 96.8, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (10, 'OCW-A24-005', 6.396, 0.43, 95.4, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (11, 'OCW-A24-013', 7.017, 0.14, 100.0, 'PASS');
INSERT INTO assay_abeuwkas2t VALUES (12, 'OCW-A24-013', 7.087, 0.14, 96.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abeuwkas2t WHERE status = 'PASS';
