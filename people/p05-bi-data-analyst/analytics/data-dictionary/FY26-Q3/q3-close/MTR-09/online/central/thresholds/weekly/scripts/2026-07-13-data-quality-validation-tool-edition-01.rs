// Synthetic plan variance helper for Mosaic Retail Analytics.
const ARTIFACT_ID: &str = "p05-full-001940";
const HOME_RELATIVE_PATH: &str = "analytics/data-dictionary/FY26-Q3/q3-close/MTR-09/online/central/thresholds/weekly/scripts/2026-07-13-data-quality-validation-tool-edition-01.rs";
const METRIC: &str = "MTR-09";
const GMV_JPY_M: f64 = 84.2;
const PLAN_JPY_M: f64 = 80.0;

pub fn metric_summary() -> (&'static str, &'static str, f64, &'static str) {
    (ARTIFACT_ID, METRIC, GMV_JPY_M - PLAN_JPY_M, "plan variance")
}
