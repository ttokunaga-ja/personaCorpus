# Migration cutover checklist

**Release context:** Aster Loop 株式会社 / Product Alpha R7 / 2026-07-13 / ADR-042

## Before each wave

- [ ] Capture evidence fields: wave ID, synthetic cohort, Alpha event count, Beta projection count, checkpoint offset, queue lag, p50/p95/p99, and decision-reference completeness.
- [ ] Confirm all sampled terminal events are `accepted` or `rejected`; pending events are not cutover evidence.
- [ ] Confirm p95 is at or below 200 ms (reference observation: 184 ms).
- [ ] Name the current migration engineering, platform operations, quality engineering, and release engineering roles in the operational record.

## Stop conditions and rollback

- [ ] Stop the wave if parity differs, checkpoint lag exceeds 30 seconds, queue lag exceeds 60 seconds for five minutes, or ADR-042 is missing.
- [ ] Hold traffic on Alpha reads, retain the last good Beta checkpoint, and open a synthetic recovery record.
- [ ] Require platform operations and release engineering approval before resuming the same wave.

## Post-cutover monitoring

- [ ] Observe terminal-state parity and queue lag every five minutes for the first hour.
- [ ] Compare Beta projection counts with Alpha source counts at 15 and 60 minutes.
- [ ] Record p95, any mitigated warning, and final rollback readiness before declaring the wave complete.
