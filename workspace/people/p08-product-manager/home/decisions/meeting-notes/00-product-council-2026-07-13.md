# Product Council minutes - 2026-07-13

**Juniper Works | Product Alpha Q3 | baseline 2026-07-13 | M-27 guided rollout gate: 2026-09-18**  
**Decision context:** adoption is 18.4% across 1,240 synthetic workspaces; the Q3 outcome target is 26.0%.

## Purpose

Establish the controlled rollout decision frame, metric definition, and evidence required for the M-27 gate. This is synthetic planning material.

## Attendees

Product Management, Product Design, Engineering, Data Analytics, Customer Support, Security and Privacy, Go-to-Market Operations.

## Facts reviewed

- The 18.4% baseline counts a workspace only when an eligible admin completes the guided first outcome within seven days of enablement; it excludes test, suspended, and duplicate synthetic workspaces.
- The target is 26.0% under the same definition. The 1,240-workspace denominator is frozen for baseline comparison; later cohorts are reported separately.
- Discovery identifies four gates: first-value arrival, permission understanding, notification fatigue, and export confidence.
- M-27 is an expansion decision on 2026-09-18, not a feature-complete date. A controlled pilot can be reversed without broad default changes.

## Debate

Product Design argued that a broad default would produce faster learning. Engineering and Support noted that it would blend onboarding, notification, and export effects, making rollback and diagnosis difficult. Data Analytics requested a stable adoption denominator before any cohort comparison. Security and Privacy asked that export confidence be evaluated before expansion because permission clarity alone does not demonstrate a trusted export.

## Decisions

| ID | Decision | Rationale |
| --- | --- | --- |
| DR-081 | Freeze the adoption definition and 1,240-workspace baseline denominator for M-27 comparison. | Prevent metric drift while evaluating the 18.4% to 26.0% trajectory. |
| DR-082 | Run a 120-workspace controlled pilot before the M-27 gate. | Isolate guided-flow effects and retain a reversible rollout. |
| DR-083 | Require an export-confidence gate before any expansion beyond the pilot. | Trusted export is a release condition, not a follow-up enhancement. |
| DR-084 | Set notifications to one weekly digest by default, with clear controls. | Address notification fatigue without suppressing essential guidance. |

## Actions

| Owner role | Due | Action |
| --- | --- | --- |
| Data Analytics | 2026-07-17 | Publish the frozen adoption query and cohort comparison template. |
| Engineering | 2026-07-24 | Prepare feature flags, pilot enrollment, and rollback evidence for 120 workspaces. |
| Product Design | 2026-07-22 | Deliver guided first-outcome, permission explainer, and digest-control review. |
| Security and Privacy | 2026-08-07 | Define export-confidence acceptance evidence. |
| Product Management | 2026-09-11 | Assemble M-27 decision packet and unresolved-question log. |

## Risks

- A change to eligibility or the adoption event would invalidate the baseline comparison.
- Pilot enrollment may be insufficiently representative of the synthetic workspace mix.
- A favorable adoption movement without export confidence is not an expansion signal.

## Unresolved

- What minimum export-confidence threshold will Security and Privacy recommend for M-27?
- Which support-contact signal should be treated as a material regression during the 120-workspace pilot?
