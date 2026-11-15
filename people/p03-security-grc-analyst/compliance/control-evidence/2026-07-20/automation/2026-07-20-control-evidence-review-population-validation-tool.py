"""Synthetic AC-27 validation helper for p03-full-001705; never connects to a system."""
from __future__ import annotations

ORGANIZATION = "Cobalt Harbor Systems"
WORKING_PAPER_CONTEXT = {"stage": "follow-up", "evidence_card": "EV-184-02", "review_lane": "PRIV-26", "verification_pass": 1, "working_paper_sequence": 1705, "spine_ids": ['org.cobalt-harbor', 'evidence.ev-184-19', 'rel.ac27.evidence-184', 'date.2026-07-20', 'term.no-kio-claim'], "rationale": 'Seven-day closure follow-up confirmed the residual-risk status and checksum reconciliation.', "observation": 'Population coverage records 48 synthetic privileged roles, with 47 timely attestations before compensating review.'}
EXPECTED = {"control_id": "AC-27", "case_id": "CASE-260713-184", "risk_id": "RISK-031", "evidence_set": 184, "reviewed": 48, "timely_attestations": 47, "residual_risk": "Medium"}

def validate(record: dict[str, object]) -> list[str]:
    return [key for key, value in EXPECTED.items() if record.get(key) != value]

if __name__ == "__main__":
    print("Synthetic validation helper; provide records through an external safe harness.")
