---
path: Documents/Clients/Pioneer/Buying/07 Jul - Request and demand evidence.csv
kind: text
author: Connor Atkinson
derived_from:
---

```csv
Request ID,Priority at request,Reason,Request time EDT,Event type,Source time EDT,Source reference,Event detail
EX-011,Urgent,stock needed today,2025-04-16 09:10,Demand record,2025-04-15 14:10,RP-6041,120 cases required 16 Apr
EX-011,Urgent,stock needed today,2025-04-16 09:10,Request received,2025-04-16 09:10,EX-011 request,First evidence received by Buying
EX-023,Urgent,stock needed today,2025-04-03 10:15,Demand amendment,2025-04-03 09:40,DA-403,Additional 60 cases required 03 Apr
EX-023,Urgent,stock needed today,2025-04-03 10:15,Request received,2025-04-03 10:15,EX-023 request,First evidence received by Buying
EX-031,Urgent,supplier delivery changed,2025-05-08 11:20,Supplier notice,2025-05-08 10:45,SN-508,Existing order delivery moved from 08 May to 09 May
EX-031,Urgent,supplier delivery changed,2025-05-08 11:20,Request received,2025-05-08 11:20,EX-031 request,Request follows supplier notice
EX-031,Urgent,supplier delivery changed,2025-05-08 11:20,Resend,2025-05-08 11:35,EX-031 resend,Repeat of same request ID
EX-052,Urgent,stock needed today,2025-06-11 08:42,Replenishment record,2025-06-10 14:20,RP-6118,Required date 11 Jun; quantity to confirm
EX-052,Urgent,stock needed today,2025-06-11 08:42,Request received,2025-06-11 08:42,EX-052 request,First evidence received by Buying
EX-077,Urgent,same-day top-up,2025-06-24 08:31,Forwarded copy,2025-06-24 08:13,EX-077 forwarded copy,Earlier quantity confirmation time not present
EX-077,Urgent,same-day top-up,2025-06-24 08:31,Request submitted,2025-06-24 08:31,EX-077 request,Submission time; earlier demand time not established
EX-088,Urgent,required date changed,2025-06-27 10:05,Request submitted,2025-06-27 10:05,EX-088 request,Required date changed orally; original confirmation time not recovered
```

The extract is at event level from the 01 Apr–30 Jun intake. Repeated IDs, including any resend, are not additional requests; the full intake contains 93 distinct requests. A blank source time is not zero. EX-077's original demand confirmation is not an event evidenced by the forwarded copy. EX-052 quantity remains open at this extract date.
