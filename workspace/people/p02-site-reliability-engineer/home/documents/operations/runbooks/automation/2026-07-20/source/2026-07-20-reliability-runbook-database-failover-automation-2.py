"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "database_failover"
SPINE_IDS = ['org.ntc', 'svc.identity.svc23', 'rel.inc260713.timeline', 'date.2026-07-20', 'term.no-kio-claim']
FINGERPRINT = "fcfa81f789937a5e"

def assess_failover_evidence(status: str) -> bool:
    return status == 'reviewed'
