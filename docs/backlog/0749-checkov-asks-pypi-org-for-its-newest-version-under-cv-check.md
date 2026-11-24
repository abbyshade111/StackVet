# Checkov asks pypi.org for its newest version under cv check, twice a run, and one setting stops it

**Status:** open

Filed 23 November 2026 by the cato-pipeline session, at the owner's word. Found by running `cv check` (built from
06f5032) in a sandbox that reports every connection a program tries, over a folder with two Dockerfiles. Related to
backlog 0746, which is about a newer version of a tool connecting some other way; this is the measured version itself.

**What was seen.** Checkov 3.4.6, installed with Homebrew, the version `data/cv-adapters.json` was measured at:

- `checkov --version` tries `pypi.org:443` once.
- `checkov --skip-download --framework dockerfile -f Dockerfile …` tries `pypi.org:443` once.
- `cv check` runs both, and the sandbox reported two tries at `pypi.org:443` for one run. With `--no-tools` there were
  none, and hadolint and Trivy, run alone with `cv`'s flags, made none.
- With `CKV_SKIP_PACKAGE_UPDATE_CHECK=true` in its environment, neither command tries anything, and the scan's result
  is the same (one failed check on that Dockerfile either way).

**Where it comes from.** Checkov looks for a newer version of itself when it starts:
`checkov/common/util/banner.py` calls `check_for_update("checkov", version, env_vars_config.SKIP_PACKAGE_UPDATE_CHECK)`,
and `checkov/common/util/update_checker/update_checker.py` sends `GET https://pypi.org/pypi/checkov/json` with a
one-second limit. The address holds the package's name and nothing of the app. `--skip-download` does not turn it off;
that option is about Prisma Cloud's guidance.

**What it means.** ADR-157, part 3, says of the three tools "Each was run in the sandbox, which reports every
connection a program tries", that `--skip-download` stopped Checkov's four requests to `api0.prismacloud.io`, and that
each tool "gave the same results offline". The note in `data/cv-adapters.json` says the same. Both leave this request
out, so `cv check`'s line in "The fence" (`docs/ARCHITECTURE.md`), that the flags keep the three tools offline, is not
yet true of Checkov. It sends no part of the app, and it fails quietly where there is no network. It is still a
connection a person running `cv check` was told would not be made.

**To do.**
- Give Checkov `CKV_SKIP_PACKAGE_UPDATE_CHECK=true`, for its version command as well as its run. `data/adapters.json`
  already has an `env` field for an adapter; check that the version command is started with it too, since the runner
  clears the environment first.
- Measure again in the sandbox, and correct the adapter's note. This changes what `cv` runs and what it connects to,
  so it is a dated "Later" entry on ADR-157, and the sentence in "The fence".
- Hold it: a test that fails when the Checkov adapter has no such setting, shown to work by taking the setting out.
- Worth a look while there: whether `sv`'s own outside tools were measured for a version check of this kind.
