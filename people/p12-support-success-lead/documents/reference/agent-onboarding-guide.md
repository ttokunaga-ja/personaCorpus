# HelioDesk Support Operations — Agent Onboarding Guide

**Synthetic internal training reference.** This guide describes the HelioDesk support operating model as of the example snapshot **2026-08-04**. All names, domains, ticket examples, and metrics are invented.

## The service promise

HelioDesk acknowledges every newly reported support case within **four elapsed hours**, 24×7. The acknowledgement must be useful: it confirms case ownership, records the current severity, and tells the requester what will happen next. The four-hour SLA is an acknowledgement target, not a promise to resolve within four hours.

Start with the least sensitive evidence that can explain the issue. Verify the requester’s authorized contact route, search for matching incidents, then set the case state and next action. Do not copy personal data, access tokens, passwords, cookies, session IDs, private keys, or unredacted logs into a case.

## First-shift workflow

1. Read the queue and take cases in severity order: Sev 1, Sev 2, then customer-impacting Sev 3, then Sev 4.
2. Confirm `case_id`, `reported_at_utc`, `acknowledged_at_utc`, current `severity`, `status`, and assigned `owner_role`.
3. Acknowledge within four elapsed hours. Record a plain-language next step and time for the next update.
4. Triage impact and scope. Use the severity reference; do not promote severity solely because a requester asks.
5. Collect only redacted evidence. Follow the Windows log collection reference if endpoint logs are needed.
6. Escalate through the named role path when the definition requires it. Maintain ownership until the receiving role explicitly accepts the handoff.
7. Before closing, document the outcome, remediation or workaround, requester confirmation where applicable, and the next monitoring action.

## Worked example — not a live queue report

On **2026-08-04 14:00 UTC**, the synthetic queue snapshot shows **17 unresolved cases**: 1 Sev 1, 4 Sev 2, 8 Sev 3, and 4 Sev 4. This count is a training example only; it is not a staffing forecast, historical performance claim, or open-case target. Use it to practice sorting work and validating coverage, not to infer current queue volume.

## Quality bar for acknowledgement

An acknowledgement should say: “We have your report, it is currently assessed as [severity], [role] owns the next step, and the next update is due by [time].” It must not restate sensitive material, make speculative root-cause claims, or give a resolution deadline that has not been approved.

## Terms used in this pack

The glossary defines common language. The field dictionary and XML reference define the exact exchange fields. In particular, `severity` measures impact and urgency; `status` measures workflow state; `sla_ack_due_at_utc` is calculated as four elapsed hours after `reported_at_utc`.

## Escalation and handoff rules

- Page the Incident Commander immediately for Sev 1 and start the communications cadence in the severity reference.
- Escalate Sev 2 when customer operations are materially impaired or a workaround is not viable; involve the relevant technical role.
- Treat a handoff as complete only after the receiving role has accepted it and the case records a next update time.
- If coverage is unavailable, use the roster’s Backup role, then the Incident Commander path; do not leave a case unowned.

## Safe evidence handling

Use the redaction reference before upload. Keep original evidence in the approved customer-controlled or internal secure channel where policy permits; the case should contain the minimum redacted excerpt and a description of where the original was handled. If a secret is exposed, stop forwarding it, request revocation/rotation through the approved security process, and alert the Security Liaison.
