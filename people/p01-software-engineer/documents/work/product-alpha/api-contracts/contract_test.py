"""Deterministic Product Alpha R7 contract examples; no network or credentials."""
from dataclasses import dataclass
@dataclass(frozen=True)
class Reply: status: int; code: str | None = None
def create(version: str, key: str, approved: bool = False, timeout: bool = False) -> Reply:
    if timeout: return Reply(504, "upstream_timeout")
    if len(key) < 8: return Reply(400, "idempotency_key_required")
    if not version.startswith("r7."): return Reply(422, "invalid_version")
    return Reply(201 if approved else 202)
def test_happy_path_after_adr_042_gate(): assert create("r7.0", "fixture-key-01", True).status == 201
def test_conflict_fixture(): assert Reply(409, "idempotency_conflict").code == "idempotency_conflict"
def test_validation_fixture(): assert create("r6.0", "fixture-key-01").code == "invalid_version"
def test_timeout_fixture(): assert create("r7.0", "fixture-key-01", timeout=True) == Reply(504, "upstream_timeout")
