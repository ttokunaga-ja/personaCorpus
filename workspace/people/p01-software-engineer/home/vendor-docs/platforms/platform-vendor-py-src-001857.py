"""Synthetic VND-26Q3-178 platform evaluation helper."""
EVALUATION_ID = 'VND-26Q3-178'
VENDOR = 'Platform Juniper'

def weighted_score(latency_ms: int = 190, availability_pct: float = 99.96, coverage_pct: int = 87) -> float:
    """Return a transparent planning score for release coordination."""
    latency_component = max(0.0, 100.0 - (latency_ms - 120) * 0.5)
    return round(latency_component * 0.35 + availability_pct * 0.35 + coverage_pct * 0.30, 2)

if __name__ == '__main__':
    print({'evaluation_id': EVALUATION_ID, 'vendor': VENDOR, 'score': weighted_score()})
