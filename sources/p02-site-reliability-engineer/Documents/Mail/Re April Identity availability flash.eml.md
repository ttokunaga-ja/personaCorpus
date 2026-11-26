---
path: Documents/Mail/Re April Identity availability flash.eml
kind: text
author: Emma Wilkinson
derived_from:
---

From: Emma Wilkinson <emma@brt-booking.example>
To: Daniel Jennings <daniel@brt-booking.example>, Anjali Shah <anjali@brt-booking.example>
Cc:
Date: Thu, 2 May 2024 10:47:00 +0100
Subject: Re: April Identity availability flash
Message-ID: <49d72721-92ca-44e9-8e9e-83769c857470@brt-booking.example>
In-Reply-To: <a47e37a1-4fc3-40c1-b5d5-9d6344730a71@brt-booking.example>
References: <a47e37a1-4fc3-40c1-b5d5-9d6344730a71@brt-booking.example>

Hi Daniel and Anjali,

I've attached the separately retained verifier aggregate for the missing tail. It has a row for each scheduled minute. Nine slots from 23:42 to 23:51 UTC have a matched completed success, as do the five from 23:55 to 00:00 UTC. The end of each interval is exclusive.

The four slots from 23:51 to 23:55 have no matched outcome in this extract. Daniel, please leave those open while you check the evidence; the successes on either side don't tell us what happened in the hole.

Emma

On Wed, 1 May 2024 at 10:31 BST, Daniel Jennings wrote:
> Hi Emma and Anjali,
>
> Attached is the April Passenger Identity availability flash. It reports 99.86343% against the 99.9% target on the full 43,200-minute UTC month.
>
> The 59 minutes charged include 41 confirmed failed minutes and 18 minutes for which the primary collector has no outcome. I have provisionally counted all 18 as downtime; that is not the same as having observed 18 failures.
>
> Emma, could you look for a separately retained minute-by-minute sign-in result for 30 April 23:42 to 1 May 00:00 UTC? If we can match completed outcomes to the scheduled minutes, we can revisit the charge before closing the report. Please include any minutes for which no outcome can be matched.
>
> Thanks,
> Daniel
>
> Attachment: April Identity availability - Flash.pdf

Attachment: identity_verifier_apr30.csv
