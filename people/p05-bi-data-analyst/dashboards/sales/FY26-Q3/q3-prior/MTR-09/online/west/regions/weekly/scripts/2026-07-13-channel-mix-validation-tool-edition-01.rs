// Synthetic refresh controls helper for Mosaic Retail Analytics.
const ARTIFACT_ID: &str = "p05-full-007696";
const HOME_RELATIVE_PATH: &str = "dashboards/sales/FY26-Q3/q3-prior/MTR-09/online/west/regions/weekly/scripts/2026-07-13-channel-mix-validation-tool-edition-01.rs";
const METRIC: &str = "MTR-09";
const GMV_JPY_M: f64 = 84.2;
const PLAN_JPY_M: f64 = 80.0;

pub fn metric_summary() -> (&'static str, &'static str, f64, &'static str) {
    (ARTIFACT_ID, METRIC, GMV_JPY_M - PLAN_JPY_M, "refresh controls")
}
