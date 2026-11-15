"""Northstar Transit Cloud; Checkout SVC-17 incident context; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "queue_saturation"
SPINE_IDS = ['org.ntc', 'svc.checkout.svc17', 'rel.inc260713.timeline', 'date.2026-07-15', 'term.no-kio-claim']
FINGERPRINT = "acd29c34942d58e6"

def estimate_drain_minutes(queue_depth: int, jobs_per_minute: int) -> float:
    return queue_depth / jobs_per_minute
