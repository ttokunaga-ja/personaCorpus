---
path: Documents/On-call/4 November mock handover observations.csv
kind: text
author: Andrew Cartwright
derived_from:
---

# 4 November mock handover observations

Andrew Cartwright for Daniel Jennings. Session began at 10:00 GMT on 4 November 2025. The CSV records one response per row, with the first destination chosen before coaching.

```csv
Session date,Session start,Zone,Trainee,Decision,Prompt cue,Expected destination,Chosen destination,Outcome
2025-11-04,10:00,GMT,Carys Morgan,1,One payment completed; its receipt is missing,Claire Hutchinson,Matthew Pickering,Incorrect
2025-11-04,10:00,GMT,Carys Morgan,2,Several Fare Payments are failing across the service,Matthew Pickering,Matthew Pickering,Correct
2025-11-04,10:00,GMT,Owen Price,1,Completed payment; customer needs the missing receipt checked,Claire Hutchinson,Claire Hutchinson,Correct
2025-11-04,10:00,GMT,Owen Price,2,A cluster of Fare Payments has failed,Matthew Pickering,Matthew Pickering,Correct
2025-11-04,10:00,GMT,Megan Davies,1,Completed payment; receipt has not arrived,Claire Hutchinson,Claire Hutchinson,Correct
2025-11-04,10:00,GMT,Megan Davies,2,Cluster of failed Fare Payments across the service,Matthew Pickering,Claire Hutchinson,Incorrect
```

Reconciliation outside the CSV: four correct destinations and two errors across six attempts. Carys and Megan each got one of two; Owen got both.
