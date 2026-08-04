# yy

**A time tracker you can trust.**

You start a task, you stop it, and at the end of the month you can prove where
the hours went. Everything stays on your machine, and your data can always leave
in a format you can read with your eyes.

> **Status: early development.** The design is settled and written down; the
> code is not there yet. If you are looking for something to use today, this is
> not it. If you are looking at how it is being built, start with
> [the design of record](docs/DESIGN.md).

## What it looks like

```sh
yy start "writing the storage layer"   # begin
yy status                              # what am I on, how long
yy stop                                # end
yy today                               # today's entries and the total
```

That is the whole of version one. A terminal UI, a browser view, reminders and
idle detection come after, and are designed so they can come after without a
rewrite.

## What makes it different

**Time is computed, never accumulated.** Only a start and an end are stored, so
a crash loses nothing and a forgotten timer is a wrong timestamp you can fix,
not a number you have to believe.

**Nothing is destroyed, so nothing asks "are you sure?".** Every change is
appended to an operation journal, `yy undo` always works, and deleting an entry
appends a deletion rather than removing a row. Confirmation prompts are what you
build when you cannot undo.

**Your data can always leave.** `yy export` produces JSONL — one JSON object per
line, greppable, diffable, editable in any text editor. Export, wipe the
database, import, export again, and the two files are byte-identical. That is a
test, not a promise. Comments you write into the file by hand survive the round
trip, and stay attached to the entry they describe.

**One place holds the rules.** A small local process owns the database; the
command line, the terminal UI and the browser all ask it. They cannot disagree
with each other because none of them decides anything.

**Local, and small.** One SQLite file. No account, no server, no network
listener until you ask for the browser view, and then only on loopback behind a
token.

## Documentation

| | |
|---|---|
| [Design of record](docs/DESIGN.md) | What `yy` is, every decision, and what each one costs |
| [Rules](docs/design/rules.md) | The twelve invariants the rest is checked against |
| [Contributing](CONTRIBUTING.md) | How to build it and how a change gets merged |
| [Jujutsu guide](docs/jj/index.md) | Optional: the VCS the maintainer uses. Git works fine |

The design is the interesting part right now. It states not just what was
decided but what each decision costs, and where a later decision reversed an
earlier one it says so.

## Building

You need a Rust toolchain and a C compiler. Nothing else — no `protoc`, no
Node.js, no `just`.

```sh
git clone https://github.com/tamadamas/yy
cd yy
cargo build
cargo test
```

`rust-toolchain.toml` pins the exact compiler, so rustup fetches the right one
for you. Formatting additionally needs a nightly toolchain; see
[CONTRIBUTING](CONTRIBUTING.md#formatting).

## Built with

[Rust](https://www.rust-lang.org/), [SQLite](https://sqlite.org/),
[jiff](https://github.com/BurntSushi/jiff) for time,
[ratatui](https://ratatui.rs/) for the terminal,
[Topcoat](https://github.com/tokio-rs/topcoat) for the browser view, and
JSON-RPC 2.0 over a Unix socket to hold them together.

## Licence

[MIT](LICENSE).
