# Smaller decisions

Part of the [design of record](../DESIGN.md).

## 4.10 Smaller decisions

- **ULID for identifiers** — sortable by creation time, generated without asking
  anyone, and readable enough to type when correcting an entry. Auto-increment
  integers would break as soon as two devices create entries offline.
- **Tags, not a project tree** — see [§3](prior-art.md#3-prior-art). A hierarchy
  has to be decided before the work is understood; tags can be added afterwards.
- **One repository, one Rust workspace** — the message types, the host, and the
  front-ends change together. Splitting them would mean version negotiation
  between your own components. Dependency versions are declared once at the
  workspace root so they cannot drift. This is also the premise that makes
  [§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends)'s
  argument work: there is no independent deployment for a protocol IDL to
  mediate.
- **Topcoat for the browser** — server-rendered Rust, so the web front-end is
  written in the same language as everything else and can call `yy-core`
  in-process instead of over a protocol. Tailwind and the asset pipeline come
  with it, and neither needs Node.js. This single choice is what removes
  TypeScript, a bundler, a client-side state model, and the cross-language
  argument for gRPC. Its cost is that Topcoat is young; see
  [§7](frontends.md#browser-and-phone--topcoat-served-by-the-host).
- **No JavaScript toolchain at all** — with Topcoat there is nothing left for
  Bun or npm to do. `bun`, `node`, and a `clients/web/` directory are all gone.
- **mdBook for the documentation** — Rust toolchain, no JavaScript needed to
  build the docs, publishes to GitHub Pages directly.
- **Jujutsu locally, Git as the contract** — the repository is a colocated
  `jj`/Git workspace. Anyone can clone and contribute with plain Git and never
  notice; the maintainer uses `jj`. See
  [§8.5](repository.md#version-control-jujutsu-and-git) for why this costs
  contributors nothing, and [the jj guide](../jj/index.md) for how to use it.
- **MIT licence, held by "the yy authors"** — a copyright notice does not have
  to name a person for copyright to exist, so the licence carries no individual
  name. MIT rather than a copyleft licence because the crates should stay usable
  from any project, including the Rust ecosystem's overwhelmingly MIT/Apache
  dependencies, and because a permissive licence is the lowest-friction choice
  for contributors. Commercial use is allowed under either; that was never the
  deciding factor.
- **Contexts in the schema from day one** — every entry carries a context
  (`work` by default). The user interface only shows `work` for now. This costs
  one column today and avoids a data migration later.
