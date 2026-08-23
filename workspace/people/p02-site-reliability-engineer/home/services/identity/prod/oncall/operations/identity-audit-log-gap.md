# Identity audit log gap response

## Trigger

Use this response when the audit pipeline reports a missing sequence range,
delivery delay over ten minutes, or a mismatch between aggregate authentication
decisions and persisted audit events. Treat an audit gap as an observability
and compliance incident even when authentication traffic is healthy.

## Preserve first

Keep the source event buffer and consumer checkpoint intact. Capture the
affected partition, sequence range, delivery destination, retention window,
and checksum status. Do not replay events into a destination until its current
checkpoint and duplicate-handling behavior are understood.

## Investigation path

1. Determine whether events were not produced, not consumed, or rejected by
   the destination.
2. Compare source sequence continuity with consumer acknowledgements.
3. Check schema version, destination availability, and consumer error classes.
4. Validate that the dead-letter path is writable and contains the expected
   count, if one is in use.
5. Notify the audit owner and compliance liaison with aggregate scope and
   estimated time range.

## Recovery

Resume from the last durable acknowledgement using the supported replay
mechanism. Reconcile the resulting sequence range and deduplicate by immutable
event ID at the destination. When source retention is insufficient, document
the irrecoverable interval, affected event category, and compensating evidence
such as identity decision aggregates.

## Closure evidence

Close only when sequence continuity is restored or the loss is formally
documented, destination lag is below two minutes, and the audit owner has
acknowledged the reconciliation result.
