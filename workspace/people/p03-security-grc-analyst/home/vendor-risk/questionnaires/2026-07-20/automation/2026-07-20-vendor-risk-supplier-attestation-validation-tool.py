"""Synthetic defensive validation for VRF-002336; no live connections.
Content spine trace: org.cobalt-harbor: Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | action.act-260713-013: Confirm supplier subprocessor attestation mapping; due 2026-07-31. | date.2026-07-20: Seven-day follow-up validates closure evidence and residual-risk status."""
from __future__ import annotations

RECORD_ID = "VRF-002336"

def validate_supplier_record(record: dict[str, object]) -> list[str]:
    required = {"control": "AC-27", "case": "CASE-260713-184", "risk": "RISK-031", "evidence_set": 184}
    return [f"{key} must equal {expected!r}" for key, expected in required.items() if record.get(key) != expected]

def normalize_completion(timely: int, compensating: int) -> float:
    return (timely + compensating) / 48
