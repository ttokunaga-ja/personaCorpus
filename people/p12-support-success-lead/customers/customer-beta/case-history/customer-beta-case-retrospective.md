# Customer Beta unresolved-case retrospective — 04 Aug 2026

## Frozen snapshot

The authoritative snapshot at `2026-08-04T09:00:00Z` contains five unresolved Customer Beta tickets under the four-hour HelioDesk SLA: three within, one at risk, and one breached. HelioDesk has 17 unresolved tickets overall. The five Customer Beta IDs are `HD-260804-1934`, `HD-260804-2861`, `HD-260804-2740`, `HD-260804-2774`, and `HD-260804-2803`.

## Focus case: HD-260804-1934

`HD-260804-1934` was created at `2026-08-04T03:42:00Z` with an SLA due at `2026-08-04T07:42:00Z`. At the snapshot it is `pending_customer` and `breached`, 78 minutes past due. The subject is “Dashboard export returns an empty CSV”; the assigned queue is `queue-west`.

The active resolution path is evidence collection, not closure. `HD-MAC-EVIDENCE-08` requested a fresh CSV export and selected dashboard filter details, and `HD-MAC-STATUS-12` records that the case remains open/breached. On receipt, queue-west will compare the requested export with the dashboard view and communicate either a verified workaround or a further investigation milestone.

## Operational follow-through

The at-risk case is `HD-260804-2861`, due at `2026-08-04T09:12:00Z`. The three within-SLA cases are `HD-260804-2740`, `HD-260804-2774`, and `HD-260804-2803`. This record is a synthetic operational summary and does not mark any Customer Beta case resolved or reopened.
