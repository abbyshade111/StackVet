# Whether CodeQL hides a result its lgtm or codeql comment marks, checked with a real run

**Status:** claimed by documentation, 15 October 2026

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
