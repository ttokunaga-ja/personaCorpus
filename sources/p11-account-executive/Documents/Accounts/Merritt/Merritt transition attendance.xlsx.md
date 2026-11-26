---
path: Documents/Accounts/Merritt/Merritt transition attendance.xlsx
kind: halstead-adoption.xlsx
author: Darren Vasquez
derived_from:
---
# Transition attendance

Sheet: Transition attendance  
Document: HBC-MER-20251218-ADP-01  |  Date: December 18, 2025  |  Prepared by: Darren Vasquez  |  For: Michelle Whitaker  
Account: Merritt Equipment Repair Inc  |  Booking: MER-2419  
Attending: Nasrin Rahman, outgoing and incoming contacts, Darren Vasquez and Michelle Whitaker. Times are Denver local time.

| Row | A: Segment | B: Start | C: End | D: Minutes | E: Hours | F: Record |
|---|---|---|---|---|---|---|
| 8 | Segment | Start | End | Minutes | Hours | Record |
| 9 | Outgoing contact explains open callbacks | 13:00 | 13:45 | =(C9-B9)*1440 | =D9/60 | Status of callbacks already started |
| 10 | Incoming contact confirms acceptance | 13:45 | 14:30 | =(C10-B10)*1440 | =D10/60 | Incoming contact checks what is being taken on |
| 11 | Read back outstanding items | 14:30 | 14:45 | =(C11-B11)*1440 | =D11/60 | Read-through before next inquiries |
| 12 | Actual total | | | =SUM(D9:D11) | =SUM(E9:E11) | 105 minutes; 1.75 hours |
| 13 | Booked | 13:00 | 15:30 | =(C13-B13)*1440 | =D13/60 | 2.50 hours reserved |
| 14 | Booked less actual | | | =D13-D12 | =E13-E12 | 0.75 hours not worked |

The three segments are consecutive. The session ended at 14:45; only the 1.75 hours worked are recorded as usage. Booking reference: HBC-MER-20251209-WRK-01.
