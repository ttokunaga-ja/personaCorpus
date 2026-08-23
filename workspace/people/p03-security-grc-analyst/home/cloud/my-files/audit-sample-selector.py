#!/usr/bin/env python3
"""Select a reproducible, local audit sample from fictional evidence labels."""

from hashlib import sha256


EVIDENCE_IDS = ("EV-184-ALPHA", "EV-184-BRAVO", "EV-184-CHARLIE", "EV-184-DELTA", "EV-184-ECHO")


def sample_key(evidence_id: str) -> str:
    """Return a short deterministic marker, not a security credential."""
    return sha256(("Cobalt Harbor|2026-07-13|" + evidence_id).encode("utf-8")).hexdigest()[:10]


def select_sample(size: int = 3) -> tuple[str, ...]:
    ranked = sorted(EVIDENCE_IDS, key=sample_key)
    return tuple(ranked[:size])


def main() -> None:
    print("Aegis Control AC-27 / evidence set 184 / RISK-031")
    print("監査抽出は再現可能な確認用サンプルであり、外部接続を行いません。")
    for evidence_id in select_sample():
        print(f"{evidence_id}: review-marker={sample_key(evidence_id)}")


if __name__ == "__main__":
    main()
