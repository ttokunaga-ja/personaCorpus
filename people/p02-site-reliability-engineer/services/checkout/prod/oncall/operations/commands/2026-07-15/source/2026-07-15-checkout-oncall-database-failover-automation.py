"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "database_failover"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.oncall.2026-07-13', 'date.2026-07-15', 'term.no-kio-claim']
FINGERPRINT = "5932c9f65369c732"

def assess_failover_evidence(status: str) -> bool:
    return status == 'reviewed'
