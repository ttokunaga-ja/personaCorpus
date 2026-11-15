// Synthetic GMV performance helper for Mosaic Retail Analytics.
const ARTIFACT_ID: &str = "p05-full-009048";
const HOME_RELATIVE_PATH: &str = "forecasts/scenarios/FY26-Q3/week-2026-07-07/MTR-09/all-channels/all-regions/base-case/definition/scripts/2026-07-13-refunds-validation-tool-edition-01.rs";
const METRIC: &str = "MTR-09";
const GMV_JPY_M: f64 = 84.2;
const PLAN_JPY_M: f64 = 80.0;

pub fn metric_summary() -> (&'static str, &'static str, f64, &'static str) {
    (ARTIFACT_ID, METRIC, GMV_JPY_M - PLAN_JPY_M, "GMV performance")
}
