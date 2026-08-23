# HF-Eval Label Taxonomy

## Decision rule

Assign the label to the first concrete outcome the synthetic requester needs.
Background events may be retained to create realistic phrasing, but they do not
become a second target. Each record has exactly one of the labels below.

| label | operational meaning | include | exclude |
| --- | --- | --- | --- |
| account_access | inability to enter or restore an account | sign-in, verification, locked access | device troubleshooting after access succeeds |
| delivery_status | request to locate or time a delivery | tracking, delay, arrival estimate | cancellation before shipment |
| plan_change | request to alter subscription terms | upgrade, downgrade, renewal setting | refund for a prior charge |
| refund_request | request to reverse a completed charge | reimbursement, duplicate billing | future plan changes |
| technical_help | malfunction while using a service or feature | errors, settings, feature behavior | account identity recovery |

## Boundary examples

A request such as “I cannot sign in after changing phones” is
`account_access` when the desired action is restoring entry. “The app loads but
the export button fails” is `technical_help`; it does not become access-related
merely because the request mentions a login session. For “Where is my parcel,
and can I cancel next month,” the first requested resolution governs the label.

## Adjudication discipline

Annotators write a short rationale only when a secondary cue could plausibly
change the label. A taxonomy owner resolves disagreements using the operational
meaning table, then records a versioned clarification. Labels are categorical
task targets, not judgments about a requester.
