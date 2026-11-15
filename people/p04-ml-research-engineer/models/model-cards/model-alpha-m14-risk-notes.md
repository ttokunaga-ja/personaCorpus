# Model Alpha M-14 risk notes

## Primary risks

Synthetic success can create false confidence. A reviewer may mistake a stable five-label result for broad language understanding, or treat a 0.873 macro F1 as a guarantee for a single prompt. Both conclusions exceed the evidence.

## Controls

- Require abstention or human review for multilingual, adversarial, abbreviated, or out-of-taxonomy text.
- Keep uncertainty examples visible in the review record.
- Re-run dataset assurance when labels, splits, or wording change.
- Do not connect the model evaluation to network, cloud, GPU, or live data services.

## Residual risk

Even with controls, synthetic labels can be interpreted differently by reasonable reviewers. The proper response is taxonomy review and documentation, not retroactive score adjustment.
