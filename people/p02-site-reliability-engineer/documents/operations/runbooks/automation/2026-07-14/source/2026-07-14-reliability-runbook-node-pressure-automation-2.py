"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "node_pressure"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.inc260713.timeline', 'date.2026-07-14', 'term.no-kio-claim']
FINGERPRINT = "2c27fcf937e38de2"

def capacity_decision(headroom_percent: int) -> str:
    return 'hold' if headroom_percent >= 31 else 'escalate'
