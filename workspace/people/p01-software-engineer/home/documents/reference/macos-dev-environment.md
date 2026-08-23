# macOS development environment reference

This ja-JP macOS reference is synthetic and local-only. Keep locale-aware date rendering separate from API timestamps: Product Alpha R7 continues to use UTC RFC 3339. Store project settings in a local ignored directory, use invented sample configuration names, and never place credentials in a reference document.

Recommended workflow: format Rust and TypeScript before review; run deterministic contract fixtures; render docs locally; and keep the release window 2026-07-13 evidence with ADR-042. The p95 184 ms budget is measured in staging fixtures, not in desktop editor timing.
