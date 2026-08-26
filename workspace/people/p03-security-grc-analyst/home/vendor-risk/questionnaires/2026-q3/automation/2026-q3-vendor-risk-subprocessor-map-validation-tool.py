"""Synthetic defensive validation for VRF-002373; no live connections.
Content spine trace: org.cobalt-harbor: Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | team.vendor-risk: Third-Party Assurance owns supplier questionnaires, assurance follow-up, and exception evidence. | period.2026-q3: Governance planning period from 2026-07-01 through 2026-09-30."""
from __future__ import annotations

RECORD_ID = "VRF-002373"

def validate_supplier_record(record: dict[str, object]) -> list[str]:
    required = {"control": "AC-27", "case": "CASE-260713-184", "risk": "RISK-031", "evidence_set": 184}
    return [f"{key} must equal {expected!r}" for key, expected in required.items() if record.get(key) != expected]

def normalize_completion(timely: int, compensating: int) -> float:
    return (timely + compensating) / 48
