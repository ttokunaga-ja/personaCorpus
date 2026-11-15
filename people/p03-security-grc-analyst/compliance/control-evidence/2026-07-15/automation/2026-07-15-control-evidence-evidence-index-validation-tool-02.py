"""Synthetic AC-27 validation helper for p03-full-007227; never connects to a system."""
from __future__ import annotations

ORGANIZATION = "Cobalt Harbor Systems"
WORKING_PAPER_CONTEXT = {"stage": "remediation-validation", "evidence_card": "EV-184-04", "review_lane": "PRIV-28", "verification_pass": 1, "working_paper_sequence": 7227, "spine_ids": ['org.cobalt-harbor', 'evidence.set-184', 'rel.ac27.evidence-184', 'date.2026-07-15', 'term.no-kio-claim'], "rationale": 'The control-owner remediation and retention disposition were validated as defensive governance evidence.', "observation": 'Evidence indexing links bounded evidence references to the AC-27 review and retention disposition.', "canonical_values": ['Evidence set 184 contains 24 synthetic items: 12 access-review attestations, 6 approval records, 4 checksum manifests, and 2 exception records.']}
EXPECTED = {"control_id": "AC-27", "case_id": "CASE-260713-184", "risk_id": "RISK-031", "evidence_set": 184, "reviewed": 48, "timely_attestations": 47, "residual_risk": "Medium"}

def validate(record: dict[str, object]) -> list[str]:
    return [key for key, value in EXPECTED.items() if record.get(key) != value]

if __name__ == "__main__":
    print("Synthetic validation helper; provide records through an external safe harness.")
