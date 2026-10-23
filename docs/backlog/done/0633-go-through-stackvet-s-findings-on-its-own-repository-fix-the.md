# Go through StackVet's findings on its own repository, fix the rules that are wrong, and set aside the false alarms for the owner to confirm

**Status:** done, 23 October 2026

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

**Triage, 23 October 2026.** Every group was read against the code:

- **Rules that are wrong, each now an item of its own:** the shell-command rule on Rust and Go (16, backlog 0643); Rust
  modules compiled only for tests not read as test code (backlog 0644); the weak-key check reading comments (9) and the
  certificate check reading one language's switch-off in another's text (2) (backlog 0645).
- **A fault the rules missed:** the stale test report is removed through a shell, with its path in quotes (backlog 0646).
- **By design, set aside for the owner to confirm in `sv review`,** each as a proposed `[[finding-review]]` entry that
  counts for nothing until then: git's SHA-1 object names (`git.rs`); OpenSSH's own key encryption, which `sv` must read
  as written (`ssh_format.rs`); the list of known breached passwords (`data/breached-password-evidence.json`); the
  passwords `sv` gives the accounts it makes in the app it checks (`signed_in/signin.rs`); the doc comments that quote a
  trial app's invite code and message (`secrets.rs`); a Semgrep React rule on the browser driver, which is not React; the
  `{ query: [...] }` object key in the test sign-in provider; and the app's own test command run through `sh -c`.
- **The 269 path findings stay counted,** at the owner's word: "leave them counted for now as recommended, but add a
  backlog item for after this one to investigate how we can fix the excessive false positives and issue with test
  code, because otherwise actual issues will get overlooked." That is backlog 0647.

What remains here: the `[[finding-review]]` entries, after 0643 to 0646 land; then the report made again and the page
refreshed.

**Done, 23 October 2026.** The fixes the triage found are built: the shell-command rule in Rust and Go (0643, ADR-135),
Rust test modules read as test code (0644, ADR-136), the weak-key and certificate checks (0645, ADR-137), and the stale
report removed with no shell (0646). Twelve findings by design were proposed as false alarms in `stackvet.toml` and
the owner confirmed each in `sv review`, after two of their reasons were reworded: quoting a trial app's
credential-shaped line in a reason made the secrets scan report `stackvet.toml` itself. The owner also tried leaving
the tests' folders out (`[[left-out]]`), to see the feature work: confirmed, it stopped checking 420 files, took 1,854
findings out of the count, and moved 17 requirements from "needs attention" to "not verified", since their only
evidence was the deliberately vulnerable fixtures; the owner then took it back, so the full scan stands. The report
made again: 151 things to fix in StackVet's own code (312 on 22 October), 3,394 more listed apart in test and sample
code, 12 set aside, and 27 things not examined. `docs/website/self-assessment/summary.md`, `sbom.cdx.json`, and the
README are refreshed; the owner publishes the page with **Actions → Site**. The 151 are nearly all the path rule's
(backlog 0647, next), and the bill of materials reading test fixtures is noted there.
