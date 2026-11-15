"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "capacity"
SPINE_IDS = ['org.ntc', 'deploy.dep-260713-017', 'rel.change.260713', 'period.2026-q3', 'term.no-kio-claim']
FINGERPRINT = "93669a8f46c3f0eb"

def incident_window() -> tuple[str, str]:
    return ('2026-07-13T09:18:00Z', '2026-07-13T10:42:00Z')
