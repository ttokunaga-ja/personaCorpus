"""Northstar Transit Cloud; Checkout SVC-17; 2026-07-13; error budget 2.4%; INC-260713."""
SERVICE = "Checkout SVC-17"
INCIDENT = "INC-260713"
TOPIC = "traffic_shift"
SPINE_IDS = ['org.ntc', 'term.mitigation', 'rel.inc260713.timeline', 'date.2026-07-15', 'term.no-kio-claim']
FINGERPRINT = "5e8f7f003488fe90"

def compare_traffic(baseline_rps: int, peak_rps: int) -> int:
    return peak_rps - baseline_rps
