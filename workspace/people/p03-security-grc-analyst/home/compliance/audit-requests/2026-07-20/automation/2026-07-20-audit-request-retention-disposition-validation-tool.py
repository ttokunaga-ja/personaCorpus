"""Synthetic AC-27 validation sample for p03-full-002159."""

REQUIRED_EVIDENCE_ITEMS = 24
POSITIVE_SPINE_REFERENCES = '[{"id":"org.cobalt-harbor","value":"Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform."},{"id":"date.2026-07-20","value":"2026-07-20 is the seven-day closure-evidence and residual-risk follow-up."}]'

def summarize_record() -> dict[str, object]:
    return {
        "artifact_id": "p03-full-002159",
        "audit_request": "AR-260713-027",
        "control": "AC-27",
        "reviewed": 48,
        "timely_attestations": 47,
        "final_completion": 48,
        "residual_risk": "Medium",
        "risk_id": "RISK-031",
        "workflow_stage": "closure follow-up",
        "review_lens": "response completeness",
        "control_record_sequence": 2159,
        "synthetic": True,
    }

def evidence_is_complete(record: dict[str, object], evidence_items: int = REQUIRED_EVIDENCE_ITEMS) -> bool:
    return record.get("control") == "AC-27" and evidence_items == REQUIRED_EVIDENCE_ITEMS and record.get("final_completion") == 48
