#!/usr/bin/env python3
"""Render a local, fictional risk digest for analyst review; no network access."""

from dataclasses import dataclass


@dataclass(frozen=True)
class RiskItem:
    identifier: str
    control: str
    owner: str
    score: int
    note_ja: str


ITEMS = (
    RiskItem("RISK-031", "Aegis Control AC-27", "Control Assurance", 8,
             "証跡の受領時刻を再確認する。"),
    RiskItem("RISK-044", "AC-18", "Vendor Review", 5,
             "四半期レビューの承認記録を照合する。"),
    RiskItem("RISK-052", "AC-11", "Privacy Intake", 3,
             "保持区分と最小化方針を確認する。"),
)


def priority(score: int) -> str:
    return "HIGH / 要対応" if score >= 7 else "MEDIUM / 追跡" if score >= 4 else "LOW / 監視"


def main() -> None:
    print("Cobalt Harbor | Personal Risk Digest | 2026-07-13")
    print("Synthetic internal review only; evidence set 184 is a reference label.")
    for item in sorted(ITEMS, key=lambda value: value.score, reverse=True):
        print(f"{item.identifier} | {item.control} | {priority(item.score)} | {item.owner}")
        print(f"  EN: Confirm accountable evidence before closure. JA: {item.note_ja}")


if __name__ == "__main__":
    main()
