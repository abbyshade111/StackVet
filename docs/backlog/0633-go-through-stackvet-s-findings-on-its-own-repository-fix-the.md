# Go through StackVet's findings on its own repository, fix the rules that are wrong, and set aside the false alarms for the owner to confirm

**Status:** open

From the self-assessment made for stackvet.dev on 22 October 2026 (backlog 0376, `docs/website/self-assessment`):
`sv report .` on this repository, held to level 3, lists 312 things to fix. The owner chose on 22 October 2026 to
build the site's page "StackVet checks itself" with the report as it stands, nothing being live, and to go through
the findings in an item of their own, refreshing the page after.

What the 312 are, by rule:

- 269 *A file is opened at a path built from a value*: `sv` reads the folder it is pointed at, so this is its job.
  Whether the rule can tell a path the person chose at a terminal from one a stranger sends is the question.
- 16 *A shell command is built from a value*, all at `Command::new(...)` in Rust (`adapters.rs`, `production.rs`,
  `review.rs`, `docker.rs`, `probes.rs`). Rust's `std::process::Command` starts a program with its arguments as a
  list and runs no shell, so the rule looks wrong for Rust unless the program is a shell (`sh -c`).
- 9 *A key is made with a method or a size that is no longer approved*, in `crates/sv-check/src/weak_keys.rs`: the
  patterns the check itself looks for.
- 9 *A value that looks like a credential is written into the code*: `crates/sv-check/src/signed_in/fake_app.rs`
  (a test fixture in `src/`, not read as test code), `signed_in/signin.rs`, and `data/breached-password-evidence.json`.
- 2 *Certificate checking is switched off*, in `crates/sv-check/src/cert_checks.rs`, the check's own patterns.
- One each: a disallowed hash (`git.rs`, which reads git's SHA-1 object names), encryption that does not show a
  change (`ssh_format.rs`), an invite code (`secrets.rs`), a query built by joining text (the test sign-in provider,
  `crates/sv-run/assets/oidc-provider.mjs`), a Semgrep React rule on the browser driver, and the bill of materials
  being incomplete.

To do: read each group against the code; fix the rules that are wrong (with a test that the rule still fires where
it should, and stops firing where it should not), and make test fixtures in `src/` read as test code if that is the
cause; write a `[[finding-review]]` entry for each false alarm that remains, for the owner to confirm with
`sv review` (an AI tool's entry never counts until then); then make the report again as
`docs/website/self-assessment/README.md` says, and replace the files there.
