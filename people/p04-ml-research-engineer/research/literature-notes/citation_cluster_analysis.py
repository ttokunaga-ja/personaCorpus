"""Cluster synthetic/internal literature citations without network access."""
from collections import defaultdict

CITATIONS = [
    ("R184 Evaluation Ledger", {"metrics", "reproducibility", "review"}),
    ("HF-Eval-S1/S2 Design Card", {"dataset", "shift", "review"}),
    ("Balanced Intent Measurement", {"metrics", "labels", "thresholds"}),
    ("Confidence Review Under Partition Shift", {"shift", "calibration", "thresholds"}),
]

def jaccard(left: set[str], right: set[str]) -> float:
    union = left | right
    return len(left & right) / len(union) if union else 0.0

def cluster_citations(minimum: float = 0.2) -> list[list[str]]:
    groups: list[list[tuple[str, set[str]]]] = []
    for title, tags in CITATIONS:
        for group in groups:
            if any(jaccard(tags, peer_tags) >= minimum for _, peer_tags in group):
                group.append((title, tags))
                break
        else:
            groups.append([(title, tags)])
    return [[title for title, _ in group] for group in groups]

if __name__ == "__main__":
    for index, group in enumerate(cluster_citations(), start=1):
        print(f"cluster {index}: " + "; ".join(group))
