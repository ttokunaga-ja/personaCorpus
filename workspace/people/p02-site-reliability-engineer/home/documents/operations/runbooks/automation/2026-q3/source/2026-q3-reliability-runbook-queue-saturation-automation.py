"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "queue_saturation"
SPINE_IDS = ['org.ntc', 'term.mitigation', 'rel.inc260713.timeline', 'period.2026-q3', 'term.no-kio-claim']
FINGERPRINT = "41c9eb2357fd67cb"

def estimate_drain_minutes(queue_depth: int, jobs_per_minute: int) -> float:
    return queue_depth / jobs_per_minute
