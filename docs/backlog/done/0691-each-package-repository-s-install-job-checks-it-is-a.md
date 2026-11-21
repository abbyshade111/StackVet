# Each package repository's install job checks it is a required check

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, beside 0690. The "Protect main" rulesets in
`abbyshade111/homebrew-stackvet` and `abbyshade111/scoop-stack-vet` require their install jobs by name, matrix values
included (`install (macos-latest, head)`). A job renamed, or a system added to the formula's matrix, is then not
required, and nothing says so: a formula or manifest that fails there could still merge.

- **The step.** In each repository's `test.yml`, every `install` job reads the rules for `main`
  (`GET /repos/{owner}/{repo}/rules/branches/main`) and fails, saying which, when its own name is not among the required
  checks. It runs on pull requests and on the weekly schedule.
- **What it misses, said in the item's note:** a job removed from the workflow stays required and never reports, so
  pull requests wait on it. That is visible ("Expected — waiting"), not silent, so it is left.
- **The witness.** Rename the job on a branch and watch the step fail.
- The two repositories are outside this one, so their pull requests carry the work; this item records them and is
  marked done here.

**Done, 21 November 2026.** abbyshade111/homebrew-stackvet#3 and abbyshade111/scoop-stack-vet#2 give each `install`
job a first step, "This job is a required check on main". It reads the job's own name from its check run
(`job.check_run_id`, with `checks: read` added; a name built from the matrix would miss a new matrix key), reads
`GET /repos/{owner}/{repo}/rules/branches/main` with the workflow's own `GITHUB_TOKEN` (no secret), and fails, naming
the job, when the name is not among the required checks. It runs on every push and pull request and on the weekly
schedule. Each README says so in a sentence. The witness: a branch `claude/0691-witness` in each repository, pushed with
no pull request so it could not merge. In Homebrew, `ubuntu-24.04-arm` added to the matrix sent its two new jobs red
with `The job "install (ubuntu-24.04-arm, release)" is not a required check on main` (and `head`), while the four
required jobs passed (run 44905110301). In Scoop, the job renamed to `install-windows` failed the same way, listing
`install` as what is required (run 44905119268). What it misses: a job removed from the workflow stays required and
never reports, so pull requests wait on it ("Expected — waiting"); that is visible, so it is left. The witness branches
were deleted the same day, at the owner's word, once each was seen to differ from `main` only by its one-line
break.
