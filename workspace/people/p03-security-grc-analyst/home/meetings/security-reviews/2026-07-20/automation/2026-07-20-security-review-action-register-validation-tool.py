"""Defensive validator metadata for a synthetic AC-27 review record."""
REVIEW = {'artifact_id': 'p03-full-002961', 'source_id': 'p03-src-002961', 'organization': 'Cobalt Harbor Systems', 'review_date': '2026-07-20', 'subject': 'Action Register', 'control': 'AC-27', 'case': 'CASE-260713-184', 'evidence_set': 184, 'risk': 'RISK-031', 'docket': 'GRC-REV-002961', 'agenda': 'confirm ACT-260713-012 checksum-reconciliation follow-up and its 2026-07-20 closure evidence', 'decision_context': 'confirm the compensating-review linkage', 'action_context': 'AR-260713-027 response packet', 'spine_ids': 'org.cobalt-harbor, period.2026-q3, rel.case.timeline, date.2026-07-20, term.no-kio-claim', 'spine_refs': 'org.cobalt-harbor, period.2026-q3, rel.case.timeline, date.2026-07-20, term.no-kio-claim'}
def validate_review(record: dict[str, object]) -> list[str]:
    findings: list[str] = []
    if record.get("control") != "AC-27": findings.append("control identifier mismatch")
    if record.get("evidence_set") != 184: findings.append("evidence-set mismatch")
    return findings
