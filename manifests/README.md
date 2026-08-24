# Corpus manifests

`corpus-sha256.jsonl` is the deterministic SHA-256 inventory of every accepted
final artifact below `workspace/people/**/home/**`. It contains one header and
one row per artifact, sorted by repository-relative UTF-8 path.

Verify a clone or accepted production wave with:

```bash
./bin/corpus-manifest verify --manifest manifests/corpus-sha256.jsonl
```

If unrelated persona production is still active in the same checkout, use
`generate --tracked-only --replace` and `verify --tracked-only` after staging
the accepted artifact paths. This mode binds both artifacts and the canonical
persona plan to their Git-index bytes and excludes untracked in-progress
output. The default mode remains the stricter whole-live-tree check for an idle
checkout or a clean clone.

Only the coordinator updates this root manifest, after all intended production
tasks have stopped and their lease/ledger/QA gates pass. Persona tasks never
modify it. Full-specific immutable and cumulative manifests live under the
matching `progress/<persona>/full/manifests/` directory.
