# Whether CodeQL hides a result its lgtm or codeql comment marks, checked with a real run

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Left over from backlog 0400 (14 October 2026). The report now lists every `lgtm` and `codeql[...]` comment in the
app's code (`crates/sv-check/src/look_away.rs`), but whether CodeQL itself leaves out a result such a comment marks,
as `sv` runs it (`data/adapters.json`: the security-extended suites), is not known. Bandit is run with `--ignore-nosec`
and Semgrep, gosec, and Brakeman mark a suppressed result rather than drop it (`adapters::suppressed_by`), so for them
the finding stays in the report. If CodeQL drops it, the finding a comment hid is missing, and only the listed comment
says something was there.

- What to do: run CodeQL as `sv` runs it over a small fixture with a result on a line marked `// lgtm[js/...]` and one
  marked `// codeql[js/...]`, and read the SARIF: dropped, marked under `suppressions`, or reported as usual. If
  dropped, make it look anyway or say so in "What was not examined"; either way, hold it with a test against the
  real SARIF kept as a fixture, as `crates/sv-check/tests/suppressed.rs` does for the others.
- How verified: not yet; no `codeql` in the cloud session's container.
- Needs a decision record?: only if CodeQL is run differently (ADR-111 governs the outside tools).

**Not built, 15 October 2026** (documentation session). The board assigned it to documentation with "if CodeQL is not
installed in the session, say so on the item and move on", and the owner said the same day that it goes to a session
with CodeQL. This cloud session has no `codeql` (not on the path, nor in `/opt` or `~/.codeql`), so nothing was run
and nothing is known yet about whether CodeQL drops a result its comment marks. The claim stays as the board has it;
handing it to a session with CodeQL is the orchestrator's (`coordination.py withdraw`, then `assign`).

**Done, 15 October 2026 (the orchestrating session, at the owner's request; CodeQL 2.27.2 from the bundle on the
owner's drive, run on the Mac).** CodeQL as `sv` runs it (`database create`, then `database analyze` with the
security-extended suite and `--format=sarif-latest`) over a small JavaScript app and a small Python app, each holding
the same command injection four times: unmarked, with `lgtm[...]` on the line, with `codeql[...]` on the line before,
and with `codeql[...]` on the line. In both languages all four were reported, and none carried a `suppressions`
entry: the command-line tool does not act on these comments (GitHub's code scanning does, after the fact). So a
finding such a comment would hide reaches `sv`'s report as an ordinary finding, and the comment is listed besides
(`look_away.rs`); nothing needs to change in how CodeQL is run. Held by
`crates/sv-check/tests/suppressed_codeql.rs` against the two real reports kept in
`crates/sv-check/tests/fixtures/suppressed/` (`codeql-2.27.2-js-lgtm.sarif`, `codeql-2.27.2-py-lgtm.sarif`, with the
apps beside them). The test first asserts that the kept report holds all four results unsuppressed, then that each
marked line is one of `sv`'s findings. Not tried: Ruby and the other CodeQL languages, and a CodeQL later than 2.27.2.
