"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "identity"
SPINE_IDS = ['org.ntc', 'date.2026-07-13', 'rel.oncall.2026-07-13', 'date.2026-07-14', 'term.no-kio-claim']
FINGERPRINT = "c41ed12d574c263e"

def incident_window() -> tuple[str, str]:
    return ('2026-07-13T09:18:00Z', '2026-07-13T10:42:00Z')
