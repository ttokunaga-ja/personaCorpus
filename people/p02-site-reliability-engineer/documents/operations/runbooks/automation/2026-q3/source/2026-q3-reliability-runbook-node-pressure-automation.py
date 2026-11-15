"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "node_pressure"
SPINE_IDS = ['org.ntc', 'svc.identity.svc23', 'rel.inc260713.timeline', 'period.2026-q3', 'term.no-kio-claim']
FINGERPRINT = "4e658d16cfe554f1"

def capacity_decision(headroom_percent: int) -> str:
    return 'hold' if headroom_percent >= 31 else 'escalate'
