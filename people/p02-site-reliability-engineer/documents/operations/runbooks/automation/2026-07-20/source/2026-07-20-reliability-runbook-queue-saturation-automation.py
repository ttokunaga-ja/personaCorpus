"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "queue_saturation"
SPINE_IDS = ['org.ntc', 'term.mitigation', 'rel.inc260713.timeline', 'date.2026-07-20', 'term.no-kio-claim']
FINGERPRINT = "d0b9c70692b83aa0"

def estimate_drain_minutes(queue_depth: int, jobs_per_minute: int) -> float:
    return queue_depth / jobs_per_minute
