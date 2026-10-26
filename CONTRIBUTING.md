# Contributing to StackVet

Thank you for thinking about it. StackVet (`sv`) checks apps against the OWASP standards and says plainly what
was verified and what was not, so the one thing a change must never do is make the report claim more than it
knows. Most of what follows comes from that.

By taking part you agree to the [code of conduct](CODE_OF_CONDUCT.md). A problem that could be a security hole in
`sv` itself goes through the private route in [SECURITY.md](SECURITY.md), never a public issue.

## Before you start

- **Look at [docs/BACKLOG.md](docs/BACKLOG.md)** for what is planned (each item is a file of its own under
  `docs/backlog/`), and **claim the item you are taking** with `python3 tools/backlog.py claim <number> --by <you>`,
  which sets its status line, in a commit of its own, merged before any other work ([ADR-061](docs/adr/ADR-061.md)).
  Several people and AI sessions work here at once, and a claim written into the repository is the only one
  everybody sees. A message is not a claim. When the work lands, `python3 tools/backlog.py done <number>` marks it.
- **For anything larger than a fix, open an issue first** (the "Idea" template), so the design can be agreed
  before the code is written.
- **Read the design entries for whatever you touch.** Each is a file of its own under `docs/design/`
  ([ADR-060](docs/adr/ADR-060.md); [docs/DESIGN.md](docs/DESIGN.md) says how they are named and found). They say
  why things are the way they are, including the ways they were wrong before.

## Building and testing

You need Rust 1.95 or newer. From the repository's top folder:

```bash
cargo fmt --all --check                    # formatting
cargo clippy --all-targets -- -D warnings  # lints, with every warning an error
cargo test --workspace                     # every test
```

These are exactly what CI runs, and all three must pass. A few more things to know:

- **Tests that start an app need Docker** (or Colima). Without it, they check that `sv` says plainly it could
  not run them, and they print which of the two they did.
- **`docs/COVERAGE.md`, `docs/REQUIREMENTS.md`, and `data/reach.json` are generated.** After changing a rule, a
  map from another tool's rules, or a requirement a check cites, run `python3 tools/coverage.py` to regenerate all
  three; never edit them by hand. A test fails while any of them is out of date.
- **The container** is built with `docker build -t stackvet/sv .`. `python3 tools/image_smoke.py stackvet/sv`
  drives it as an AI coding tool would; CI runs both on every pull request.

## The rules every change keeps

The rules are written in full in one place, [`CLAUDE.md`](CLAUDE.md) ("Rules that hold everywhere" and "Working
style"), which every AI coding session reads at its start and every person contributing follows too. Here they are
in one sentence each, so this page cannot drift from that one; where the two seem to differ, `CLAUDE.md` holds.

- **History is never rewritten** (no force-pushing to `main`, no squashing old commits), since papers and
  documents cite commit hashes here.
- **Evidence is honest.** Something nobody assessed is never a pass and never a failure, and a check is never made
  to look stronger than it is.
- **Break your own rule and watch what catches it.** Disable each guard you add, run the tests, and say in the pull
  request which went red.
- **`sv` opens no network connection of its own**, with four exceptions, drawn and listed in
  [the fence](docs/ARCHITECTURE.md#the-fence-what-can-reach-the-internet-and-what-turns-it-on).
- **Never print, log, echo, or commit a key**, including in a test, where one is built from pieces at run time.
- **A citation is a claim:** cite a requirement only when the check really speaks to it, after reading its text.
- **One app is anonymous in everything public:** `CLAUDE.md` says how to name it.
- **Name the tool in every commit:** an `Assisted-by:` or `Co-Authored-By:` line when an AI tool helped.

Everything a person reads is in plain language, without jargon left unexplained, and in American English with the
Oxford comma ("a, b, and c"); identifiers keep their names. Say what was verified and what was not, in the code's
messages and in your pull request alike.

## Pull requests

Fill in the template: what changed and why, how it was verified, **what was not verified**, and the backlog
entry or issue it closes. Keep one pull request to one change. It can be merged once CI passes.
