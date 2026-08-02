# What existing tools already figured out

Part of the [design of record](../DESIGN.md).

## 3. Prior art

Before designing anything, here is what the field has settled on. These are the
lessons `yy` copies deliberately.

**From plain-text trackers ([klog](https://klog.jotaen.net/),
[bartib](https://github.com/nikolassv/bartib), Watson, Timewarrior):**

- The data outlives the program. Every one of these tools stores something you
  can read with your eyes and fix in a text editor. Users pick them *because* of
  that, not despite it. → `yy` must always be able to hand you your data in a
  readable form.
- A file you can copy is a backup, a sync mechanism, and a migration path in
  one, with no code.
- Machine-readable export (`--json`) is what makes a tool scriptable and turns
  users into contributors.
- Timewarrior's lesson specifically: **tags, not a hierarchy.** Rigid
  project trees stop matching reality within a month.
- Bartib's lesson: **resuming the last task must be one command.** It is the
  single most frequent action after a break.

**From terminal UIs (lazygit, k9s, gitui):**

- The real advantage of a terminal UI over a CLI is *discoverability* — you see
  what you can do. People discover features in lazygit they never knew existed
  in git. → The TUI is not a prettier CLI; it is where you learn the tool.
- A fixed set of panels, with the focused one obvious at a glance, beats a
  flexible layout.
- A key-hint bar plus a `?` overlay, always. No hidden shortcuts.
- Do not ask "are you sure?" — see the next point.

**From jujutsu (`jj`):**

- Record every operation, and no operation is dangerous. Undo always works, so
  confirmation prompts become unnecessary friction. This matters *more* for time
  tracking than for version control: lost code usually exists somewhere else,
  but a lost afternoon exists only in your memory, and by Friday it does not.

**From local-first tools (ActivityWatch, atuin):**

- A small local background process with a clear API is a proven pattern for
  "many front-ends, one truth", and it keeps everything on your machine.
- SQLite is the boring, correct choice for local state that several processes
  touch.

**From starship / shell prompts:**

- Anything that runs on every shell prompt has a hard budget of about 10 ms,
  and must degrade to silence rather than block.
