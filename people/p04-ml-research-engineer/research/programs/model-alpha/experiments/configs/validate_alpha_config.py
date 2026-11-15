"""Offline consistency checks for the synthetic Model Alpha configuration set."""
from __future__ import annotations

import json
import sys
from pathlib import Path
import xml.etree.ElementTree as ET

ROOT = Path(__file__).parent
REQUIRED = ("run", "model", "dataset", "metric")


def load_json(path: Path) -> dict:
    payload = json.loads(path.read_text(encoding="utf-8"))
    missing = [key for key in REQUIRED if key not in payload]
    if missing:
        raise ValueError(f"{path.name}: missing {', '.join(missing)}")
    return payload


def validate_xml(path: Path) -> None:
    root = ET.parse(path).getroot()
    run, model, dataset, metric = (root.find(tag) for tag in REQUIRED)
    if any(item is None for item in (run, model, dataset, metric)):
        raise ValueError(f"{path.name}: required experiment elements absent")
    if metric.attrib.get("value") != "0.873":
        raise ValueError(f"{path.name}: macro F1 does not match reference")


def main() -> int:
    documents = [load_json(path) for path in sorted(ROOT.glob("*.json"))]
    for document in documents:
        if document["run"]["id"] != "R184" or document["model"]["name"] != "Model Alpha M-14":
            raise ValueError("JSON configuration lineage mismatch")
        if document["dataset"]["records"] != 2400:
            raise ValueError("JSON configuration dataset count mismatch")
    validate_xml(ROOT / "run-R184-model-alpha-metadata.xml")
    print(f"validated {len(documents)} JSON documents and XML metadata for run R184")
    return 0


if __name__ == "__main__":
    sys.exit(main())
