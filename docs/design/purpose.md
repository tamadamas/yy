# Purpose and the one-day MVP

Part of the [design of record](../DESIGN.md).

## 1. What this is

`yy` records how you spend your working day. You start a task, you stop it, and
at the end of the month you can prove where the hours went.

Two goals, in this order:

1. **A tool used daily.** It must be good enough to track real work within days,
   not months. If it is not usable early, it will never be finished.
2. **A presentable open-source project.** Clear documentation, a stable format,
   a workflow other people can contribute to.

Everything in this plan is judged against those two goals. Anything that serves
neither is cut.

---

## 2. The one-day MVP

The first version must be usable at the end of day one, so that `yy` can track
the work of building `yy`. This is the single most important constraint in the
plan, and it decides what goes into version one and what waits.

Day one delivers exactly this:

```
yy start "writing the storage layer"    # begin
yy status                                # what am I on, how long
yy stop                                  # end
yy today                                 # list of today's entries + total
```

Stored in one SQLite file. No terminal UI, no browser, no synchronization, no
reminders — and, per [§4.7.1](architecture.md#471-and-why-the-process-boundary-arrives-on-day-two),
**no background process and no protocol either**: on day one the command line
links the storage layer directly. Those come later, and the design is arranged
so they can come later without a rewrite.

Realistically this is one focused day if things go well and two if the code
generation setup fights back. That is acceptable. What is not acceptable is
spending two weeks on architecture before the first entry is recorded.
