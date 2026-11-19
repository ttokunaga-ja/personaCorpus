---
path: Documents/Confidence-abstention/Conditions for the August routing replay.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# Conditions for the August routing replay

Document ID: MDR-CA-20240807-RM-01-R01  
Issued: August 7, 2024  
Author: Vanessa Nguyen  
To: Emily Chen

## Question for the check

Emily, please check the population, boundary and counts with me before we read the queue. This replay asks what a single confidence rule would send for review; it does not route live intake or establish that a reviewed error has been corrected.

## Population and saved predictions

Joshua and Fatima assigned the 4,674 bodies with supported labels under the July revision of the twelve-category guide to training (2,800), development (934) and evaluation (940), by document. We fixed 800 bodies from development for this replay. Each body's saved top prediction and unrounded confidence were obtained from the received body alone. The reference label and prediction remain fixed during routing. The evaluation partition is not used here.

The 126 July Hold bodies have no supported gold label. They are outside all three partitions and outside this scoring population. Their earlier disposition is not a confidence-based review decision.

## One routing condition

Send a body to review when its unrounded confidence is strictly below 0.60. A confidence equal to 0.60 or above goes to the automatic side. Apply this one condition to all 800, without changing the reference or saved prediction. No actual intake is sent onward by this replay.

## Count and denominators

For each body compare its saved top label with the reference, then cross that match or mismatch with the replay destination. Count one body in exactly one cell: match/review, mismatch/review, match/automatic, or mismatch/automatic. The four cells must sum to 800. The two mismatch cells together are all errors before routing.

Error capture is mismatches sent to review divided by all mismatches before routing, not by the review queue or all 800. Review rate is all bodies sent to review divided by 800, irrespective of correctness. Please verify both denominators and read a correct as well as an incorrect prediction from each destination before drawing conclusions about the rule.

References: Short intake annotation guide, July 18; Gold labels before the first score, July 22.
