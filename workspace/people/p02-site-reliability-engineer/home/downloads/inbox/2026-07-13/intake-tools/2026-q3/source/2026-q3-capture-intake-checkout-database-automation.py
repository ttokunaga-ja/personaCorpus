"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "incident_timeline"
SPINE_IDS = ['org.ntc', 'term.evidence-chain', 'rel.archive.closed', 'period.2026-q3', 'term.no-kio-claim']
FINGERPRINT = "91eaa7046a6bc231"

def incident_window() -> tuple[str, str]:
    return ('2026-07-13T09:18:00Z', '2026-07-13T10:42:00Z')
