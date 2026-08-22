# Pinned persona controller runtime

- Source repository: `/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/kio`
- Source commit: `5270a015bdbc83cfc7f7af198de89a11e51508c4`
- Build input: clean `git archive` of that commit
- Rust: `rustc 1.98.0 (88d9e12ae 2026-08-18)`
- Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`
- Profile: `release`
- Local binary: `.runtime/kio-eval`
- Binary bytes: `11,617,728`
- SHA-256: `b20d8591227c745ad0f1f954aa64f3f7678a0ca73f500411f074076d81b3d29f`

The binary is intentionally ignored by Git but is pinned locally through
`config/kio-eval.sha256`. Production tasks invoke it only through
`./bin/persona`, which checks the digest before every command.
