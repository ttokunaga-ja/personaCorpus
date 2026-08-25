"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "incident_timeline"
SPINE_IDS = ['org.ntc', 'measure.log-export-gap', 'rel.inc260713.timeline', 'date.2026-07-15', 'term.no-kio-claim']
FINGERPRINT = "624b5676b16aa32f"

def incident_window() -> tuple[str, str]:
    return ('2026-07-13T09:18:00Z', '2026-07-13T10:42:00Z')
