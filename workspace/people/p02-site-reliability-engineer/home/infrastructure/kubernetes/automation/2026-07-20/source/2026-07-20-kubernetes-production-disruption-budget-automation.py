"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "incident_timeline"
SPINE_IDS = ['org.ntc', 'svc.identity.svc23', 'rel.topology.260713', 'date.2026-07-20', 'term.no-kio-claim']
FINGERPRINT = "e1e68e8fd49112f7"

def incident_window() -> tuple[str, str]:
    return ('2026-07-13T09:18:00Z', '2026-07-13T10:42:00Z')
