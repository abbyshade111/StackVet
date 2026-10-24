# The starter tells the AI tool to keep the app's data under /tmp, and the app ships with /tmp as its default

**Status:** done, 24 October 2026

Found 23 October 2026, when the owner asked whether StackVet itself makes the trial apps prone to their findings.
Backlog 0648's survey found Bandit's B108 (a fixed path under `/tmp`) 12 times in 8 of 12 apps, each a data folder or a
database defaulting to `/tmp`: `os.environ.get("DATABASE_PATH", "/tmp/...")`.

The cause is `sv`'s own starter manifest (`crates/sv-manifest/src/spec.rs`), whose comments AI coding tools follow as
instructions: "The app's folder is read-only while `sv` runs it, so keep its data somewhere like /tmp." That is true of
the copy `sv` runs, whose container can write only `/tmp` and `/sv-reports` (`crates/sv-run/src/docker/containers.rs`).
But stackvet.toml has no way to set a variable for `sv`'s copy alone (the app is given `PORT` and the addresses of the
helpers `sv` starts), so an app can follow the advice only by making `/tmp` its own default, and that default ships: in
production `/tmp` is shared with every user of the machine and emptied on reboot.

To do, at the owner's word ("yes, file it as 0659 and build it after 0658 please"):

- **stackvet.toml can set variables for `sv`'s copy of the app,** `[stack.run] env = { DATABASE_PATH = "/tmp/app.db" }`,
  passed to the app's container, never to the install step or the tests' host. A value may not carry a credential: a
  key's name or a value that looks like a key is refused, as the manifest refuses one everywhere else (ADR-124).
- **The starter's advice changes** to: keep the data folder configurable by an environment variable, with the default
  the real app should use, and point `sv`'s copy at `/tmp` through `[stack.run] env`.
- **`sv preflight` says** when the start command or the app's code defaults its data to `/tmp`, and how to move it to
  `env`.
- Tests: a variable reaches the app's container and nothing else; a credential is refused; the starter's words; the
  preflight's line both ways. ADR-141, proposed.

**Order,** at the owner's word of 23 October 2026: after 0658.

**Done, 24 October 2026** (ADR-141, accepted). `[stack.run] env` sets variables in the app's container for `sv`'s run;
a name `sv` sets itself, one that says it holds a credential, or one that is not a variable name is refused before the
run, with a plain message. The starter now says to keep the app's data folder configurable, with a default of its own,
and to point `sv`'s copy at /tmp with `env`; `sv preflight` names a path under /tmp in the app's own code and says how
to move it. Tests: `crates/sv-run/src/env_tests.rs` and `crates/sv-cli/src/preflight/data_folder_tests.rs`.
