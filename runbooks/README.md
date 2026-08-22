# Operational runbooks

This directory is a local operational copy of the accepted persona-production
guidance from the Kio planning repository. In this standalone project:

- `canonical/` contains create-only canonical planning artifacts.
- `workspace/` is the single shared production workspace.
- `./bin/persona` invokes the pinned Rust `kio-eval persona` interface.
- `runbooks/personas/` contains the twenty creative production briefs.
- `prompts/` contains one ready-to-paste parent-task prompt per persona.

The canonical plan and Rust owner record override prose if they disagree.
Do not use these Markdown files to regenerate, reinterpret, or replace them.
