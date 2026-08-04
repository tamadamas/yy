# Security Policy

## Scope

`yy` is a local, single-user time tracker. There is no account and no server
until the browser view is asked for, and then only on loopback behind a token
(see [the design of record](docs/DESIGN.md)). The interesting attack surface is
therefore narrow and specific:

- **The Unix socket** the host process listens on, and the JSON-RPC 2.0
  protocol spoken over it.
- **The SQLite store** and the migrations that run against it.
- **Anything that parses input that did not originate from `yy` itself** — in
  particular JSONL import, and JSON-RPC requests arriving over the socket.
- **The token that guards the loopback browser listener**, once that exists.

General bugs — a wrong total, a UI glitch, a panic on bad input that does not
cross a trust boundary — are not security issues. File those as ordinary
[issues](https://github.com/tamadamas/yy/issues).

## Reporting a vulnerability

Use GitHub's private vulnerability reporting, on the repository's **Security**
tab — "Report a vulnerability". That is deliberately the only channel: this
project publishes no contact address, and a security report should not travel
by a route that leaks it.

Do not put exploit details in an issue or a pull request. Filing publicly is
the one thing this policy asks you not to do.

## What to expect

An acknowledgement within a few days, and an assessment of whether the report
is in scope and how severe it is. There is no fixed disclosure timeline yet —
this is a project with one maintainer, not a security team — but you will hear
back, and you will be credited in the fix unless you ask not to be.

## Supported versions

Nothing has been released yet. Once releases exist, this section will say which
versions receive security fixes; until then, the only supported version is the
tip of `main`.
