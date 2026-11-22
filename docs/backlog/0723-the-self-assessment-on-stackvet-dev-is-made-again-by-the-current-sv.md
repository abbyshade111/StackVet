# The self-assessment on stackvet.dev is made again by the current `sv`

**Status:** open

Filed 22 November 2026, at the owner's word. "StackVet checks itself" shows a summary of `sv`'s report on this
repository, made on 26 October 2026 by `sv` 0.1.0. The workspace is 0.3.0, so `tools/website.py` warns that it is
stale (backlog 0702), and the weekly check, `.github/workflows/site-weekly.yml`, opens its issue on its first Monday
and keeps it open until this is done.

- **To do:** make the report again as `docs/website/self-assessment/README.md`, "How it was made", says, with `sv`
  built from the commit it names, in the repository's own folder, after taking out what git ignores but `sv` would read
  (`git status --short --ignored`; the README's two lessons of 26 October). Update `summary.md`, `sbom.cdx.json`, and the
  README's account of how it was made and what it found, so the site's summary is the new report's.
- **Check:** `tools/website.py` no longer warns about it, the site's tests pass, and the weekly check's issue can be
  closed. Removing anything from the folder, or keeping anything out of `stackvet.toml`'s setting-aside, is the owner's
  choice, asked first.
- **Not here:** the site deploy, which the owner runs.
