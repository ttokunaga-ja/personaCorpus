# January 30 — dropping the visible copies

After talking with Fatima: I cannot rescue the January score by removing only the 84 documents we found. They are the documents in the 24 currently identified crossing cases, not a proof that every remaining case is confined to one partition. The old allocation was made by document; deleting the visible group leaves that mechanism untouched and leaves the rest unverified.

Removing them only from evaluation is weaker still. The fitted classifier has already seen the training documents; changing the evaluation list does not undo that training or establish independence of the remaining evaluation cases. Removing all 84 from every partition and retraining would address those known cases but still leave the same uncertainty about unrecognized crossings.

Hold this proposal. We need an evidence-backed way to collect documents by case across channels, allocate whole cases and check the boundaries before scoring. Preserve the label dispositions separately: a partition defect is not a reason to fill the 126 blank Hold labels or discard supported decisions.
