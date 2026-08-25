"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "log_gap"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.inc260713.timeline', 'date.2026-07-20', 'term.no-kio-claim']
FINGERPRINT = "b0dd62f294e39837"

def reconcile_export_gap(minutes: int, reconciled_by: str) -> dict[str, object]:
    return {'gap_minutes': minutes, 'reconciled_by': reconciled_by, 'status': 'reconciled'}
