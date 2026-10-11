# Find pushes to main that skipped the pull-request rule, from the repository's own history

**Status:** claimed by securevibe-git-history, 11 October 2026

Found by the Cursor build (design entry 0368, on the branch of backlog 0222). The AI coding tool pushed to `main`
directly four times, twice after the owner had set a rule requiring pull requests, and the owner found a second
ruleset with a bypass only by looking. `sv` did not see any of it: its build-loop record holds only the calls the tool
makes to `sv` (ADR-076), and `sv` opens no network connection, so it cannot read branch protection or rulesets.

The idea: a check that reads the repository's own history, which needs no network. Commits on `main` that did not
arrive through a merge from a pull request, and commits that name an AI coding tool in an `Assisted-by:` or
`Co-Authored-By:` line (as the tool's own rules require), would be listed with their hashes. The report would say
what the check could and could not see: a bypass shows as a plain commit on `main`, but a settings change that
removes a rule does not show at all.

Open questions for whoever takes it up: whether `git log --first-parent` on `main` is enough to tell a pull-request
merge from a direct push on this repository, and how the check handles a merge the owner makes by hand. It should be
checked first against the four direct pushes that design entry 0368 describes.
