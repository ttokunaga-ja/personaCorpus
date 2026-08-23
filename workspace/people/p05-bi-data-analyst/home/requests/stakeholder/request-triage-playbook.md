# Stakeholder request triage playbook

## Purpose

Use this playbook to turn an analytics request into an owned, auditable response without treating an email thread as the system of record. It applies to Mosaic Retail Analytics stakeholder work such as dashboard definitions, variance explanations, and reporting-cut questions.

## 1. Intake and classify

Capture a request ID, requester, received time, requested outcome, affected metric/report, business deadline, and named owner. Classify as one of:

1. **Definition or policy:** formula, inclusion/exclusion, grain, currency, cutoff, or comparison basis.
2. **Data quality or reconciliation:** a total, source extract, hierarchy, or refresh does not tie.
3. **Presentation:** label, chart, drill-through, access, or wording change with no metric change.
4. **New analysis:** a question that needs a new cut, hypothesis, or decision support.

Acknowledge receipt within 4 business hours. State the target response time, even when the request is incomplete.

## 2. Set the response path

| Class | Minimum evidence | Required reviewer | Default SLA |
|---|---|---|---|
| Definition or policy | Formula, grain, cutoff, inclusions/exclusions, tie-out | Finance Business Partner and business owner | 1 business day |
| Data quality or reconciliation | Source snapshot, expected/actual delta, reproducible tie-out | Data steward; Finance if financial | 1 business day to isolate |
| Presentation | Screenshot/mockup and unaffected-metric confirmation | Report owner | 2 business days |
| New analysis | Question, decision use, dimensions, due date | Requesting business owner | Scope within 1 business day |

If the request would change a published formula, do not patch the dashboard first. Create a decision record, preserve the prior definition, and obtain the required approval.

## 3. Verify before answering

Use the stated reporting cutoff. Reconcile the headline number through at least one independent breakdown when practical (for example channel, region, or category). Record source names, refresh time, rounding treatment, and known exclusions. A result that ties only after unexplained manual adjustment is a data-quality incident, not a closed request.

## 4. Communicate and approve

Write a short response with the decision, evidence, owner, due date, and next action. For a definition request, include the exact dashboard label and tooltip language. Ask the business owner to approve or reject the specific wording; Finance concurrence is required where a financial metric, currency, cutoff, or comparison baseline is affected.

## 5. Close with controls

Update the request register with status, approver, approval timestamp, and a file/link reference to the published change. Verify the production wording after refresh. Close only when the requester’s decision need is met and required evidence is attached. Reopen if a later cutoff, source, or formula changes the conclusion.

## Escalation triggers

Escalate immediately for a metric definition conflict, material unexplained financial variance, late-close request, disclosure risk, or a request to overwrite history without a documented decision. State what is known, what is blocked, the decision needed, and the time impact.
