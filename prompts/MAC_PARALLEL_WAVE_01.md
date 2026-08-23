# Mac parallel wave 01

This coordinator handoff launches production; it is not itself a persona
production prompt.

## Open three Local tasks now

Open three separate Codex tasks against this exact directory:

```text
/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus
```

Choose **Local**, never Worktree, and paste one prompt into each task:

1. `prompts/p08-product-manager.md`
2. `prompts/p09-ux-researcher.md`
3. `prompts/p10-management-consultant.md`

Each task owns one persona; its Subagents own distinct leaf scopes inside that
persona. A task may see leases for the other two personas and must treat them as
normal parallel state. It stops only if its own parent lease is already held or
its own preflight fails. No task commits or pushes.

## Optional fourth Local task

After this Full foundation commit is present, `prompts/p01-full-pilot.md` may be
opened as a fourth Local task. It owns only p01 and adds exactly 12 files to
`p01-primary-01`; it does not allocate or create the remaining 11,788 p01 Full
files. Run it concurrently only while the Mac remains responsive enough for
document rendering and visual QA.

## Coordinator wave end

Individual tasks report their own lease release and do not require global
`active_leases=0`. After all intended tasks have stopped, the coordinator runs
`./bin/check-ready`, reconciles each report and manifest, and only then expects
global `active_leases=0`.
