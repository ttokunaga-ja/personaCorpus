# Identity key rotation procedure

## Rotation objective

Rotate signing keys without interrupting validation by publishing a new public
key before it signs tokens and retaining the prior public key for a measured
overlap period. Key material itself is managed by the approved key-management
service and is never copied into tickets, chat, or shell history.

## Change plan

1. Obtain a scheduled change record and two approvers: Identity Platform and
   Security Operations.
2. Create the new signing-key reference in the key-management service.
3. Publish its public metadata to the verifier distribution channel as
   `pending`; confirm all regions acknowledge the same key identifier.
4. Wait 15 minutes, or two complete cache-refresh intervals, before assigning
   new token issuance to the pending reference.
5. Observe token validation, unknown-key rejections, and issuer latency for
   30 minutes.
6. Mark the new reference active. Retain the prior public metadata through the
   maximum token lifetime plus clock-skew allowance.
7. Retire the old reference only after the overlap report shows no remaining
   valid tokens signed with it.

## Abort conditions

Pause the rollout if public metadata differs across regions, unknown-key
rejections exceed 0.05%, or validation errors increase materially in any
canary. Restore issuance to the previously active reference; retain both
public entries while investigation proceeds.

## Evidence

The change record must contain the new and prior key identifiers, timestamped
distribution acknowledgements, overlap duration, approvers, and aggregate
validation metrics. It must not contain private keys, token samples, or
credential-bearing command output.
