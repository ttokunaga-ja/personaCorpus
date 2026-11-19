---
path: Downloads/August rehearsal hold points.csv
kind: text
author: Matthew Pickering
derived_from:
---

```csv
Date,Start UTC,End UTC,Worker pool,Workload mix,Target eligible attempts/min,Achieved sustained eligible attempts/min,Stabilised queue eligible attempts,Result
2026-08-17,12:00,12:30,Normal,Fare Payments representative,2400,2400,0,Pass
2026-08-17,12:30,13:00,Normal,Fare Payments representative,2600,2600,0,Pass
2026-08-17,13:00,13:30,Normal,Fare Payments representative,2800,2800,0,Pass
```

The last hold is the highest sustained rate verified by this rehearsal. Each interval is start inclusive and end exclusive. The queue figures are stabilised observations during the respective holds; this exercise does not verify failover capacity or production change safety.
