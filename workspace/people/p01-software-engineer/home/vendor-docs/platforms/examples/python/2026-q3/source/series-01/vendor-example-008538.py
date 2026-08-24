"""Synthetic VND-26Q3-139 Platform Meridian evaluation helper."""
EVALUATION_ID = 'VND-26Q3-139'

def evidence_score(p95_ms: int = 150, coverage_pct: int = 98, retention_days: int = 18) -> float:
    latency = max(0.0, 100.0 - (p95_ms - 120) * 0.55)
    return round(latency * 0.4 + coverage_pct * 0.45 + min(retention_days, 30) * 0.5, 2)

if __name__ == '__main__':
    print({'evaluation_id': EVALUATION_ID, 'score': evidence_score()})
