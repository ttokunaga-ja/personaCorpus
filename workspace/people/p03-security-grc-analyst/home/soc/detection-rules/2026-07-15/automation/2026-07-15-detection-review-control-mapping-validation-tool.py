"""Synthetic AC-27 evidence-record validation helper; no network or filesystem access."""

RECORD_ID = 'p03-full-007335'
SOURCE_ID = 'p03-src-007335'

def validate(metrics: dict[str, int | str]) -> list[str]:
    findings: list[str] = []
    if metrics.get('evidence_complete') != '24/24':
        findings.append('evidence completeness must be 24/24')
    if metrics.get('reviewed') != 48 or metrics.get('completed') != 48:
        findings.append('review and completion totals must be 48')
    if metrics.get('residual_risk') != 'Medium':
        findings.append('residual risk must remain Medium')
    return findings

REVIEW_CONTEXT = {'review_topic': 'control mapping', 'review_date_or_period': '2026-07-15', 'evidence_reference': 'evidence set 184, reconciliation lane 007335', 'decision': 'SOC duty review confirms the minimized Sentinel Export boundary; CASE-260713-184 retains RISK-031 at Medium residual risk.', 'synthetic_boundary': 'defensive governance record only; no live telemetry, credentials, personal data, or routable targets'}
CONTENT_SPINE_TRACEABILITY = {'org.cobalt-harbor': 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.', 'team.soc': 'Security Operations owns bounded SIEM exports, detection review evidence, and incident evidence-chain handoff.', 'date.2026-07-15': 'Control owner validates the remediation plan and retention disposition.'}
