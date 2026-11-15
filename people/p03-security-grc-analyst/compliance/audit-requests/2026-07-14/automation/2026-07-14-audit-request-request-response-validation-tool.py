"""Synthetic AC-27 validation sample for p03-full-002112."""

REQUIRED_EVIDENCE_ITEMS = 24
POSITIVE_SPINE_REFERENCES = '[{"id":"org.cobalt-harbor","value":"Cobalt Harbor Systems (CHS), a fictional managed logistics and harbor-operations platform."},{"id":"audit.ar-260713-027","value":"AR-260713-027 is the fictional AC-27 audit request for bounded review material."},{"id":"date.2026-07-14","value":"2026-07-14 is reserved for evidence normalization, checksum confirmation, and action-owner acknowledgement."}]'

def summarize_record() -> dict[str, object]:
    return {
        "artifact_id": "p03-full-002112",
        "audit_request": "AR-260713-027",
        "control": "AC-27",
        "reviewed": 48,
        "timely_attestations": 47,
        "final_completion": 48,
        "residual_risk": "Medium",
        "risk_id": "RISK-031",
        "workflow_stage": "intake triage",
        "review_lens": "retention traceability",
        "control_record_sequence": 2112,
        "synthetic": True,
    }

def evidence_is_complete(record: dict[str, object], evidence_items: int = REQUIRED_EVIDENCE_ITEMS) -> bool:
    return record.get("control") == "AC-27" and evidence_items == REQUIRED_EVIDENCE_ITEMS and record.get("final_completion") == 48
