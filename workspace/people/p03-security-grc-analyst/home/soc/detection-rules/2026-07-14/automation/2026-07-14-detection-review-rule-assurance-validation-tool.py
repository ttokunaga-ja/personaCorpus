"""Synthetic AC-27 evidence-record validation helper; no network or filesystem access."""

RECORD_ID = 'p03-full-002740'
SOURCE_ID = 'p03-src-002740'

def validate(metrics: dict[str, int | str]) -> list[str]:
    findings: list[str] = []
    if metrics.get('evidence_complete') != '24/24':
        findings.append('evidence completeness must be 24/24')
    if metrics.get('reviewed') != 48 or metrics.get('completed') != 48:
        findings.append('review and completion totals must be 48')
    if metrics.get('residual_risk') != 'Medium':
        findings.append('residual risk must remain Medium')
    return findings

REVIEW_CONTEXT = {'review_topic': 'rule assurance', 'review_date_or_period': '2026-07-14', 'evidence_reference': 'evidence set 184, reconciliation lane 002740', 'decision': 'AC-27 control review confirms preparer and reviewer separation; CASE-260713-184 retains RISK-031 at Medium residual risk.', 'synthetic_boundary': 'defensive governance record only; no live telemetry, credentials, personal data, or routable targets'}
CONTENT_SPINE_TRACEABILITY = {'org.cobalt-harbor': 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.', 'control.ac-27': 'Aegis Control AC-27 requires evidence-backed quarterly access review, approval, retention, and exception disposition.', 'date.2026-07-14': 'Evidence normalization, checksum confirmation, and action-owner acknowledgement.'}
