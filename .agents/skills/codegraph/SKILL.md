---
name: codegraph
description: Use this skill when locating or understanding code in the yy repository and .codegraph/ exists
---

# CodeGraph in yy

`.codegraph/` is indexed for this repository. The tool itself, `codegraph
explore` / `codegraph_explore`, and the rule to reach for it before grep, find,
or Read are covered by the `## CodeGraph` section in your global instructions,
and by the MCP server's own instructions when it is connected — read those, not
this file, for how to call it. If neither is present, `codegraph help` and the
[project README](https://github.com/colbymchenry/codegraph) are the reference;
do not restate either of them here.

What is yy-specific: today the repository is one `src/main.rs`, so plain Read
is still fine — there is nothing for a graph to add yet. The payoff arrives
with the day-one split into `yy-types`, `yy-core`, `yy-store`, `yy-host`,
`yy-client`, `yy-tui`, `yy-cli` ([`AGENTS.md`](../../../AGENTS.md)): tracing a
call path across crate boundaries, e.g. a JSON-RPC method from `yy-cli`
through `yy-client` into `yy-core`, is exactly the "call paths grep can't
follow" case the tool is for.

Use `codegraph impact` / `codegraph affected` to check the blast radius of a
change before touching it — in particular against the invariants in
[`docs/design/rules.md`](../../../docs/design/rules.md), where a violation is
often a crate boundary a normal grep would not show you crossed.
