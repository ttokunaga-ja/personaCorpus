# Pinned persona controller runtime

- Source repository: `/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/kio`
- Source commit: `1e5d7917e09efc97fab2a9151734d308606b7565`
- Build input: clean `git archive` of that commit
- Rust: `rustc 1.98.0 (88d9e12ae 2026-08-18)`
- Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`
- Profile: `release`
- Local binary: `.runtime/kio-eval`
- Binary bytes: `11,587,120`
- SHA-256: `c45cf65ecaa8cfb68292b9efd97b8cf1b4894b0004219ef4caad473d8c050c67`

The binary is intentionally ignored by Git but is pinned locally through
`config/kio-eval.sha256`. Production tasks invoke it only through
`./bin/persona`, which checks the digest before every command.
