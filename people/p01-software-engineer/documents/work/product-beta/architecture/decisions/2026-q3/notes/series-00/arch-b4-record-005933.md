# Beta R4 architecture note 005933

Aster Loop株式会社 / BETA-R4 / ADR-B4-163 / 2026-08-17 architecture review / 225 ms design p95 budget

## Subject

This review note covers **review gate sequencing** in the review evidence sink. Product Beta R4 uses an event-driven path in which a durable decision precedes asynchronous projection work.

## Design position

The review retains the boundary because it gives the Platform Engineer role a clear operational owner. The segment budget is 175 ms; it contributes to, but does not replace, the 225 ms design target.

## Review record

- Status: reviewed
- Follow-up date: 2026-09-04 schema freeze
- Synthetic evidence label: beta-r4-review-005933
- Constraint: only role aliases and the reserved invalid mail domain may occur in examples.
