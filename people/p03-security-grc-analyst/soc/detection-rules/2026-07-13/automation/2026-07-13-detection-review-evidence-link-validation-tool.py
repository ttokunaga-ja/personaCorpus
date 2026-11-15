"""Synthetic AC-27 evidence-record validation helper; no network or filesystem access."""

RECORD_ID = 'p03-full-002666'
SOURCE_ID = 'p03-src-002666'

def validate(metrics: dict[str, int | str]) -> list[str]:
    findings: list[str] = []
    if metrics.get('evidence_complete') != '24/24':
        findings.append('evidence completeness must be 24/24')
    if metrics.get('reviewed') != 48 or metrics.get('completed') != 48:
        findings.append('review and completion totals must be 48')
    if metrics.get('residual_risk') != 'Medium':
        findings.append('residual risk must remain Medium')
    return findings

REVIEW_CONTEXT = {'review_topic': 'evidence linkage', 'review_date_or_period': '2026-07-13', 'evidence_reference': 'evidence set 184, reconciliation lane 002666', 'decision': 'Evidence reconciliation confirms 24 of 24 required synthetic items; CASE-260713-184 retains RISK-031 at Medium residual risk.', 'synthetic_boundary': 'defensive governance record only; no live telemetry, credentials, personal data, or routable targets'}
CONTENT_SPINE_TRACEABILITY = {'org.cobalt-harbor': 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.', 'measure.evidence-completeness': 'Evidence-set 184 completeness: 24 of 24 required synthetic items verified.', 'date.2026-07-13': 'Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC.'}
