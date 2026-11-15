"""Deterministic percentile-bootstrap helper for synthetic evaluation scores."""
from __future__ import annotations

from random import Random


def percentile(values: list[float], q: float) -> float:
    if not values or not 0 <= q <= 1:
        raise ValueError("values must be nonempty and q must be in [0, 1]")
    ordered = sorted(values)
    return ordered[round((len(ordered) - 1) * q)]


def bootstrap_mean_ci(scores: list[float], repeats: int = 2_000, seed: int = 184) -> tuple[float, float, float]:
    """Return observed mean and a 95% percentile interval without external packages."""
    if len(scores) < 2 or repeats < 100:
        raise ValueError("need two scores and at least 100 repeats")
    rng, n = Random(seed), len(scores)
    samples = [sum(scores[rng.randrange(n)] for _ in range(n)) / n for _ in range(repeats)]
    return sum(scores) / n, percentile(samples, 0.025), percentile(samples, 0.975)


if __name__ == "__main__":
    print(tuple(round(x, 3) for x in bootstrap_mean_ci([.865, .873, .881, .876, .870])))
