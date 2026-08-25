"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "queue_saturation"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.inc260713.timeline', 'date.2026-07-14', 'term.no-kio-claim']
FINGERPRINT = "6b35528c6e4d45f2"

def estimate_drain_minutes(queue_depth: int, jobs_per_minute: int) -> float:
    return queue_depth / jobs_per_minute
