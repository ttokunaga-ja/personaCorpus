# Product Alpha to Product Beta migration plan

**Aster Loop 株式会社 · Product Alpha R7 · release 2026-07-13 · ADR-042 · p95 184 ms**

## Objective and prerequisites

Move the synthetic release-decision projection from Alpha R7 to Beta without changing Alpha's release authority. ADR-042 remains the authority for Alpha's asynchronous gate; Beta consumes only terminal decisions. Before migration, verify terminal polling compatibility, an append-only decision reference, a healthy checkpoint, and a rehearsed rollback owner.

## Phases and timeline

| Phase | Window | Work | Accountable role |
|---|---|---|---|
| Prepare | T-7 to T-2 days | Validate schema guard, baseline queue lag, publish waves | Migration engineering |
| Dual-run | T-1 day | Mirror terminal decisions to Beta; compare projections without changing reads | Platform operations |
| Validate | Release window | Confirm cohort parity, p95 evidence, and checkpoint advancement | Quality engineering |
| Cutover | After validation | Route synthetic read traffic to Beta in waves | Release engineering |
| Observe | T+1 day | Monitor lag, terminal parity, and rollback triggers | Platform operations |

## Dual-run and verification

Alpha remains the source of truth throughout dual-run. Beta consumes the same terminal event sequence and calculates a projection checksum per cohort. The validation gate opens only when every sampled event has the same terminal state and decision reference, checkpoint lag is below 30 seconds, and the observed synthetic gate p95 remains 184 ms or lower than the 200 ms guardrail.

## Rollback

Stop the next wave if parity is incomplete, queue lag exceeds 60 seconds for five minutes, or terminal decisions lack ADR-042. The release engineering role restores Alpha reads, platform operations pauses Beta consumption after retaining checkpoints, and quality engineering records the synthetic evidence. No records are deleted or replayed in place; recovery resumes from the retained checkpoint.
