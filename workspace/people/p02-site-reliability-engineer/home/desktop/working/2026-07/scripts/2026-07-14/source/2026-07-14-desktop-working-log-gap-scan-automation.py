"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "log_gap"
SPINE_IDS = ['org.ntc', 'inc.inc-260713', 'rel.oncall.2026-07-13', 'date.2026-07-14', 'term.no-kio-claim']
FINGERPRINT = "b8be6376138364e4"

def reconcile_export_gap(minutes: int, reconciled_by: str) -> dict[str, object]:
    return {'gap_minutes': minutes, 'reconciled_by': reconciled_by, 'status': 'reconciled'}
