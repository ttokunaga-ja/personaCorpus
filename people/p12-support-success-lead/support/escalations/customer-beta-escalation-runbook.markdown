# Customer Beta escalation runbook

**Scope:** 5 unresolved Beta cases in the frozen 2026-08-04 09:00 UTC snapshot, including breached `HD-260804-1934` (`pending_customer`).

## Triage in five minutes

1. Verify the customer-impact statement and the 4-hour first-response deadline.
2. Route breached `HD-260804-1934` to the assigned Support Duty Manager; record the recovery acknowledgement and next update time.
3. For each case, assign one customer communicator and one technical action owner.
4. Place the update deadline in the case timeline and add a matching JSONL event.

## Customer communications

- Use plain language: impact, current action, owner role, and the next update time.
- Do not describe an issue as resolved until the customer communicator has confirmed the expected behavior.
- If the first response is late, acknowledge the miss directly and give the next concrete checkpoint.

## Escalation ladder

Duty Manager → Support Success Lead → Engineering Incident Commander. Escalate immediately for P1, security-sensitive reports, or a missed commitment. The remaining Beta at-risk case is `HD-260804-2861`; keep P3/P4 cases in the normal queue unless customer impact or deadline risk changes.
