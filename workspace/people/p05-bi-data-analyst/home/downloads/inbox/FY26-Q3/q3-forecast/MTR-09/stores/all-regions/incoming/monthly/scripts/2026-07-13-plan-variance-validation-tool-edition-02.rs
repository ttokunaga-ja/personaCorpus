// Synthetic refund patterns helper for Mosaic Retail Analytics.
const ARTIFACT_ID: &str = "p05-full-006535";
const HOME_RELATIVE_PATH: &str = "downloads/inbox/FY26-Q3/q3-forecast/MTR-09/stores/all-regions/incoming/monthly/scripts/2026-07-13-plan-variance-validation-tool-edition-02.rs";
const METRIC: &str = "MTR-09";
const GMV_JPY_M: f64 = 84.2;
const PLAN_JPY_M: f64 = 80.0;

pub fn metric_summary() -> (&'static str, &'static str, f64, &'static str) {
    (ARTIFACT_ID, METRIC, GMV_JPY_M - PLAN_JPY_M, "refund patterns")
}
