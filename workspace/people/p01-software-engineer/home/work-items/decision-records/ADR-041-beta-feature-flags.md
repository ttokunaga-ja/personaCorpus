# ADR-041: Beta feature-flag controls

**Status:** Accepted | **Date:** 2026-07-13

Preview flags have a named owner role, a rollout constraint, an expiry, and a fallback. The Platform Release Owner enables the Product Beta preview only during the Product Alpha R7 release window. The flag expires after the review decision; fallback returns callers to the Alpha R7 contract without executing a Beta-only action.

Flag state is not an approval substitute. ADR-042 remains authoritative; no flag enables execution before approval.
