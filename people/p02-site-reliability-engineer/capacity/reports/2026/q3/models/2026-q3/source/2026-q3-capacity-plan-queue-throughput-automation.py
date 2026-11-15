"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "capacity"
SPINE_IDS = ['org.ntc', 'measure.capacity.headroom', 'rel.capacity.2026-q3', 'period.2026-q3', 'term.no-kio-claim']
FINGERPRINT = "d615f3a66e45d5bd"

def incident_window() -> tuple[str, str]:
    return ('2026-07-13T09:18:00Z', '2026-07-13T10:42:00Z')
