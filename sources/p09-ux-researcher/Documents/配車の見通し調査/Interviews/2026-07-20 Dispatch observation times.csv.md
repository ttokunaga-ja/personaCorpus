---
path: Documents/配車の見通し調査/Interviews/2026-07-20 Dispatch observation times.csv
kind: text
author: Jennifer Salazar
derived_from:
---

# Dispatch observation times

Jennifer Salazar to Brian Chen · July 20, 2026 · Research team only

Count distinct interview IDs for participants and use interview ID plus session offset to locate individual scenes. There are 24 participants and 26 scenes. The three rows for interview 19 all come from Adam's walkthrough. Session offsets point to the transcript passage and can refer to a participant's retelling of an earlier shift. Playback begins one minute into the interview. A blank playback offset means the temporary audio was deleted after transcript checking. Check the recording register for the keep-through date before re-listening.

The CSV contains the following header and 26 scene rows. The counts and reading notes above are outside the CSV.

```csv
Interview order,Interview ID,Session start (CDT),Route,Session offset (mm:ss),Playback offset (mm:ss),Scene type,Evidence type,Action or account,Hold reason pathway,Transcript file
1,UX-166,2026-07-06 09:00,R-352,12:50,,Reason established,Participant account,Set yesterday's slip aside and read today's slip for R-352,Today's handoff slip identified the pending loading confirmation,2026-07-06 UX-166 Pay attention to the date on the slip.txt
2,UX-167,2026-07-06 11:00,R-373,24:15,,Reason unconfirmed,Participant account,Asked the vehicle coordinator and got the van type,Reply confirmed the type only. The replacement decision and hold reason were still unconfirmed.,2026-07-06 UX-167 The coordinator says which van.txt
3,UX-168,2026-07-06 14:00,R-362,16:10,15:10,Reason established,Participant account,Read past the stop-order correction and matched the separate notebook entry by route number,The R-362 entry identified a pending loading confirmation,2026-07-06 UX-168 Two lines in the handoff notebook.txt
4,UX-169,2026-07-06 17:30,R-367,19:40,,Reason established,Participant account,Asked the previous dispatcher whose reply was due,The previous dispatcher identified the receiving contact as the one expected to reply,2026-07-06 UX-169 折り返しの主語を聞き直した.txt
5,UX-170,2026-07-07 09:00,R-381,15:35,,Reason unconfirmed,Participant account,Checked the message against the two adjacent Held rows and matched it to R-382,The pending confirmation belonged to R-382. The reason for R-381 remained unknown.,2026-07-07 UX-170 The reason belongs to the next route.txt
6,UX-171,2026-07-07 11:00,R-392,14:20,,Reason unconfirmed,Participant account,Read out and passed on the west door instruction for stop 02,The stop Note gave the entrance. The participant still lacked a reason for the hold.,2026-07-07 UX-171 The door note was all she had.txt
7,UX-172,2026-07-07 14:00,R-398,20:30,19:30,Reason established,Participant account,Confirmed R-398 when the yard contact read back the number,The yard contact then said a loading bay had yet to be assigned,2026-07-07 UX-172 The yard call has a route number.txt
8,UX-173,2026-07-07 17:30,R-405,22:05,,Reason established,Walkthrough and participant account,Opened Stops at 04:11 and later explained the two pieces of information separately,The outgoing dispatcher reported the loading wait aloud. The unloading instruction came from the stop row.,2026-07-07 UX-173 停車メモと積込待ちを別に言う.txt
9,UX-174,2026-07-08 09:00,R-408,29:15,,Reason learned after handoff,Participant account,Passed along Held before finding out what the route was waiting for,A later answer identified a pending inventory reconciliation,2026-07-08 UX-174 An answer came after the handoff.txt
10,UX-175,2026-07-08 11:00,R-416,17:10,,Reason established,Participant account,Opened the full message when the preview cut off the route number,The message body linked R-416 to a pending answer about the receiving time,2026-07-08 UX-175 The message preview cut off the route.txt
11,UX-176,2026-07-08 17:30,R-421,18:25,17:25,Reason established,Participant account,Repeated R-421 after getting an answer about a different route,The previous dispatcher confirmed the number and identified the loading supervisor's pending reply,2026-07-08 UX-176 便番号を言い直して分かった待ち先.txt
12,UX-177,2026-07-09 09:00,R-430,13:45,,Reason unconfirmed,Walkthrough and participant explanation,Observed Ready change to Held as Window changed at 12:23. At 13:45 the participant described a possible delay.,The delay was the participant's guess from the two changed cells. Reason source and release authority remained unknown.,2026-07-09 UX-177 Ready changed while she was looking.txt
13,UX-178,2026-07-09 11:00,R-434,21:30,,Reason established,Walkthrough and participant explanation,Reread the numbered handoff entry at 20:55 and explained the scope of the note at 21:30,The outgoing dispatcher's note identified a pending vehicle desk inspection. Release authority was absent from the note.,2026-07-09 UX-178 A clear reason without a release name.txt
14,UX-179,2026-07-09 17:30,R-439,23:40,22:40,Reason established,Participant account,Checked with the previous dispatcher after Window returned to its old value,The previous dispatcher identified a pending callback about the receiving slot. The restored Window did not clear Held.,2026-07-09 UX-179 時間帯が戻っても保留は残った.txt
15,UX-180,2026-07-10 10:00,R-445,12:25,,Reason unconfirmed,Walkthrough and participant explanation,Reread the stop 01 Note at 11:51 and identified the entry gate at 12:25,The participant had no evidence connecting the gate instruction to the hold,2026-07-10 UX-180 The note says which gate only.txt
16,UX-181,2026-07-12 17:30,R-451,26:05,,Reason unconfirmed,Walkthrough and participant account,Reopened the two messages and checked their dates as well as the shared route number,The loading wait belonged to the prior day. Asked the incoming dispatcher to check with the previous dispatcher about today's hold.,2026-07-12 UX-181 積込待ちの伝言が別の日だった.txt
17,UX-182,2026-07-13 10:00,R-458,18:45,,Reason established,Participant account,Read back the route number and time to the receiving desk,The receiving desk confirmed it was still waiting for permission to receive,2026-07-13 UX-182 Agreement from the other desk.txt
18,UX-183,2026-07-13 13:00,R-463,16:30,,No hold encountered,Participant account,Checked the revised Window and said the route stayed Ready throughout the handoff,There was no hold during the period described,2026-07-13 UX-183 The route never went on hold.txt
19,UX-184,2026-07-14 10:00,R-472,12:40,11:40,Reason unconfirmed,Observed in interview,Selected the Held row and opened Stops,Adam could read the state. The reason remained unconfirmed in the interview.,2026-07-14 UX-184 Adam on three held routes.txt
19,UX-184,2026-07-14 10:00,R-476,21:15,20:15,Reason unconfirmed,Observed in interview,"Pointed to Use west loading door at Stops 02 while the selected row showed Held, 13:30–15:00 and Van 18",Adam identified a direction for the stop. The reason remained unconfirmed in the interview.,2026-07-14 UX-184 Adam on three held routes.txt
19,UX-184,2026-07-14 10:00,R-481,32:50,31:50,Reason unconfirmed,Observed in interview,Returned from detail to the same Held row in the route list,Adam still had no reason to pass to the next dispatcher,2026-07-14 UX-184 Adam on three held routes.txt
20,UX-185,2026-07-14 13:00,R-484,11:55,,Reason known before opening list,Participant account,Heard the route number and reason in the handoff and saw Held afterward,The outgoing dispatcher said they were waiting for receiving's response before Mara opened the list,2026-07-14 UX-185 She knew before opening the list.txt
21,UX-186,2026-07-15 10:00,R-489,22:30,,Reason established,Participant account,Read back R-489 on the same call after hearing a wait instruction with no route number,The caller confirmed the route and said the loading list still had to be checked,2026-07-15 UX-186 Read back the number while on the call.txt
22,UX-187,2026-07-15 17:30,R-493,31:10,,Reason unconfirmed,Participant account,Looked for a message on the shared terminal and handed over Held with a request to check with the outgoing dispatcher,Passed along a contact for follow-up. The reason was still unknown.,2026-07-15 UX-187 理由を渡せず確認先だけ残した.txt
23,UX-188,2026-07-16 10:00,R-501,25:35,24:35,Reason established,Participant account,Called the departure coordinator again after the first reply only confirmed the route could not go yet,On the second call the coordinator said they were waiting for a departure bay to become free,2026-07-16 UX-188 The dispatch board and the call.txt
24,UX-189,2026-07-16 17:30,R-509,20:10,,No hold encountered,Participant account,Passed on the corrected scheduled time and reported that R-509 stayed Ready,The handoff covered a time correction and there was no hold during the period described,2026-07-16 UX-189 時刻だけ直した交代.txt
```
