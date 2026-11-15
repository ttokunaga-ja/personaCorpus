"""Synthetic AC-27 validation helper for p03-full-006197; never connects to a system."""
from __future__ import annotations

ORGANIZATION = "Cobalt Harbor Systems"
WORKING_PAPER_CONTEXT = {"stage": "intake-and-closure", "evidence_card": "EV-184-06", "review_lane": "PRIV-06", "verification_pass": 5, "working_paper_sequence": 6197, "spine_ids": ['org.cobalt-harbor', 'control.approval-dual', 'rel.ac27.evidence-184', 'date.2026-07-13', 'term.no-kio-claim'], "rationale": 'The bounded intake and AC-27 closure record were checked against the same synthetic evidence chain.', "observation": 'Population coverage records 48 synthetic privileged roles, with 47 timely attestations before compensating review.', "canonical_values": ['AC-27 requires preparer and reviewer role separation for each exception disposition.']}
EXPECTED = {"control_id": "AC-27", "case_id": "CASE-260713-184", "risk_id": "RISK-031", "evidence_set": 184, "reviewed": 48, "timely_attestations": 47, "residual_risk": "Medium"}

def validate(record: dict[str, object]) -> list[str]:
    return [key for key, value in EXPECTED.items() if record.get(key) != value]

if __name__ == "__main__":
    print("Synthetic validation helper; provide records through an external safe harness.")
