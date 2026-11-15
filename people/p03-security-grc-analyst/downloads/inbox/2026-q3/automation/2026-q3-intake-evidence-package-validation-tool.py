# spine_traceability: org.cobalt-harbor=Cobalt Harbor Systems | period.2026-q3=2026-Q3 governance planning period 2026-07-01 through 2026-09-30
"""Synthetic AC-27 intake validator; no network or filesystem operations."""

RECORD = {"artifact_id": "p03-full-008990", "audit_request": "AR-260713-027", "case": "CASE-260713-184", "control": "AC-27", "disposition": "synthetic intake normalized; bounded review retained", "evidence_record": "EV-184-19", "evidence_set": 184, "network_note": "No live endpoints, raw events, or routable addresses are represented.", "organization": "Cobalt Harbor Systems", "program": "Aegis Assurance", "record_id": "INB-164", "review_date": "2026-07-13", "review_period": "2026-Q3", "risk": "RISK-031", "source_id": "p03-src-008990", "unique_marker": "foundation-b001-164"}

def validate(record: dict[str, object]) -> bool:
    required = {"control", "audit_request", "case", "evidence_set", "risk"}
    return required.issubset(record) and record["control"] == "AC-27" and record["evidence_set"] == 184

if __name__ == "__main__":
    assert validate(RECORD), "synthetic intake record is incomplete: foundation-b001-164"
