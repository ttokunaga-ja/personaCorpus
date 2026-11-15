"""Cobalt Harbor Systems defensive validation for IR-008034; synthetic evidence only.
Content spine references: org.cobalt-harbor = Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform; case.case-260713-184 = CASE-260713-184, the synthetic evidence intake case; it opens and closes on 2026-07-13; date.2026-07-15 = 2026-07-15, control-owner remediation-plan and retention-disposition validation
"""
REQUIRED = {"case": "CASE-260713-184", "control": "AC-27", "risk": "RISK-031", "evidence_set": 184}

def validate_record(record: dict) -> list[str]:
    findings = [key for key, value in REQUIRED.items() if record.get(key) != value]
    if record.get("completed") != 48 or record.get("residual_risk") != "Medium":
        findings.append("completion_or_residual_rating")
    return findings
