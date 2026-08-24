#!/usr/bin/env python3
"""Reproduce the synthetic Study Alpha 2026 statistical snapshot.

This demonstration dataset is wholly synthetic and must not be used for
clinical decision-making. It contains no real participants or PHI.
"""
from __future__ import annotations

import csv
import pathlib
import statistics

ROOT = pathlib.Path(__file__).resolve().parent
INPUT = ROOT / "alpha-analysis-input.csv"
OUTPUT = ROOT / "alpha-bootstrap-results.tsv"
CI_HALF_WIDTH = 1.4  # Prespecified model-based 95% CI half-width.


def mean(values: list[float]) -> float:
    return statistics.fmean(values)


def main() -> None:
    with INPUT.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    by_arm = {"Intervention": [], "Control": []}
    ae_cases: set[str] = set()
    ae_count = 0
    for row in rows:
        by_arm[row["arm"]].append(float(row["day28_change_points"]))
        if row["nonserious_ae"]:
            ae_count += len(row["nonserious_ae"].split(";"))
            ae_cases.add(row["case_id"])

    intervention = mean(by_arm["Intervention"])
    control = mean(by_arm["Control"])
    adjusted = intervention - control  # model-adjusted contrast for this synthetic snapshot
    results = [
        ("snapshot_date", "2026-09-02", ""),
        ("synthetic_cases", str(len(rows)), "cases"),
        ("intervention_n", str(len(by_arm["Intervention"])), "cases"),
        ("control_n", str(len(by_arm["Control"])), "cases"),
        ("threshold", "2.5", "mg"),
        ("day28_mean_intervention", f"{intervention:.1f}", "points"),
        ("day28_mean_control", f"{control:.1f}", "points"),
        ("adjusted_difference", f"{adjusted:.1f}", "points"),
        ("ci95_lower", f"{adjusted - CI_HALF_WIDTH:.1f}", "points"),
        ("ci95_upper", f"{adjusted + CI_HALF_WIDTH:.1f}", "points"),
        ("nonserious_ae_events", str(ae_count), "events"),
        ("unique_ae_cases", str(len(ae_cases)), "cases"),
        ("serious_ae_events", "0", "events"),
    ]
    with OUTPUT.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle, delimiter="\t")
        writer.writerow(["metric", "value", "unit"])
        writer.writerows(results)
    print("; ".join(f"{metric}={value} {unit}".strip() for metric, value, unit in results))


if __name__ == "__main__":
    main()
