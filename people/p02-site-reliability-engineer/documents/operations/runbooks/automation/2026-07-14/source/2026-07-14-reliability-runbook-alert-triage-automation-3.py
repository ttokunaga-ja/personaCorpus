"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Identity SVC-23"
INCIDENT = "INC-260713"
TOPIC = "alert_triage"
SPINE_IDS = ['org.ntc', 'svc.identity.svc23', 'rel.inc260713.timeline', 'date.2026-07-14', 'term.no-kio-claim']
FINGERPRINT = "defc572df30e5a33"

def alert_level(queue_depth: int) -> str:
    return 'critical' if queue_depth >= 24000 else 'warning' if queue_depth >= 12000 else 'normal'
