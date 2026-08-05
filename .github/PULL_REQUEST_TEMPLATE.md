## What this changes and why

Describe the whole diff, not just your last commit. Reference any issue it
closes (`Closes #123`).

## How you verified it

`just check` output, or the manual steps you ran, if `just check` does not
cover the change.

## AI assistance

Name the tool and the model, and say how far it got — questions only, the plan,
or the plan and the implementation. Write "none" if no agent was involved.
[CONTRIBUTING](../CONTRIBUTING.md#before-you-write-code) explains the levels.

## Checklist

- [ ] I have read every change in this pull request, including the ones an AI
      agent made. I understand them and can explain why each one is there.
- [ ] `just check` passes locally.
- [ ] The pull request title is a [Conventional Commit](https://www.conventionalcommits.org/)
      (`<type>(<scope>): <subject>`) — CI checks this, and because pull requests
      are squash-merged, the title becomes the commit on `main`.
- [ ] If this contradicts or extends the [design of record](../docs/DESIGN.md),
      the relevant file in `docs/design/` is updated in this pull request, not
      left for later.
- [ ] If this touches the protocol types in `yy-types`, `just schema` has been
      run and the `schema/current.json` diff is included.
