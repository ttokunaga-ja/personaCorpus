---
path: Documents/Customer-updates/Faregate recovery and change freeze.pdf
kind: tex:brt-customer-update.cls
author: Claire Hutchinson
derived_from:
---
# Faregate recovery and change restriction

Reference: BRT-CU-20260713-001  
Issued: 13 Jul 2026  
Prepared by: Claire Hutchinson, Customer Support  
To: Stephen Cartledge, Westbury Coach Cooperative

## What happened

On 13 July, some Faregate payment attempts failed between 08:07 and 08:41 UTC. Service recovery was observed at 08:41 UTC. Our follow-up checks were completed at 08:56 UTC, after three successive five-minute windows without a repeat of the incident failure.

Our reconciled count is 312 failed eligible attempts out of 4,800 eligible attempts during the affected window. This is a service-wide count across the monitored Faregate path; it is not a count of people or a count for Westbury alone. The earlier internal estimate was based on indications before retries and diagnostic activity were separated from eligible failures.

## Guidance for the desk

A missing receipt does not by itself tell you whether the booking or payment succeeded. Please check both the booking and payment outcome, or contact BRT Support for help establishing the outcome, before asking a customer to make a second payment. If the outcome is still unclear, do not ask for another payment until it has been checked.

## Changes to Fare Payments

The service has recovered, but the July error budget calculation through 11:00 UTC on 13 July showed 2.4% of the monthly allowance remaining. To protect the remaining allowance, our Operations team is keeping the existing hold on regular Fare Payments production changes. The next Faregate rollout tranche and the routine payment reconciliation release remain on hold. Investigation outside production can continue; urgent production mitigation would need separate Operations approval.

The queue and balance are due for review on 14 July at 10:00 UTC. That checkpoint does not automatically release the held changes. We will provide further information when a decision is made.

Claire Hutchinson  
Customer Support, BRT Booking Systems Ltd
