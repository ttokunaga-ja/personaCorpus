# Product Alpha weekly notes - 2026-07-20

**Juniper Works | Product Alpha Q3 | M-27: 2026-09-18**

## Purpose

Confirm execution readiness for the 120-workspace pilot and preserve the Product Council decisions from 2026-07-13.

## Attendees

Product Management, Engineering, Product Design, Data Analytics, Customer Support.

## Facts

- Baseline adoption remains 18.4% across the frozen 1,240 synthetic workspaces; the target remains 26.0%.
- The pilot is limited to 120 synthetic workspaces and will report separately from the frozen baseline denominator.
- The guided path includes a first outcome, permission explainer, weekly digest default, and export preview.

## Debate

Engineering proposed temporarily using a daily digest to surface early guidance. Product Design and Support expected this to confound notification-fatigue evidence. The group retained the weekly default and agreed to observe opt-out and support-contact signals.

## Decisions

- **DR-085:** Keep the pilot at 120 workspaces; no additional cohorts before M-27 evidence review.
- **DR-086:** Instrument adoption, notification preference changes, permission-help exits, and export-preview completion using the DR-081 adoption definition.

## Actions

| Owner role | Due | Action |
| --- | --- | --- |
| Engineering | 2026-07-27 | Verify flag isolation and rollback in the pilot environment. |
| Data Analytics | 2026-07-28 | Validate the cohort dashboard against DR-081. |
| Customer Support | 2026-07-31 | Define synthetic support-contact categorization for notification and access concerns. |

## Risks

The short pilot window may show early engagement without a stable seven-day adoption read.

## Unresolved

Whether export-preview completion alone is sufficient evidence for the DR-083 export-confidence gate remains open.
