# May Identity reporting — first-cut checklist

For Daniel and Emma · review 3 June 2024, 10:00 UTC

- [ ] Keep the UTC month boundary unchanged: 1 May 00:00 inclusive to 1 June 00:00 exclusive. Reconcile the elapsed time to 31 days, 744 hours and 44,640 scheduled minutes; check for duplicated or missing hours before calculating the rate.
- [ ] Reconcile successful and failed scheduled outcomes to the eligible minutes. Put any absent outcome in its own unobserved-minutes field beside the availability figure; do not silently remove it from the denominator or treat it as zero failures.
- [ ] Check separately retained evidence against the same scheduled minute and remove duplicate matches. Record what it establishes and leave unmatched minutes unobserved.
- [ ] If absence remains, calculate the lower bound with all unknown minutes failed and the upper bound with them successful, both over the full 44,640-minute window. Compare unrounded bounds with 99.9%; if the target lies between them, report target status undetermined.
- [ ] Read the count and any interval against the supporting hourly detail before issue.

## April reminder

For April, 43,200 eligible minutes, 41 confirmed failures and four unknown produce 1 − 41/43,200 = 99.90509% at the upper bound. Looking at that figure alone would suggest a pass against 99.9%. But if all four unknown minutes failed, 1 − 45/43,200 = 99.89583%. The target lies between the two; the actual status is undetermined.

## May first cut

Eligible minutes: __________  
Confirmed successes: __________  
Confirmed failures: __________  
Unobserved minutes: __________  
Availability / bounds: __________  
Target status: __________

Emma review: __________  
Daniel review: __________  
Date: __________
