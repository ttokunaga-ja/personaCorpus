"""Defensive validator metadata for a synthetic AC-27 review record."""
REVIEW = {'artifact_id': 'p03-full-002927', 'source_id': 'p03-src-002927', 'organization': 'Cobalt Harbor Systems', 'review_date': '2026-07-20', 'subject': 'Assurance Briefing', 'control': 'AC-27', 'case': 'CASE-260713-184', 'evidence_set': 184, 'risk': 'RISK-031', 'docket': 'GRC-REV-002927', 'agenda': 'prepare a bounded AC-27 assurance readout with numerator, denominator, and residual-risk rationale', 'decision_context': 'queue the seven-day closure validation', 'action_context': 'EXC-260713-006 reviewer attestation', 'spine_ids': 'org.cobalt-harbor, action.act-260713-012, rel.case.timeline, date.2026-07-20, term.no-kio-claim', 'spine_refs': 'org.cobalt-harbor, action.act-260713-012 (ACT-260713-012), rel.case.timeline, date.2026-07-20, term.no-kio-claim'}
def validate_review(record: dict[str, object]) -> list[str]:
    findings: list[str] = []
    if record.get("control") != "AC-27": findings.append("control identifier mismatch")
    if record.get("evidence_set") != 184: findings.append("evidence-set mismatch")
    return findings
