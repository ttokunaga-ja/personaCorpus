# Replay sort mistake

December 9, 2024 — for Brandon and me

I sorted a working view with `review_required = true` at the top so I could skim the items that actually take review time. I handed that view to the replay. The count still came back at 600 rows with 90 flagged, which is why my first check missed it. The first row had changed from QD-001 (December 4 at 09:02 PST) to QD-007 (the first review item, at 09:15 PST). The earlier arrivals were still in the list, just out of position.

I've set that run aside. I went back to the fixed list from the December 3 design and checked the entire `arrival_seq` column in the order it was read, then checked the dated arrival against each intake ID. I reran from that list after the check. Use the rerun for the comparison. I can still keep a review-only view for reading the result, as long as it is separate from what feeds the clock.
