"""Synthetic Orchid Cellworks assay validation for OCW-LAB-ZOP6PMXI."""
from __future__ import annotations

RECORD_CODE = 'OCW-LAB-ZOP6PMXI'
REFERENCE_DATE = "2026-07-13"
PROTOCOL = "OCW-BIO-017"

def accepted(duplicate_cv_percent: float, recovery_percent: float) -> bool:
    """Return the explicit synthetic assay acceptance decision."""
    return duplicate_cv_percent <= 10.0 and 90.0 <= recovery_percent <= 110.0

def summarize(values: list[float]) -> dict[str, float]:
    """Summarize a bounded in-memory measurement list."""
    if not values:
        raise ValueError("values must not be empty")
    return {"count": float(len(values)), "mean": sum(values) / len(values)}

if __name__ == "__main__":
    result = accepted(4.3, 93.7)
    print(f"{RECORD_CODE}:{PROTOCOL}:{result}")
