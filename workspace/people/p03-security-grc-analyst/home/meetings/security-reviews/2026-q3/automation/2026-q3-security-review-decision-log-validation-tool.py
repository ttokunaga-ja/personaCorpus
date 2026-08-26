"""Defensive validator metadata for a synthetic AC-27 review record."""
REVIEW = {'artifact_id': 'p03-full-006822', 'source_id': 'p03-src-006822', 'organization': 'Cobalt Harbor Systems', 'review_date': '2026-Q3', 'subject': 'Decision Log', 'control': 'AC-27', 'case': 'CASE-260713-184', 'evidence_set': 184, 'risk': 'RISK-031', 'docket': 'GRC-REV-006822', 'agenda': 'record DEC-260713-004 retention disposition with preparer-reviewer separation', 'decision_context': 'close checksum reconciliation evidence', 'action_context': 'EXC-260713-006 reviewer attestation', 'spine_ids': 'org.cobalt-harbor, period.2026-q3, rel.case.timeline, term.no-kio-claim', 'spine_refs': 'org.cobalt-harbor, period.2026-q3, rel.case.timeline, term.no-kio-claim'}
def validate_review(record: dict[str, object]) -> list[str]:
    findings: list[str] = []
    if record.get("control") != "AC-27": findings.append("control identifier mismatch")
    if record.get("evidence_set") != 184: findings.append("evidence-set mismatch")
    return findings
