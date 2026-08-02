# yy

**A time tracker you can trust.**

You start a task, you stop it, and at the end of the month you can prove where
the hours went. Everything stays on your machine, and your data can always leave
in a format you can read with your eyes.

> **Early development.** The design is settled and written down; the code is
> not there yet. The pages below are written as they are implemented.

## What this book contains

**The guide** is for using `yy`: installing it, tracking a day, correcting a day
you got wrong, and getting your data back out.

**The specification** is for anyone who wants to read `yy`'s data or talk to it
without using `yy`. Both formats are plain JSON on purpose: the export is JSONL,
one object per line, and the protocol is JSON-RPC 2.0 over a Unix socket. A
working client is about fifty lines in any language, and you do not need Rust to
write one.

**Contributing** covers building the project and how a change gets merged.

## The guarantees

These are the things `yy` promises, each backed by a test rather than an
intention:

- **Time is computed, never accumulated.** Only a start and an end are stored, so
  a crash loses nothing and a forgotten timer is a wrong timestamp you can fix.
- **Nothing is destroyed**, so nothing asks "are you sure?". Every change is
  appended to a journal and `yy undo` always works.
- **Your data can always leave**, losslessly. Export, wipe the database, import,
  export again: the two files are byte-identical, and hand-written comments
  survive in place.
- **One place holds the rules.** Every front-end asks the same host, so they
  cannot disagree.

The reasoning behind each is in the [design of record](design.md), which states
not only what was decided but what it cost.
