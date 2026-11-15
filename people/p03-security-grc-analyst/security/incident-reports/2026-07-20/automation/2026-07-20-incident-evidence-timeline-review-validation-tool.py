"""Cobalt Harbor Systems defensive validation for IR-004433; synthetic evidence only.
Content spine references: org.cobalt-harbor = Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform; date.2026-07-20 = 2026-07-20, seven-day follow-up for closure evidence and residual-risk status
"""
REQUIRED = {"case": "CASE-260713-184", "control": "AC-27", "risk": "RISK-031", "evidence_set": 184}

def validate_record(record: dict) -> list[str]:
    findings = [key for key, value in REQUIRED.items() if record.get(key) != value]
    if record.get("completed") != 48 or record.get("residual_risk") != "Medium":
        findings.append("completion_or_residual_rating")
    return findings
