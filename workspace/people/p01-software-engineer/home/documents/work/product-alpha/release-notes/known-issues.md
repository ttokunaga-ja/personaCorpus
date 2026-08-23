# Product Alpha R7 known issues

| Symptom | Impact | Workaround | Role | Status |
|---|---|---|---|---|
| Legacy automation expects `200 OK` from gate evaluation | Promotion script may stop before a terminal decision | Update the script to accept `202` and poll by `gate_id` | Release engineering | Documented |
| A drained worker leaves a gate `pending` during rollout | One synthetic canary can exceed the normal p95 | Retry the poll after worker recovery; do not resubmit the release | Platform operations | Monitoring |
| Timeline reader does not render `decision_ref` | Decision context is absent from an older dashboard | Upgrade the timeline renderer; the status endpoint remains complete | Developer experience | Planned |
| Gate completion p99 briefly crossed the warning threshold during canary | No promotion was affected; p95 remained 184 ms | Increase worker warm pool before the next release window | Platform operations | Resolved |

All examples are synthetic. See [canary observations](canary-observations.log) for the resolved warning and [rollback notes](rollback-notes.md) for escalation triggers.
