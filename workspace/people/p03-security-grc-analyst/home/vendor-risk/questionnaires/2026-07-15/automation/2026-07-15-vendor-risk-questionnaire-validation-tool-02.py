"""Synthetic defensive validation for VRF-006490; no live connections.
Content spine trace: org.cobalt-harbor: Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | risk.risk-031: RISK-031: incomplete quarterly privileged-access review evidence could prevent AC-27 operating-effectiveness confirmation; inherent High and residual Medium after bounded remediation. | date.2026-07-15: Control owner validates the remediation plan and retention disposition."""
from __future__ import annotations

RECORD_ID = "VRF-006490"

def validate_supplier_record(record: dict[str, object]) -> list[str]:
    required = {"control": "AC-27", "case": "CASE-260713-184", "risk": "RISK-031", "evidence_set": 184}
    return [f"{key} must equal {expected!r}" for key, expected in required.items() if record.get(key) != expected]

def normalize_completion(timely: int, compensating: int) -> float:
    return (timely + compensating) / 48
