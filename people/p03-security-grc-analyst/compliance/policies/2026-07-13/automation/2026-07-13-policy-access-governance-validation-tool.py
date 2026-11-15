"""Synthetic Cobalt Harbor Systems AC-27 policy evidence normalization helper; no external access."""
ARTIFACT_ID = 'p03-full-005568'

def normalize_review_record(record: dict) -> dict:
    """Return a defensive, bounded AC-27 evidence summary."""
    return {
        'artifact_id': ARTIFACT_ID, 'control': 'Aegis Control AC-27',
        'case': 'CASE-260713-184', 'risk': 'RISK-031', 'residual': 'Medium',
        'evidence_set': 184, 'verified_items': 24, 'reviewed_roles': 48,
        'final_completion': 48, 'retention_months': 18,
        'record_sequence': 185, 'input_label': str(record.get('label', 'synthetic')),
    }

# Content-spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | control.ac-27 = Aegis Control AC-27 requires evidence-backed quarterly access review, approval, retention, and exception disposition. | date.2026-07-13 = Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC.
