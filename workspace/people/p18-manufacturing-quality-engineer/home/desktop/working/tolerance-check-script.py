"""Validate synthetic ForgeLine dimensions against the ±0.18 mm working criterion."""
from __future__ import annotations

TOLERANCE_MM = 0.18


def within_tolerance(nominal_mm: float, measured_mm: float) -> bool:
    """Return whether a measurement lies within the approved working band."""
    return abs(measured_mm - nominal_mm) <= TOLERANCE_MM


if __name__ == "__main__":
    example = within_tolerance(12.0, 12.12)
    print(f"ForgeLine Product Alpha NCR-026 check: {example}")
