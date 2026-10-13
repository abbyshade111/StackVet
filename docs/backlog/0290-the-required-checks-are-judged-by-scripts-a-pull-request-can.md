# The required checks are judged by scripts a pull request can edit (the owner's setting: code-owner review for them)

**Status:** open

**Was queued on the coordination board** (`docs/coordination/board.md`, backlog 0273), which closed on 13 October 2026 when the trial ended; it is taken the normal way since, from the roadmap, with any hold below still standing. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** This is GitHub's standard limit, not a bug in the scripts, which fail closed (`record_numbers.py:109-129` lets any HTTP error propagate; a missing token is a failure; `adr_check.py` reads the description from the event file and re-runs on `edited`). A pull request that edits `ci_scope.py` to print `false`, or `adr_check.py` to print "nothing owed" and exit 0, reports green; a session with auto-merge on merges it; `main`'s own run (ADR-051) finds it afterwards.

**Where.** `.github/workflows/rust.yml:72` runs the branch's own `tools/ci_scope.py`; `.github/workflows/decision-records.yml:42-58` runs the branch's own `tools/adr_check.py` and `tools/record_numbers.py`; for a `pull_request` event GitHub runs the branch's version of every workflow file.

**Evidence.** The step lines above.

**How it bites.** A green check on a pull request that changed the check's own judge.

**Fix.** A ruleset requiring a review from a code owner for `.github/workflows/**`, `tools/ci_scope.py`, `tools/adr_check.py`, `tools/record_numbers.py`, and `action.yml` (a `CODEOWNERS` file plus "require review from code owners"), which is the owner's setting to make; the `CODEOWNERS` file itself is a build.

**Record.** ADR-103 and ADR-104 govern the two scripts; the ruleset is the owner's decision and wants a Later line on ADR-051 when made.
