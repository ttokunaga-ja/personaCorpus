# Prototype change log

**Anchor:** Beacon Field Research / Study Alpha / 2026-07-13 / session UX-184 / n=24

## 2026-07-13 - Research handoff baseline

| ID | Area | Change | Why it changed | Validation focus |
|---|---|---|---|---|
| UX-184-01 | Onboarding | Added skip and resumable steps. | Orientation should not become a gate. | Can participants land in the catalog without replaying prior content? |
| UX-184-02 | Search | Set 250 ms debounce and preserved facets on whitespace. | Separate input noise from intentional query changes. | Does a revised query retain useful refinement? |
| UX-184-03 | Filters | Made facet application explicit. | Avoid result movement while a person is still choosing. | Do participants understand when filters take effect? |
| UX-184-04 | Saved studies | Added optimistic save with offline queue state. | Give immediate confirmation without hiding uncertainty. | Can participants explain what will sync later? |
| UX-184-05 | Error recovery | Added retryable timeout at 8 seconds. | Preserve task work when the prototype simulates a delay. | Can participants recover without moderator help? |
| UX-184-06 | Accessibility | Defined focus return, live updates, and 44 px targets. | Keep equivalent paths available in the session prototype. | Keyboard and screen-reader walkthrough. |
| UX-184-07 | Responsive | Added 320 px action stacking and 200% zoom checks. | Protect essential actions in a narrow view. | No horizontal page scroll or hidden primary action. |

## Open questions for the next research review

1. Does explicit Apply improve confidence enough to justify the extra step?
2. Is the saved-for-sync message understood without an account concept?
3. Which result-card metadata helps selection versus adding scan burden?

All scenarios and content in this handoff are synthetic. No participant data, customer data, or production metrics are represented.
