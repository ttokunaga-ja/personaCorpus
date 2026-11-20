# Count the body before capping it — August 5

Vanessa,

In the check I ran, I used the length after cutting the body to 2,048 to decide whether to take the >3,200 route. Nothing entered that route. That says nothing about how many records were actually long enough; the branch was looking at a length that could never exceed the threshold.

Before scoring, compare the original body length with the cutoff, then construct the windows. Please check these two boundary cases against the recorded route:

| Original body tokens | Initial pass | Second pass |
|---:|---|---|
| 3,200 | first 2,048 | none |
| 3,201 | first 2,048 | first 1,536 + last 1,024 |

For the second pass keep the selected spans in their original order. The first prediction and the reference label must not decide the route. I will check the logged original length and pass count before the full scoring run.

Emily
