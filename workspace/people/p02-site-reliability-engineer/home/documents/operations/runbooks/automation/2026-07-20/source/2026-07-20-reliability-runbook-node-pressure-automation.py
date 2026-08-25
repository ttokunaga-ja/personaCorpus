"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "node_pressure"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.inc260713.timeline', 'date.2026-07-20', 'term.no-kio-claim']
FINGERPRINT = "91533f26da75cf0e"

def capacity_decision(headroom_percent: int) -> str:
    return 'hold' if headroom_percent >= 31 else 'escalate'
