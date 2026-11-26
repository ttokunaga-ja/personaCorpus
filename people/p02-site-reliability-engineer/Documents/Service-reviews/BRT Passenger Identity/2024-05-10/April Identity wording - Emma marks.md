# April Passenger Identity — cover wording for Emma

3 May 2024 · Daniel Jennings · Draft for discussion

## Proposed headline

~~**April availability: 99.90509% against a 99.9% target.**~~

April Passenger Identity availability is between 99.89583% and 99.90509% against the 99.9% target. Four scheduled minutes, 23:51–23:55 UTC on 30 April, remain unobserved. The interval crosses the target, so its status cannot be determined from the available outcomes.

The April window contains 43,200 scheduled sign-in minutes. We have 41 observed failures. The primary collector was blank for the final 18 minutes of the month; Emma's separately retained verifier confirms 14 successful scheduled minutes within that gap. Four minutes, 23:51–23:55 UTC on 30 April, have no matched outcome.

The 14 recovered outcomes are nine from 23:42 to 23:51 and five from 23:55 to 00:00 UTC. They are completed sign-ins matched to their scheduled minutes, not a carry-forward across the gap. The 1 May flash charged all 18 missing minutes as downtime, producing 59 charged minutes and 99.86343%; that provisional estimate needs to be replaced when the report closes.

Emma's marks for Daniel, 7 May 2024:

- Strike through the proposed headline “April availability: 99.90509% against a 99.9% target.” It reads as a pass when the four missing outcomes could put the result below target.
- Replace the opening with: “April Passenger Identity availability is between 99.89583% and 99.90509% against the 99.9% target. Four scheduled minutes, 23:51–23:55 UTC on 30 April, remain unobserved. The interval crosses the target, so its status cannot be determined from the available outcomes.”
- Remove the footnote and bring its lower-bound assumption into the opening discussion. The lower bound charges the four unobserved minutes as failures (45 minutes); the upper bound assumes they succeeded (41 minutes). Keep the full 43,200 eligible minutes as the denominator in both cases. Do not drop the missing minutes from the denominator.
- Compare the unrounded rates with 99.9% when deciding the target status; the displayed five-decimal figures are for the cover. Daniel, please carry both limits and the four unobserved minutes forward together.
