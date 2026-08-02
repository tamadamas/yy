---
name: commit
description: Always use this skill before authoring a commit message in the yy repository
---

# Authoring Commit Messages

`yy` follows [Conventional Commits](https://www.conventionalcommits.org/).
`.github/workflows/semantic-pr.yml` enforces the format on pull request titles,
and pull requests are squash-merged, so **the title becomes the commit on
`main`** and the changelog entry. Keep commits and the title consistent.

## Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Only the header is required.

**Type** -- one of `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`,
`build`, `ci`, `chore`, `revert`. Only `feat` and `fix` are user-facing and
appear in the changelog. Pick deliberately: version bumps are derived from the
type (`feat` minor, `fix` patch, breaking major).

**Scope** -- optional. A crate name with the `yy-` prefix dropped (`core`,
`store`, `host`, `cli`, `tui`, `types`, `client`), or `design` / `jj` for
documentation. Omit it when the change spans crates or the type says enough
(`chore: bump dependencies`).

**Subject** -- imperative present tense ("add", not "added"), lowercase first
letter, no trailing period, short.

**Body** -- add one when the "what" or the "why" is not obvious from the
subject. State the motivation and contrast it with the previous behaviour.

**Footer** -- `Closes #123` for issues. For a breaking change add `!` after the
type or scope and end with:

```
BREAKING CHANGE: <what breaks and how to migrate>
```

## Design changes

If the change alters a decision in `docs/design/`, say so in the body and name
the section. `docs(design): ...` is the right type when the document is all that
changed; when code and design change together, the type follows the code.

## Examples

```
fix(store): keep comments anchored when re-exporting
feat(host): resync subscribers instead of buffering updates
docs(design): record why the protocol is JSON-RPC rather than gRPC
chore(release): v0.2.0
```

## Be succinct

State what changed and why. Skip restated context and throat-clearing.
