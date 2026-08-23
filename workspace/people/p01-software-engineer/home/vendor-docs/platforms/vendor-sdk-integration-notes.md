# Synthetic SDK integration notes

Adapters isolate the application from fictional Nimbus, Orbit, and Harbor SDK surfaces. Pin the adapter contract and the package versions listed in `platform-version-pins.yaml`; upgrades require compatibility review and a staging retry exercise.

The adapter assigns a request ID, maps transport failures to safe domain errors, and retries only idempotent reads or replay-safe create operations. Limitations: no cross-tier transaction guarantee, asynchronous worker completion is not part of the p95 184 ms API objective, and ADR-042 approval remains application-owned. No credentials or vendor endpoints are stored in this snapshot.
