---
path: Documents/長文点検記録分類/Cost-discussions/Can we price only the extra tokens.md
kind: text
author: Brandon Romero
derived_from:
---

# Can we price only the extra tokens?

October 17, 2025 — notes for Vanessa

We set aside the shortcut of pricing the October difference by multiplying the first pass cost by 2,560 / 2,048. Those numbers are body-token ceilings, not the number added to every record. For a body of 2,048 tokens or less both passes take the complete body once: there are no extra body tokens. For a body longer than 2,560, the first pass takes 2,048 and the two-end pass takes 2,560, an extra 512. Between those lengths the extra amount varies: at 2,049 it is one token; at 2,560 it is 512. Common control tokens sit outside these body budgets.

Even with the same hardware and parallelism, host processing and input preparation include work that cannot be assumed to rise in direct proportion to body length. The two-end pass also selects and joins segments on longer records. We need to measure the same boundary rather than apply a token ratio to the whole cost.

The October table covers one pass over the same 480 records at the same unit prices. On an index where first 2,048 totals 100, accelerator is 80 versus 98, host processing 12 versus 12, and input preparation 8 versus 8: totals 100 and 118. That 18% increase belongs to the two measured conditions, first 2,048 and first 1,536 plus last 1,024. Both placement and budget changed; it is not a price for placement alone.

For the next comparison I will measure costs under the same boundary and unit prices for the two matched budgets: first 2,048 versus first 1,536 plus last 512, and first 2,560 versus first 1,536 plus last 1,024. We have not measured the additional conditions yet. Vanessa and I are keeping the application decision open until those results and the corresponding missed-record readings are available.
