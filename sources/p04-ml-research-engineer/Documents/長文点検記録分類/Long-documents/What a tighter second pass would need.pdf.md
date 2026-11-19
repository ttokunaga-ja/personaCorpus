---
path: Documents/長文点検記録分類/Long-documents/What a tighter second pass would need.pdf
kind: tex:mercer-research-brief.cls
author: Emily Chen
derived_from:
---
# What a tighter second pass would need

Document ID: MDR-LDI-20260818-RP-01-R01  
Issued: August 18, 2026  
Author: Emily Chen  
To: Vanessa Nguyen and Fatima Rahman

## Question

Can we select fewer records for a second inference and still obtain a useful reduction in action-required misses? The August comparison supports a further experiment, not a claim that a smaller second-pass group will retain its result. I propose that we fix the selection rule before looking at outcomes in another independent series.

## Cost room, with a proportional assumption

At 100,000 original records per month the first-pass-only estimate is USD 3,600, including USD 600 monthly standby monitoring. Against the USD 4,500 ceiling that leaves USD 900. The measured 25% second-pass condition adds USD 1,200, giving USD 4,800. Its estimate comes from three runs of 10,000 original records each, with 2,500 reread per run; both passes are included in the measured host and accelerator time.

If the additional cost scales directly with the proportion reread, the estimate is 3,600 + 1,200 × (rate / 25%). The USD 900 room corresponds to 18.75%. This is a sensitivity calculation, not a measured operating threshold: changing the selection may change the workload per reread, and arrivals, reruns and recovery work have not been established. Neither cost nor accuracy at 18.75% has been measured.

## Comparison to fix in advance

On the August evaluation of 4,000 original records, the existing rule reread 1,000 records whose untruncated body exceeded 3,200 tokens. It replaced the first prediction with a second prediction from the first 1,536 and last 1,024 body tokens. The first-only condition used up to the first 2,048 body tokens. Action-required misses were 32/1,000 and 24/1,000 respectively; 14 were rescued but six were newly missed. False alarms among 3,000 non-action records rose from 90 to 103. These are results for the existing rule, not for a narrower one.

Before examining the new series, Vanessa, Fatima and I should agree on an observable selection rule that targets a smaller reread share, document its exact boundary and apply it without using reference labels or first-pass outcomes to tune it on that series. Keep the model, tokenizer, input boundary and reference labels common across conditions. Record actual selected counts and their denominator; the share need not equal 18.75% exactly.

Run first-only, the existing length rule and the fixed narrower rule on the same new series. Report action-required support and misses for each, paired rescues and newly missed records against first-only, and false alarms with the non-action denominator. Read the new misses against the signed source text. In the six already examined, four lost a middle condition or instruction from the second window, while two retained the evidence but favored an old disposition. A narrower length-based selection cannot be presumed to remove either failure.

## Before considering use

Brandon should measure the selected workload and timing at the agreed boundary; proportional cost is not a substitute. Peak response and the person and work needed to recover from a wrong routing remain open. We should not promise the 2.4% miss rate at the smaller share, or change the current human review route on the basis of this proposal.
