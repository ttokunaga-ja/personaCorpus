"""AC-27 evidence completeness validator for synthetic records.
Context: 2026-07-20-personal-grc-evidence-digest-validation-tool-py-s003735.py | evidence digest | exception disposition | ledger date anchors: 2026-07-20 | positive spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform | team.grc = Security GRC owns the AC-27 evidence register, risk treatment tracking, and audit-request coordination | date.2026-07-20 = Seven-day follow-up validates closure evidence and residual-risk status | For 2026-07-20, this working file prepare a personal evidence note focused on exception disposition; it supports a Medium residual-risk conclusion without asserting live-system access.
"""

def validate(record):
    required = {"control": "AC-27", "case": "CASE-260713-184", "risk": "RISK-031"}
    missing = [key for key, value in required.items() if record.get(key) != value]
    return {"artifact_id": "p03-full-003735", "working_purpose": "prepare a personal evidence note", "conclusion": "For 2026-07-20, this working file prepare a personal evidence note focused on exception disposition; it supports a Medium residual-risk conclusion without asserting live-system access.", "valid": not missing, "missing": missing, "classification": "synthetic defensive control evidence"}
