"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "database_failover"
SPINE_IDS = ['org.ntc', 'dep.payments.q07', 'rel.oncall.2026-07-13', 'date.2026-07-15', 'term.no-kio-claim']
FINGERPRINT = "4848e0ee999311b8"

def assess_failover_evidence(status: str) -> bool:
    return status == 'reviewed'
