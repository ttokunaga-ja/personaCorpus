"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "database_failover"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.inc260713.timeline', 'period.2026-q3', 'term.no-kio-claim']
FINGERPRINT = "72a6d45339de5089"

def assess_failover_evidence(status: str) -> bool:
    return status == 'reviewed'
