# Corpus manifests

`corpus-sha256.jsonl` is the deterministic SHA-256 inventory of every accepted
final artifact below `workspace/people/**/home/**`. It contains one header and
one row per artifact, sorted by repository-relative UTF-8 path.

Verify a clone or accepted production wave with:

```bash
./bin/corpus-manifest verify --manifest manifests/corpus-sha256.jsonl
```

Only the coordinator updates this root manifest, after all intended production
tasks have stopped and their lease/ledger/QA gates pass. Persona tasks never
modify it. Full-specific immutable and cumulative manifests live under the
matching `progress/<persona>/full/manifests/` directory.
