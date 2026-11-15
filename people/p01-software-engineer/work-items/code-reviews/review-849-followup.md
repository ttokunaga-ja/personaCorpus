# Review 849 - instrumentation and schema guard follow-up

**Product Alpha R7 · 2026-07-13 · Aster Loop 株式会社**

Review 849 verifies two follow-up fixes from review 842. First, latency queries must filter to completed synthetic gate events before percentile calculation; pending and retry records are not comparable measurements. Second, the event payload schema guard must reject a terminal decision without the ADR-042 reference while allowing unknown additive fields for R6 coexistence.

The reviewed sample gives p95 **184 ms** after applying the terminal-state filter. The reviewer role requested no scope expansion: the change is limited to instrumentation semantics and validation errors. Approval is conditional on the query and fixture tests remaining deterministic.
