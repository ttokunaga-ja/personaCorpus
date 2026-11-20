# Can we label the request without the queue?

April 11, 2024 — notes for Michael

Hypothesis: a reader can often identify the action requested from the message itself, without knowing which queue received it. The queue may tell us where the message went, but that is not necessarily the action the sender asked for.

- “Can you move the visit to Thursday?” asks for a change to an appointment.
- “What time is the visit Thursday?” asks for information about its time.

Both contain “visit” and “Thursday.” I expect the verb and the object of the question to be more useful than the frequency of either shared word. A word-frequency baseline may still pick up useful correlations; this pair is a reason to inspect its errors, not proof of an advantage.

What happens when the text supplies no action, or does not identify its object? We need to examine those cases separately rather than infer an answer from the queue. Ask Michael how much context should count as the message, and how independent readers should record their evidence before we call a label correct.
