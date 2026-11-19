---
path: Documents/Accounts/Aldridge/RFP/Aldridge conditional response.pdf
kind: tex:halstead-rfp-response.cls
author: Michelle Whitaker
derived_from:
---

# Branch Communications: Conditional RFP Response

HBC-ALD-20241023-RFP-01 | Version 1.0 | October 23, 2024  
Prepared by Michelle Whitaker, Halstead Branch Communications Inc  
Prepared for Ahmed Farouk, Aldridge Service Group

## 1. Scope of response

Ahmed, thank you for the opportunity to respond to ASG-RFP-20241008-01 v1.0. The numbered answers below follow your six requirement groups. They address 50 of the 56 requirements. The remaining six are mandatory requirements that the current service or proposed rollout does not meet; each is identified separately in section 3. A possible operating alternative is not a statement that the requirement is met.

Please let us know whether Aldridge can evaluate the proposal with those six exceptions. This technical response does not establish pricing or contract terms, or promise a future feature or delivery date. Commercial terms are for separate discussion.

## 2. Responses by requirement

### 2.1 Sites and directory

- **#1:** Branch Connect maintains a site record with a unique site identifier and name for each branch.
- **#2:** A roster records the people assigned to each site; administrators maintain the roster as assignments change.
- **#3:** An administrator can add roster entries and deactivate entries when people leave.
- **#4:** Each user record includes the user's site assignment.
- **#6:** Inactive status is visible on roster records so administrators can distinguish inactive users from active ones.
- **#7:** A roster record can hold a business telephone number.
- **#8:** A roster record can hold an email address.
- **#9:** Administrators can search roster records by name.
- **#10:** Administrators can filter the roster by site.

### 2.2 Access

- **#11:** Each person uses a named user account.
- **#12:** Users authenticate before accessing the service.
- **#13:** A central administrator can create and deactivate accounts.
- **#15:** Administrative functions are restricted to authorized administrators.
- **#16:** A user can update their own contact details.
- **#17:** An administrator can reset a user's account access.
- **#18:** A user can see their assigned site in their account details.
- **#19:** A central administrator can review account status.
- **#20:** A user can change their password.

### 2.3 Handoffs

- **#21:** A branch can send a handoff message to the selected next duty team.
- **#22:** The handoff identifies the sending branch.
- **#23:** The service records when the handoff message was sent.
- **#25:** The sender selects the recipients of a handoff.
- **#26:** Sent handoff messages are available to authorized recipients.
- **#27:** The sender can include a subject in the handoff.
- **#28:** The sender can review the message after sending it.
- **#29:** An authorized recipient can view the message text.
- **#30:** The sender can include a callback number in the message.

### 2.4 Reporting

- **#31:** A delivery log is available for sent messages.
- **#32:** The delivery log identifies the sending site.
- **#33:** An administrator can view activity by date.
- **#35:** An administrator can export activity records as CSV.
- **#36:** Activity records include the event time.
- **#37:** Activity can be filtered by site.
- **#38:** Activity can be filtered by date range.
- **#39:** An administrator can review the contents of an export.
- **#40:** Delivery records include the message identifier.

### 2.5 Rollout

- **#41:** The implementation plan identifies Halstead and Aldridge tasks and their respective owners.
- **#42:** Before import, Aldridge checks the roster and confirms readiness with the implementation team.
- **#44:** The cutover procedure documents the steps and the return path; the roster owner and return destination are checked before dates are set.
- **#45:** The first site can validate the cutover procedure before the sequence continues.
- **#46:** Subsequent sites are scheduled after the first-site review.
- **#47:** Halstead provides instructions for preparing the roster CSV for import.
- **#48:** The plan identifies an Aldridge contact for rollout coordination.

### 2.6 Support

- **#49:** For service issues, Aldridge can contact the Halstead support team by email at support@halstead-branch.example.
- **#50:** Email that address with the affected site, a contact name and reply address, when the issue occurred, the observed problem and any steps already taken. Do not send passwords. Support logs the issue for follow-up; further details can be supplied in reply to the support correspondence.
- **#52:** After logging the issue, support contacts the requester at the supplied address, identifies the logged issue and confirms the next investigation step or requests missing information. For an issue received overnight, that contact is on the next business day; see exception #51.
- **#53:** Administrator guidance covers account and roster administration and activity exports.
- **#54:** Halstead can provide remote assistance during onboarding to walk administrators through the setup and roster process.
- **#55:** An account access issue can be reported to the same support email address; include the affected user's name and site, without a password.
- **#56:** To follow up, reply to the support correspondence for the open issue, identifying the issue and any change in symptoms. Support can use that correspondence to continue the discussion.

## 3. Mandatory exceptions

| Requirement | Status | Available operating alternative and boundary |
|---|---|---|
| **#5 — Automatic two-way directory synchronization** | Exception; not met | Aldridge maintains its source roster and supplies a CSV for manual import. This is not automatic synchronization in either direction. |
| **#14 — Site-restricted delegated user administration** | Exception; not met | A central administrator can process requests from sites. The service cannot delegate user administration restricted to an individual site. |
| **#24 — Individual acknowledgment of after-hours handoffs** | Exception; not met | The delivery log records sending, not each recipient's acknowledgment. Aldridge could use callbacks as its own process to establish receipt. |
| **#34 — Seven-year in-service activity log retention** | Exception; not met | An administrator can export activity records as CSV. Aldridge would manage the storage location, access and retention period; the service does not provide seven-year retention within the service. |
| **#43 — Single-night cutover of all sites** | Exception; not met | After roster checks, the proposed sequence validates the procedure at one site before proceeding to subsequent sites. We cannot offer all-site cutover in one night. |
| **#51 — Initial response within 30 minutes overnight** | Exception; not met | Issues may be submitted overnight, with contact on the next business day. This does not provide an initial response within 30 minutes overnight. |

Please identify whether each exception can be considered in Aldridge's evaluation, or whether any prevents further consideration. We can discuss the operating boundaries above without changing their exception status.
