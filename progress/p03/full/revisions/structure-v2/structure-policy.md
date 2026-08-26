# p03 Full structure-v2 policy

Revision `p03-full-structure-v2` is byte-preserving and path-only. It retains each p03 Security GRC canonical scope (`security/*`, `compliance/*`, `privacy/*`, `cloud/*`, `archive/*`, and the other frozen scope roots) and groups Full artifacts beneath natural date/period and work-kind leaves such as `evidence`, `records`, `reports`, `messages`, `documents`, `workbooks`, `presentations`, `images`, `captures`, and `automation`.

M1 remains at its frozen 200 paths. The Full overlay binds every frozen assignment artifact ID to one current structure-v2 path and inherits unchanged bytes/SHA-256 from QA-v2 remediation-after. The mover uses exclusive same-filesystem rename(2), never copy/rewrite. Forty-seven Full exact paths and 89 Full referenced basenames remain compatible; direct-file concentration is bounded at 66.

The 19 invalid JSONL artifacts are a known, separately authorized format-remediation blocker. This revision does not alter their bytes and does not make p03 Git-migration ready.
