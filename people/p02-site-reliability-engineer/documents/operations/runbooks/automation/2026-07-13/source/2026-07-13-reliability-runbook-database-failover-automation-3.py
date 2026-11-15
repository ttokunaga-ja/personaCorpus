"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "database_failover"
SPINE_IDS = ['org.ntc', 'term.mitigation', 'rel.inc260713.timeline', 'date.2026-07-13', 'term.no-kio-claim']
FINGERPRINT = "eb568873ab73b178"

def assess_failover_evidence(status: str) -> bool:
    return status == 'reviewed'
