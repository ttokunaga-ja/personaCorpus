"""Cobalt Harbor Systems defensive validation for IR-008257; synthetic evidence only.
Content spine references: org.cobalt-harbor = Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform; evidence.ev-184-07 = EV-184-07, the Sentinel Export minimization statement reporting only synthetic event categories and counts; period.2026-q3 = 2026-Q3 governance planning period from 2026-07-01 through 2026-09-30
"""
REQUIRED = {"case": "CASE-260713-184", "control": "AC-27", "risk": "RISK-031", "evidence_set": 184}

def validate_record(record: dict) -> list[str]:
    findings = [key for key, value in REQUIRED.items() if record.get(key) != value]
    if record.get("completed") != 48 or record.get("residual_risk") != "Medium":
        findings.append("completion_or_residual_rating")
    return findings
