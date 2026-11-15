"""Synthetic AC-27 validation helper for p03-full-008294; never connects to a system."""
from __future__ import annotations

ORGANIZATION = "Cobalt Harbor Systems"
WORKING_PAPER_CONTEXT = {"stage": "governance-planning", "evidence_card": "EV-184-15", "review_lane": "PRIV-39", "verification_pass": 1, "working_paper_sequence": 8294, "spine_ids": ['org.cobalt-harbor', 'control.approval-dual', 'rel.ac27.evidence-184', 'period.2026-q3', 'term.no-kio-claim'], "rationale": 'Quarterly governance planning preserves the completed July review and schedules evidence retention follow-through.', "observation": 'Approval separation records preparer and reviewer roles for the synthetic exception disposition.', "canonical_values": ['AC-27 requires preparer and reviewer role separation for each exception disposition.']}
EXPECTED = {"control_id": "AC-27", "case_id": "CASE-260713-184", "risk_id": "RISK-031", "evidence_set": 184, "reviewed": 48, "timely_attestations": 47, "residual_risk": "Medium"}

def validate(record: dict[str, object]) -> list[str]:
    return [key for key, value in EXPECTED.items() if record.get(key) != value]

if __name__ == "__main__":
    print("Synthetic validation helper; provide records through an external safe harness.")
