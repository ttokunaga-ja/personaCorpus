# Identity MFA provider failure response

## Detection

An MFA provider event is declared when challenge delivery or verification
success drops below 97% for five consecutive minutes, segmented by factor
type and geography. Separate provider failure from client network problems by
checking callback success, challenge creation latency, and provider status
signals.

## Immediate customer-safe behavior

- Continue to require MFA for policies that require it.
- Offer an already enrolled alternate factor when policy permits.
- Preserve the same risk evaluation and session duration for alternate-factor
  flows.
- Display a retry interval for transient provider errors rather than repeating
  challenge creation immediately.

## Response sequence

1. Open an incident and notify the provider liaison with aggregate timestamps
   and failure codes.
2. Enable the alternate-factor route for one canary region.
3. Observe completion rate, fraud signals, and support-contact rate for ten
   minutes.
4. Expand only if alternate-factor capacity and error rate remain within their
   objectives.
5. If no eligible alternative exists, retain the challenge requirement and
   present a clear availability message; do not convert the request into a
   password-only login.

## Recovery

After the provider recovers, keep alternate routing available for 15 clean
minutes, then return traffic in stages. Reconcile challenge-created and
challenge-verified aggregates to identify delivery gaps. Preserve audit events
for both attempted and completed factors.
