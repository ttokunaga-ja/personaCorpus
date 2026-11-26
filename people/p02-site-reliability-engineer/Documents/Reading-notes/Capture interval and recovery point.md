# Capture interval and recovery point

13 September 2024 · Note to self

- **Schedule:** When do we intend to start a capture? Ten minutes is the spacing between due starts in the short-interval run.
- **Write finish:** When did the named write stop being open? Keep that stamp with the write reference. For readback, it gives the time at which the set became complete.
- **Record coverage:** Through what point are the records present and recoverable in an eligible set? Confirm that boundary from the restored records.

On 11 September the 12:20:00 UTC start arrived while W-3184 was open. That write began at 12:10:00 UTC and finished at 12:22:10 UTC, leaving both writes active at the new start and an overlap of 2 minutes 10 seconds before the predecessor finished. The September comparison recorded those clock marks. Restorable record coverage for that window remains unmeasured.

For the next readback, fix the reference on the UTC clock before looking at results. Select from sets complete by that reference whose records can be read back. Take the latest confirmed coverage among those sets and subtract it from the reference. Record the selected set, its completion stamp and its coverage boundary separately. If coverage cannot be confirmed, leave point age unmeasured. Record restore duration separately as the elapsed time from starting the restore to completing it.
