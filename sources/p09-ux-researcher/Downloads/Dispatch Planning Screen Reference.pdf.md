---
path: Downloads/Dispatch Planning Screen Reference.pdf
kind: tex:oakridge-screen-reference.cls
author: Christine Wright
derived_from:
---
# Dispatch Planning Screen Reference

ODC-DSP-20240415-REF-01  
Issued: April 15, 2024  
Author: Christine Wright  
Oakridge Dispatch Cooperative · Dispatch Systems

## Reading a route at shift change

The route list at the top gives the current plan for each route. Select a route in that list to see its Stops below. Read the stop-level Note with its stop: it may contain a handling instruction for that location. It is not a route-level explanation of a Held status.

## Screen example: R-214 selected

The diagram shows the route list above the detail for the selected route. The arrow runs from the selected R-214 row to its Stops panel; the Note is part of stop 03, not a field on the route list.

```text
Route list
┌─────────┬─────────────┬───────────────┬─────────┐
│ Route   │ Plan status │ Window        │ Vehicle │
├─────────┼─────────────┼───────────────┼─────────┤
│ R-214   │ Held        │ 14:00–16:00   │ Van 28  │ ← selected
└────┬────┴─────────────┴───────────────┴─────────┘
     │ selection displays details below
     ▼
Stops — R-214
┌──────┬─────────────────────────────────┐
│ Stop │ Note                            │
├──────┼─────────────────────────────────┤
│ 03   │ Use east receiving door         │
└──────┴─────────────────────────────────┘
```

## What each field tells you

| Field | How to read it | What it does not tell you |
|---|---|---|
| Route | Identifies the route whose plan is listed. Select it to view its stops. | Selection alone does not convey the details of every stop. |
| Plan status | Ready or Held shows the route's current status. | There is no dedicated field for the reason for a hold, its reviewer or who can release it. |
| Window | Shows the current planned time window. | It does not show the earlier window. |
| Vehicle | Shows the current vehicle assignment. | The displayed number has no separate confirmation indicator. |
| Stops | Lists stops for the route selected above. | The route list alone does not show the stop instructions. |
| Note | Free text on a stop row; read it with that stop. | A stop Note is not a dedicated hold-reason or reviewer field. |

For R-214 the list shows Held, a 14:00–16:00 window and Van 28. After selecting R-214, stop 03 shows “Use east receiving door.” Check the source of any earlier window or confirmation separately when it matters to the handoff; the current values alone do not supply that history.
