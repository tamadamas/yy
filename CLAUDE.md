# CLAUDE.md

Guidance for Claude Code working in this repository. Read this first, every session.

## Project

`yy` is a terminal-first time tracker for one developer's real workday (Rust,
Linux/macOS). Time is tracked against **issues** that receive many **entries**; data is
plain, hand-editable **JSON Lines**; there is a **CLI**, a **TUI** (Tokyo Night), and a
one-shot background reconcile for reminders and idle/sleep detection.

**Status:** planning → building. No code exists yet; the current goal is the **MVP**
(Sprint 0, see below).

## Documentation map

Architecture spec, decision log, sprint plan, out-of-scope list, and private
conventions live in `.local/CLAUDE.md` (gitignored). Read that next.

## Golden rules (invariants — never violate)

1. **Derived time.** Store intervals (`start`/`end`) only. Elapsed is always computed:
   `elapsed = (end ?? now) − start`. Never accumulate seconds in a counter or a running
   process. A crash must lose zero time.
2. **Plain-text & local.** Everything round-trips through hand-editable JSONL. No
   database, no cloud, no accounts, no network.
3. **One-shot, no daemon, no async.** No Tokio in v1. CLI calls, a scheduled `tick`, and
   OS wake/unlock hooks all call the same reconcile routine. No long-lived process.
4. **Preserve, don't drop.** Comments (`#…`) and malformed lines survive every write.
   Reads dispatch on the `t` record-type; unknown `t` is preserved.
5. **Atomic writes.** Write a temp file, then rename. Never write in place.
6. **Core independence.** `core/` must not depend on `cli/` or `tui/`. Tests must run
   without a terminal.
7. **Type discriminator `t` on every record from day one**, even in the MVP.

Out-of-scope list and full stack/conventions detail: see `.local/CLAUDE.md`.

## Commands

```bash
just check                 # check code without building
just build                 # build
cargo run -- <args>         # e.g. cargo run -- start "task" --issue YY-1
just test                   # unit tests (must not require a terminal)
just clippy                 # lint — keep warning-free
just format                 # format
just coverage               # cargo llvm-cov coverage
```

Run the tool as `yy …` once installed (`cargo install --path .`).

## Definition of done (every issue)

- Behavior matches the architecture spec (`.local/CLAUDE.md`); invariants above hold.
- `just check` pass.
- New logic in `store/` and `core/` has tests that don't need a terminal.
- JSONL round-trips: comments and malformed lines are preserved; writes are atomic.
- Check `just coverage` before git commit (command fails below 80%)

Current sprint, backlog, and workflow notes: see `.local/CLAUDE.md`.

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

Rules:

- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).

## Context Navigation (Graphify)

### 3-Layer Query Rule

1. **First:** query `graphify-out/graph.json` or `graphify-out/wiki/index.md`
   to understand code structure and connections
2. **Second:** query the Obsidian vault for decisions, progress, and project context
3. **Third:** only read raw code files when editing
   or when the first two layers don't have the answer

### When to rebuild the graph

- After structural changes (new modules, major refactors)
- Headless: `graphify update .` (only processes modified files)
- Skill: `/graphify . --update` (same behavior, runs through the skill — also accepts `--obsidian` to refresh the vault)
- The graph is persistent — NO need to rebuild every session

### Do NOT

- Don't manually modify files inside `graphify-out/`
- Don't re-read the entire codebase if the graph already has the information

## Git conventions

- Branch naming: feat/short-description, fix/issue-number-description
- Commit format: conventional commits (feat:, fix:, refactor:, chore:, docs:)
- Use git worktree for large features
- Never commit directly to main
- Squash feature branches before merging
