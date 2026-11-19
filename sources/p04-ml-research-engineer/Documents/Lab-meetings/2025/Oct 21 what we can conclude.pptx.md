---
path: Documents/Lab-meetings/2025/Oct 21 what we can conclude.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---
# Slide 1
Layout: Cover

Title: What can we conclude from the inspection windows?

MDR-LDI-20251021-LM-01-R01 | Issued: October 21, 2025 | Author: Vanessa Nguyen

Speaker notes: For Michael, Fatima and Emily. Start with the paired comparison, then ask what needs to be matched before an adoption decision.

# Slide 2
Layout: Table

Title: The same 480 records, paired

| First 2,048 | First 1,536 + last 1,024: miss | First 1,536 + last 1,024: no miss | Total |
|---|---:|---:|---:|
| Miss | 9 | 28 | 37 |
| No miss | 2 | 141 | 143 |
| Total | 11 | 169 | 180 |

Table note: Fixed evaluation: 480 records, including 180 with action-required references. A miss predicts one of the other four classes for an action-required record. Other errors remain in the eight-class evaluation.

Speaker notes: These are the same records under both windows. The ends window recovers 28 first-only misses and introduces two; nine remain missed by both. The net change is 37 to 11, not 28 to zero. This is separate from the short-intake population.

# Slide 3
Layout: Figure

Title: Where the missed evidence fell

Image: `Pictures/Charts/Where the missed evidence fell.png`

Caption: Effective evidence endpoint in full body; records missed by either window, n = 39. Final quartile: 23; two ends-only misses fall in the third quartile.

Speaker notes: This is a distribution within the 39 records missed under at least one condition, not a distribution over all 480. Position of evidence is a reading observation, not proof of a model's reason for its prediction.

# Slide 4
Layout: Two Columns

Title: Two records newly missed by the ends window

Left: Repair instruction in the middle

The opening describes the condition and the ending closes the record. The operative repair instruction is in the omitted middle; the action itself is lost.

Right: Reinspection deadline condition in the middle

A condition governing the deadline is omitted. The remaining instruction cannot be interpreted correctly without that qualifier.

Speaker notes: Both are fixed-evaluation records, distinct from the August development readings. Inspect each prediction and the complete effective phrase separately in the next comparison; recovering one does not establish recovery of the other.

# Slide 5
Layout: Table

Title: What the October cost measures

| One pass over the same 480 records | First 2,048 | First 1,536 + last 1,024 |
|---|---:|---:|
| Accelerator | 80 | 98 |
| Host processing | 12 | 12 |
| Input preparation | 8 | 8 |
| Relative total | 100 | 118 |
| Misses | 37 | 11 |

Table note: Same equipment, parallelism and unit prices. The 18% increase belongs to these two measured conditions.

Speaker notes: Brandon and I checked the measurement boundary on October 14. Both location and body-token budget changed, so this comparison cannot isolate the effect of location. It is a relative cost for one pass, not a monthly estimate.

# Slide 6
Layout: Table

Title: Compare location within each budget

| Body-token budget | Opening only | Opening and end |
|---|---|---|
| 2,048 | First 2,048 | First 1,536 + last 512 |
| 2,560 | First 2,560 | First 1,536 + last 1,024 |

Table note: Same fixed evaluation, references, model and tokenization. Keep order and avoid overlapping input.

Speaker notes: The first 2,048 and the 1,536 + 1,024 conditions have been measured; the other two conditions still need measurement. Ask whether these paired contrasts answer the location question and whether the 512-token end leaves enough useful evidence.

# Slide 7
Layout: Heading and Body

Title: Take back from the meeting

- Emily: check input construction for both budgets, including short records and the join.
- Fatima and Vanessa: compare misses in both directions by effective evidence position; read the two middle-evidence records individually.
- Brandon: measure each pass at the same boundary and unit prices.
- Question for the group: under which matched budget does an ends window improve the trade-off without hiding new losses?
- Application adoption remains on hold until the additional comparisons are reviewed.

Speaker notes: Seek agreement on the comparisons and the evidence we need, not a decision to adopt either window today. Report both reversals and cost alongside the conditions that produced them.
