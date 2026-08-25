"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "log_gap"
SPINE_IDS = ['org.ntc', 'term.mitigation', 'rel.inc260713.timeline', 'date.2026-07-13', 'term.no-kio-claim']
FINGERPRINT = "efa3066a59138bc0"

def reconcile_export_gap(minutes: int, reconciled_by: str) -> dict[str, object]:
    return {'gap_minutes': minutes, 'reconciled_by': reconciled_by, 'status': 'reconciled'}
