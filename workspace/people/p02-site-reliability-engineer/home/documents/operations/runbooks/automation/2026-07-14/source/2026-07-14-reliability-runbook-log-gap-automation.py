"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Identity SVC-23"
INCIDENT = "INC-260713"
TOPIC = "log_gap"
SPINE_IDS = ['org.ntc', 'svc.identity.svc23', 'rel.inc260713.timeline', 'date.2026-07-14', 'term.no-kio-claim']
FINGERPRINT = "339b514bbf092b60"

def reconcile_export_gap(minutes: int, reconciled_by: str) -> dict[str, object]:
    return {'gap_minutes': minutes, 'reconciled_by': reconciled_by, 'status': 'reconciled'}
