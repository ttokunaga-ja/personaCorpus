# Model Alpha M-14 Architecture Reading Notes

## Architectural question

Which parts of a compact intent system should be separable enough to review? The answer proposed here is a three-part boundary: input normalization, intent evidence, and abstention decision.

## Component ledger

1. **Normalization** resolves surface variation without deciding intent.
2. **Intent evidence** ranks the five synthetic labels using grouped cues.
3. **Abstention** records when evidence is insufficient rather than forcing a route.

## Reading annotations

*Architecture Boundary Memo* [synthetic/internal] argues that a component earns its place only if its removal has an interpretable failure. *Abstention Interface Sketch* [synthetic/internal] adds that confidence policy must not be mistaken for representation quality.

## Design implication

For the fictional M-14 system, an ablation should report which boundary moved. This is more actionable than a broad claim that the architecture is “better.”
