# Rust workspace layout

`workspace/` was created once by the accepted Rust
`kio-eval persona scaffold` command. Runbooks and production tasks do not
create, complete, copy, adopt, or reinterpret it.

```text
personaCorpus/workspace/
  persona-plan.json
  persona-workspace-owner.json
  _control/
    personas/
      <persona-id>/
    scopes/
      <persona-id>/
        <rust-scope-id>/
  people/
    <persona-id>-<role>/
      home/
        <scope-path>/
```

The exact persona IDs, role slugs, Rust scope IDs, and `scope-path` values are
read from the accepted plan; the illustrative names above are not a second
schema. `persona-plan.json` and `persona-workspace-owner.json` are Rust-owned
records and must not be edited.

`_control/`, `_control/personas/`, `_control/scopes/`, and each
`_control/scopes/<persona-id>/` are sealed routing directories. The direct
persona and scope leaf directories beneath them are mutable only for the
Rust lease implementation. No additional operator-managed record hierarchy
is created by the scaffold or required for production.

For a plan row, final content belongs only in the corresponding
`people/<persona-id>-<role>/home/<scope-path>/`. The scope's control directory
is selected by its separate Rust scope ID:

```bash
./bin/persona lease scope claim \
  --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
  --persona <persona-id> \
  --scope-id <rust-scope-id> \
  --parent-session <parent-session> --worker-session <worker-session>
```

The lease is duplicate-writer coordination, not a semantic plan parser or
authorization for Kio replay. Rust `persona attest` separately observes the
exact materialized artifacts and makes no Kio/history claim.
