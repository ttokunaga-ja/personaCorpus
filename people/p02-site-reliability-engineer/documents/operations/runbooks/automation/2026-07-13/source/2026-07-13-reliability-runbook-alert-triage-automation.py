"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "alert_triage"
SPINE_IDS = ['org.ntc', 'term.mitigation', 'rel.inc260713.timeline', 'date.2026-07-13', 'term.no-kio-claim']
FINGERPRINT = "d83814503a56c154"

def alert_level(queue_depth: int) -> str:
    return 'critical' if queue_depth >= 24000 else 'warning' if queue_depth >= 12000 else 'normal'
