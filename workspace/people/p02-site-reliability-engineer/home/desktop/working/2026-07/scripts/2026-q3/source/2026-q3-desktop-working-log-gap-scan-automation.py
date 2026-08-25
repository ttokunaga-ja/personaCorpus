"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "log_gap"
SPINE_IDS = ['org.ntc', 'team.platform-sre', 'rel.oncall.2026-07-13', 'period.2026-q3', 'term.no-kio-claim']
FINGERPRINT = "ba3d2c4f20c69898"

def reconcile_export_gap(minutes: int, reconciled_by: str) -> dict[str, object]:
    return {'gap_minutes': minutes, 'reconciled_by': reconciled_by, 'status': 'reconciled'}
