"""Synthetic Cobalt Harbor Systems AC-27 policy evidence normalization helper; no external access."""
ARTIFACT_ID = 'p03-full-001562'

def normalize_review_record(record: dict) -> dict:
    """Return a defensive, bounded AC-27 evidence summary."""
    return {
        'artifact_id': ARTIFACT_ID, 'control': 'Aegis Control AC-27',
        'case': 'CASE-260713-184', 'risk': 'RISK-031', 'residual': 'Medium',
        'evidence_set': 184, 'verified_items': 24, 'reviewed_roles': 48,
        'final_completion': 48, 'retention_months': 18,
        'record_sequence': 61, 'input_label': str(record.get('label', 'synthetic')),
    }

# Content-spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | decision.dec-260713-004 = Retain evidence set 184 under the synthetic GRC retention schedule; no live-source data is included. | date.2026-07-20 = Seven-day follow-up validates closure evidence and residual-risk status.
