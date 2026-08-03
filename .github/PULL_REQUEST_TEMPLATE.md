## What this changes and why

Describe the whole diff, not just your last commit. Reference any issue it
closes (`Closes #123`).

## How you verified it

`just check` output, or the manual steps you ran, if `just check` does not
cover the change.

## Checklist

- [ ] `just check` passes locally.
- [ ] The pull request title is a [Conventional Commit](https://www.conventionalcommits.org/)
      (`<type>(<scope>): <subject>`) — CI checks this, and because pull requests
      are squash-merged, the title becomes the commit on `main`.
- [ ] If this contradicts or extends the [design of record](../docs/DESIGN.md),
      the relevant file in `docs/design/` is updated in this pull request, not
      left for later.
- [ ] If this touches the protocol types in `yy-types`, `just schema` has been
      run and the `schema/current.json` diff is included.
