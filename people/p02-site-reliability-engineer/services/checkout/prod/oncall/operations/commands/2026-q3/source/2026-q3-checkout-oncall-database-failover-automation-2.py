"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "database_failover"
SPINE_IDS = ['org.ntc', 'dep.payments.q07', 'rel.oncall.2026-07-13', 'period.2026-q3', 'term.no-kio-claim']
FINGERPRINT = "48b1cfa10bcc141f"

def assess_failover_evidence(status: str) -> bool:
    return status == 'reviewed'
