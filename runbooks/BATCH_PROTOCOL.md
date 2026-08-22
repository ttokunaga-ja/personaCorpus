# Batch production protocol

## Authority and paths

The coordinator has generated plan, schedule, render, materialization, and
workspace through `kio-eval persona`. Do not regenerate them. The accepted
Rust plan is the sole authority
for the persona ID, Rust `scope_id`, its distinct `scope_path`, and all file
allocation. The final-output directory is exactly:

```text
/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/<persona-id>-<role>/home/<scope-path>/
```

Never use a home path as a lease identifier and never derive an ID from it. The
Rust lease interface takes `--scope-id <Rust scope id>` and derives all
workspace-owner binding itself; it accepts no caller-supplied record digest.

## Batch lifecycle

1. The parent reads the accepted plan row and chooses one persona and one
   unleased Rust scope ID. It preserves the plan's required home path and
   allocation; this protocol adds no semantic allocation or manifest.
2. The parent obtains one persona coordination lease:

   ```bash
   ./bin/persona lease claim \
     --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
     --persona p01 \
     --session <parent-session>
   ```

3. Immediately before assigning a worker, the parent claims the exact Rust
   scope ID. The returned token remains with the parent.

   ```bash
   ./bin/persona lease scope claim \
     --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
     --persona p01 --scope-id <rust-scope-id> \
     --parent-session <parent-session> --worker-session <worker-session>
   ```

4. The worker produces one bounded first-milestone batch of 4–20 assigned
   artifacts (later batches normally use 5–20) and writes
   final artifacts only below the plan row's `home/<scope-path>/`. It does not
   write `_control/`, alter Rust records, change the allocation, or receive a
   release token. Route DOCX/XLSX/PPTX/PDF/image creation through the matching
   skill and inspect the final artifact before reporting it.
5. The parent validates the work against the plan row, records any operational
   checkpoint in its own process, then releases the scope using its Rust scope
   ID and returned token:

   ```bash
   ./bin/persona lease scope release \
     --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
     --persona p01 --scope-id <rust-scope-id> \
     --parent-session <parent-session> --release-token <scope-release-token>
   ```

6. Release the parent lease only after all child scope leases are absent:
   `./bin/persona lease release --root
   /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace --persona p01
   --release-token <parent-release-token>`. Interrupted work is inspected with
   `lease show` / `lease scope show`. `lease recover` / `lease scope recover`
   are explicit trusted-coordinator actions after confirmation that the named
   writer stopped. After the lifecycle, `kio-eval persona attest --root
   <materialized-root> --out <new-report>` may publish a create-only bounded
   observation; its Kio/history claims remain false.

For the first milestone, a scope's target count is its full-plan
`scope.raw_files` multiplied by 200 and divided by that persona's
`persona.raw_files`. The accepted full plan makes every result an integer from
4 through 20 and the twenty scope targets sum to 200. Each format target is the
persona brief's integer percentage multiplied by two, so its fifteen format
targets also sum to 200.

Before spawning workers, the parent writes a token-free operational assignment
at `progress/<persona-id>/m1-assignments.md`. It lists all twenty Rust scope
IDs, exact final folders, target counts, and fixed filenames/formats. The
brief's named seed artifact occupies the matching primary scope first; the
remaining entries complete the scope and format totals. This checkpoint is a
persona-owned work allocation, not a replacement plan or canonical manifest.
The parent passes each worker only that worker's fixed scope subset.

Lease state is a duplicate-writer coordination aid, not a privilege boundary
against another process under the same OS account. It is not evidence of Kio
prepare/index/replay, chunks, search correctness, or history readiness.
