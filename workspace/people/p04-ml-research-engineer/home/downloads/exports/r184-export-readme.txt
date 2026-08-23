Helix Forge Lab — R184 export bundle

Purpose
  Compact, self-contained examples for reviewing evaluation-export handling.
  The bundle is synthetic and contains no customer records, credentials, network
  configuration, or external service dependency.

Evaluation reference
  Date: 2026-07-13
  Suite: HF-Eval-S1/S2 v3.2
  Total: 2400 examples (S1 1600; S2 800)
  Labels: account_access 492; delivery_status 476; plan_change 481;
          refund_request 462; technical_help 489.
  Alpha M-14: precision 0.881, recall 0.865, F1 0.873.
  Beta B-07: precision 0.852, recall 0.841, F1 0.846.

Contents
  export_metrics.py    Python metric consistency summary.
  export_manifest.rs   Rust manifest-style total and metric checks.
  sanitize_export.rs   Rust field normalization and label allow-list example.
  export_report.ts     TypeScript text-report example.

Suggested local checks
  python3 -B export_metrics.py
  rustc --test export_manifest.rs && ./export_manifest
  rustc --test sanitize_export.rs && ./sanitize_export
  node --experimental-strip-types export_report.ts
