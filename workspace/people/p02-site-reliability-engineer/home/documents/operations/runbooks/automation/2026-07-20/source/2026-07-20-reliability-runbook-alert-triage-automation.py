"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "alert_triage"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.inc260713.timeline', 'date.2026-07-20', 'term.no-kio-claim']
FINGERPRINT = "efa786b9c21f41a5"

def alert_level(queue_depth: int) -> str:
    return 'critical' if queue_depth >= 24000 else 'warning' if queue_depth >= 12000 else 'normal'
