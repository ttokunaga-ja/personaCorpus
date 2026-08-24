HelioDesk synthetic export batch
Snapshot anchor: 2026-08-04 | unresolved queue: 17 | SLA target: 4 hours

Provenance
----------
Every file in this directory is synthetic training material. HelioDesk, Customer
Alpha, Customer Beta, case IDs, teams, and metrics are invented for this corpus.
The batch contains no customer records, credentials, connection strings, or live
system exports. SQL files are documented query templates only; do not treat them
as executable production access instructions.

Files and schemas
-----------------
2026-08-04-support-metrics.csv
  Point-in-time unresolved queue snapshot. Columns:
  snapshot_date, queue_scope, case_id, customer_segment, priority,
  sla_target_hours, sla_state, age_hours, owner_team, summary.
  Canonical source: support/ticket-exports/heliodesk-2026-08-04-open-tickets.json.
  Each row uses the canonical ticket ID, customer (with non-Alpha/Beta customers
  normalized to Other), priority, SLA state, subject, assignee, and unresolved
  age rounded from minutes to one decimal hour. Reconciliation: 17 rows;
  Customer Alpha=9, Customer Beta=5, Other=3; priority P1=1/P2=5/P3=8/P4=3;
  SLA within=11/at-risk=4/breached=2. Breaches are HD-260804-2718 and
  HD-260804-1934.

weekly-resolution-trend.csv
  Weekly flow metrics, not a point-in-time queue. Columns:
  week_start, metric_basis, new_cases, resolved_cases, reopened_cases,
  median_first_response_minutes, median_resolution_hours, definition_note.
  New and resolved cases are counted by weekly event flow; they must not be
  compared directly to the 17-item snapshot queue.

alpha-qbr-export.sql and beta-qbr-export.sql
  Structured SQL text with named parameters :customer_segment, :period_start,
  and :period_end. They describe QBR reporting aggregates for synthetic Alpha
  and Beta segments. Bind parameters through an approved reporting environment.

export_cleaner.rs
  Standard-library-only offline utility: export_cleaner <input.csv> <output.csv>.
  It parses RFC 4180-style single-line records, rejects cells beginning (after
  whitespace) with =, +, -, or @, and replaces identifier-like tokens containing
  .invalid with [redacted-invalid-identifier]. It makes no network calls.

Integrity checks
----------------
SHA-256 payload values, calculated after validation:
2026-08-04-support-metrics.csv  50798cb0bd82f95bbab428bdd02694527552b53b4a5343d5f0e49f0c78689ef5
weekly-resolution-trend.csv     570c344640356ff5e9c5a44087d4585bfb97de022d75df89a382246fe6d29077
alpha-qbr-export.sql            7e2b1d87db9a58c77f59c4d84e52060574333e9e4356ab95fc559e902077f485
beta-qbr-export.sql             46017c84c2076a38b99bff12c2bb260fafee55481e2615aa6bb306e14cbdb727
export_cleaner.rs               2bde97f4dcf39811e5345040d7d079076c8d9feda130bd649dd00312898cf1e1
The README is intentionally excluded from its embedded list because a file
cannot contain a stable hash of its own final bytes. Verify every current file
with: shasum -a 256 2026-08-04-support-metrics.csv weekly-resolution-trend.csv
alpha-qbr-export.sql beta-qbr-export.sql export_cleaner.rs export-batch-readme.txt

Suggested offline validation
----------------------------
python3 -c 'import csv; list(csv.DictReader(open("2026-08-04-support-metrics.csv")))'
python3 -c 'import csv; list(csv.DictReader(open("weekly-resolution-trend.csv")))'
rustc --edition=2021 export_cleaner.rs -o export_cleaner
./export_cleaner 2026-08-04-support-metrics.csv cleaned.csv
