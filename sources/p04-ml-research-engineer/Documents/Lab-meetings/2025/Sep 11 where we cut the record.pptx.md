---
path: Documents/Lab-meetings/2025/Sep 11 where we cut the record.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---
## Slide 1
Layout: Cover

# Where we cut the record

MDR-LDI-20250911-LM-01-R01 | Issued September 11, 2025 | Vanessa Nguyen

Notes: 2:00 p.m. with Fatima Rahman and Emily Chen. Today is about the development readings and the shape of the inputs, not evaluation results.

## Slide 2
Layout: Figure

# Two windows, two body budgets

Figure: draw two aligned horizontal bars for one 4,000-token body, left to right in original order, with token boundaries marked 0, 1,536, 2,048, 2,976 and 4,000. Upper bar: select [1, 2,048] and leave 2,049–4,000 unselected, label “First 2,048”. Lower bar: select [1, 1,536] and [2,977, 4,000], leaving 1,537–2,976 as a gap; label the selected sections “First 1,536” and “Last 1,024”. Caption: “Illustration: 4,000 body tokens. Budgets are 2,048 and 2,560.”

Notes: Emily, confirm the token boundaries and concatenation. The second condition has 512 more body tokens; this is not a same-budget comparison.

## Slide 3
Layout: Two Columns

# What is lost depends on the record

Left: **Later disposition changes the call**

Opening: “Unit may remain in service while checks continue.” Near the end: “Remove the unit from service pending replacement.” Full-record answer: Remove from service. An opening-only cut can retain the permission and lose the controlling stop.

Right: **The middle carries the instruction**

Middle: “Repair the damaged guard before returning the unit to use.” The ending is a signature. Full-record answer: Repair required. A gap between retained ends can remove the instruction even though the signature remains.

Notes: These are development readings. Evidence visible in a window and a classifier's prediction are separate observations. The illustrations do not establish an improvement.

## Slide 4
Layout: Heading and Body

# Before opening the evaluation partition

- How do we concatenate the selected segments in original order?
- If the body is within the window's budget, is every token passed exactly once, including when nominal end segments overlap?
- If it exceeds the budget, are selected segments disjoint and is the middle gap explicit?
- Are tokenizer, model and reference labels fixed across conditions? What common control tokens lie outside the body budget?

Notes: Ask Emily to specify the boundary behavior before the fixed comparison. Fatima and I will keep the full-record reference and evidence location beside each reading. No evaluation result is available yet.
