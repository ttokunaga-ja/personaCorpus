# Identity break-glass validation

## Purpose

Break-glass access is a tightly controlled emergency path for restoring
operations when the ordinary administrator authentication route is unavailable.
This document validates the path without using it for routine work.

## Quarterly validation

1. Schedule the exercise with Identity Platform, Security Operations, and the
   incident-management owner.
2. Confirm the designated emergency approver roster is current in the
   protected access system.
3. In a non-production validation environment, request access using the
   normal dual-approval workflow.
4. Verify that access is time bounded, produces an immutable audit event, and
   is limited to the documented recovery role.
5. Confirm expiry revokes the access automatically and no session remains
   active after the test window.
6. Reconcile requester, approvers, role, start, expiry, and audit event ID.

## Production incident use

The incident commander must record why ordinary administration is unavailable,
which recovery action requires elevated access, and the maximum needed time.
Two independent approvers are required unless the formally documented
emergency exception is invoked. Use the least privileged recovery role and
end the access immediately after the action is verified.

## Validation failures

Page Security Operations if approval is bypassable, audit evidence is absent,
expiry does not occur, or the role grants unrelated administration. Suspend
further exercises until the access-control owner documents a safe correction.

## Evidence hygiene

Store approval references and audit event IDs, not screenshots of access pages,
credentials, recovery codes, or personal contact details.
