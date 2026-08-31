"""Synthetic Mosaic FY26 Q3 metric validation."""

ARTIFACT_ID = "p05-full-011238"
HOME_RELATIVE_PATH = "exports/warehouse/FY26-Q3/week-2026-07-07/MTR-09/stores/west/extract/stakeholder/scripts/2026-07-13-refunds-validation-tool-edition-03.py"
METRIC = "MTR-09"
GMV_JPY_M = 84.2
PLAN_JPY_M = 80.0

def validate_metric() -> dict[str, float | str]:
    variance = round(GMV_JPY_M - PLAN_JPY_M, 1)
    return {"artifact_id": ARTIFACT_ID, "metric": METRIC, "gmv_jpy_m": GMV_JPY_M, "plan_jpy_m": PLAN_JPY_M, "variance_jpy_m": variance}

if __name__ == "__main__":
    print(validate_metric())
