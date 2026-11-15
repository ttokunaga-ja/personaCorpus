"""Synthetic AC-27 validation helper for p03-full-001860; never connects to a system."""
from __future__ import annotations

ORGANIZATION = "Cobalt Harbor Systems"
WORKING_PAPER_CONTEXT = {"stage": "intake-and-closure", "evidence_card": "EV-184-13", "review_lane": "PRIV-37", "verification_pass": 2, "working_paper_sequence": 1860, "spine_ids": ['org.cobalt-harbor', 'evidence.set-184', 'rel.ac27.evidence-184', 'date.2026-07-13', 'term.no-kio-claim'], "rationale": 'The bounded intake and AC-27 closure record were checked against the same synthetic evidence chain.', "observation": 'Checksum reconciliation records the 24-item evidence package without retaining source-system content.', "canonical_values": ['Evidence set 184 contains 24 synthetic items: 12 access-review attestations, 6 approval records, 4 checksum manifests, and 2 exception records.']}
EXPECTED = {"control_id": "AC-27", "case_id": "CASE-260713-184", "risk_id": "RISK-031", "evidence_set": 184, "reviewed": 48, "timely_attestations": 47, "residual_risk": "Medium"}

def validate(record: dict[str, object]) -> list[str]:
    return [key for key, value in EXPECTED.items() if record.get(key) != value]

if __name__ == "__main__":
    print("Synthetic validation helper; provide records through an external safe harness.")
