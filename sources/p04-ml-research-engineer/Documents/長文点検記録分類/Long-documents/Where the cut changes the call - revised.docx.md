---
path: Documents/長文点検記録分類/Long-documents/Where the cut changes the call - revised.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
created: 2025-08-22 16:04
derived_from:
---
# Where the cut changes the call

Document ID: MDR-LDI-20250822-RM-01-R02  
Issued: August 29, 2025  
Author: Vanessa Nguyen  
To: Fatima Rahman and Emily Chen

## Purpose and revision

Fatima and Emily, this revision separates what a reader can recover from each input window from what the classifier predicted. Fatima's August 26 notes exposed the ambiguity in my original last column. I have also added a record in which a past repair instruction is quoted at the end. The last mention is not necessarily the instruction in force.

These are development-partition readings, not an estimate of performance on the fixed evaluation partition. On August 21 Fatima and I differed before rereading on 4 of 8 records with end instructions, 2 of 7 with middle evidence, and 1 of 9 with opening evidence. The position of evidence and agreement between readers are separate observations.

The reference is the disposition effective at signature, read from the whole record under Fatima's signed-disposition guide. Below, “retained” means that the phrase supporting that reference is present in the window; it does not mean the classifier used it. Predictions are recorded separately for the same development examples. The first window takes the opening 2,048 body tokens; the other takes the first 1,536 and last 1,024 body tokens, in record order.

## Full-record readings

| Reading | Reference and effective evidence | Context and location |
|---|---|---|
| A — later stop | Remove from service: “Remove the unit from service pending replacement.” | The opening says “Unit may remain in service while checks continue.” The later stop near the end supersedes that permission. |
| B — middle repair | Repair required: “Repair the damaged guard before returning the unit to use.” | The repair direction is in the middle, within the opening 2,048 but beyond the first 1,536. The final paragraph is a signature and does not change the direction. |
| C — opening reinspection | Repeat inspection: “Repeat inspection after the next operating cycle.” | The instruction is in the opening; the ending signs the record without changing it. |
| D — quoted old repair | Repeat inspection: “Repeat inspection after the next operating cycle; no repair is directed at this visit.” | The current instruction is in the middle, within the first 2,048 but beyond the first 1,536. Near the end a history note says “Previous instruction: repair the loose guard before use.” It identifies an earlier instruction, not a new direction at signature. |

## Window readings and predictions

| Reading | First 2,048: evidence retained and reader's assessment | First 2,048: classifier prediction | First 1,536 + last 1,024: evidence retained and reader's assessment | First 1,536 + last 1,024: classifier prediction |
|---|---|---|---|---|
| A | No. The opening permission survives, but the later stop is outside the window; the complete-record answer cannot be supported from this view. | Routine closeout; incorrect against Remove from service. | Yes. The later stop is visible along with the opening permission; the later direction controls. | Remove from service; correct. |
| B | Yes. The repair direction is visible and remains effective at signature. | Repair required; correct. | No. The middle repair direction is absent; the ending only signs the record. | Routine closeout; incorrect against Repair required. |
| C | Yes. The opening reinspection instruction remains visible. | Repeat inspection; correct. | Yes. The same instruction is in the opening portion; the signature does not replace it. | Repeat inspection; correct. |
| D | Yes. The current reinspection instruction and its explicit no-repair direction are visible. | Repeat inspection; correct. | No. The current instruction is missing; the end quotes the previous repair instruction as history. That quotation alone does not establish the action effective at signature. | Repair required; incorrect against Repeat inspection. |

## Interpretation and next reading

A shows why the opening permission can be misleading when the controlling stop falls outside the input. B shows the opposite loss: the middle repair direction is present in the opening window but absent from the two-ended view. In D the end supplies a recognisable repair phrase, but its attribution to a previous instruction matters. The full record's current reinspection instruction controls; a reader of the two-ended view should not promote the quoted repair to the current disposition.

I withdraw any suggestion that showing the end always improves the answer. These windows have different maximum body-token budgets as well as different positions, and these selected examples cannot isolate either effect. A visible supporting phrase is not proof of a correct prediction, nor does an incorrect prediction alone tell us which phrase the model used. Emily, please check the window contents alongside the recorded predictions before we take a comparison forward; Fatima, please check the effective instruction in D against the full record.
