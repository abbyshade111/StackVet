# Go through StackVet's findings on its own repository, fix the rules that are wrong, and set aside the false alarms for the owner to confirm

**Status:** claimed by claude/0633, 23 October 2026

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

**The owner's word, 23 October 2026:** "when you get to 0633, can you include consideration of whether the full
stackvet report should be included on stackvet.dev? Looking through it this morning, I'm thinking things like
vulnerabilities shouldn't be advertised." So this item also decides, with the owner, what of StackVet's report on
itself the site shows: the live site serves `docs/website/self-assessment/` as it is (fix-now.md, compliance.md,
your-decisions.md, not-examined.md, before-going-live.md, low-findings.md, seen.json, sbom.cdx.json), each finding with
its file and line in StackVet's own code. Among the choices: a summary only (the counts, what was not examined, and
that the findings are being gone through), the full files behind a note, or the files after triage with what is a real
fault fixed first. Until then the files stay as they are, unless the owner asks for them to come down sooner.

**The owner's answers, 23 October 2026,** asked what stackvet.dev should show of StackVet's report on itself, and
when: "A summary only", and "Now". So the site's "StackVet checks itself" keeps the counts, what could not be examined,
and the bill of materials, says the findings are being gone through, and no longer publishes the finding-by-finding
files; that goes first, ahead of the triage, and the owner publishes it with **Actions → Site**. The triage that follows
is this item's own. **Claimed, 23 October 2026,** for both. The files stay in the repository, where they are public
too: whether they come off `main` is put to the owner, since the history keeps them either way.

**First part, 23 October 2026:** stackvet.dev's "StackVet checks itself" is a summary, read from
`docs/website/self-assessment/summary.md`, with the bill of materials; the finding-by-finding files are removed from
`main` (ADR-132, Later). What remains is the triage itself, and the page refreshed after it.
