"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Identity SVC-23"
INCIDENT = "INC-260713"
TOPIC = "identity"
SPINE_IDS = ['org.ntc', 'inc.inc-260713', 'rel.oncall.2026-07-13', 'date.2026-07-20', 'term.no-kio-claim']
FINGERPRINT = "4192729d1cf9f7d0"

def incident_window() -> tuple[str, str]:
    return ('2026-07-13T09:18:00Z', '2026-07-13T10:42:00Z')
