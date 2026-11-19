# February 6 — three ways to split by case

For Fatima and Emily: the January reconciliation gives us 24 known crossing cases to test grouping, not a set on which to estimate model performance.

| Option | What it does | Assessment |
|---|---|---|
| Redraw individual documents | Randomly assigns each document again. | Reject: related documents can cross a boundary again, even with a different seed. |
| Group by case within each intake channel | Keeps records with the same local key together. | Insufficient: only 17 of the 24 known cases return to one group; cross-channel references can remain separated. |
| Group by a cross-channel case key | Links records using evidence across the three channels before allocating whole groups. | Use the key Joshua checked against receipt records or confirmed alias mappings. All 24 known cases reunite in the trial; this is a check on known cases, not proof that all other links are complete. |

Before scoring, check every pair of partitions for zero shared cases and verify that evaluation has reference support for all twelve intents. Resolve unsupported links rather than using similar wording alone as a case identity. If a proposed allocation fails either gate, revise it before the evaluation is opened. Do not let case keys or channel information become classifier input; the task remains classification of the received body.

The size of each partition is a practical constraint, but not a reason to split a case. Fatima, please compare the resulting allocation with the intended evaluation support; Emily, keep feature and setting choices on development data.
