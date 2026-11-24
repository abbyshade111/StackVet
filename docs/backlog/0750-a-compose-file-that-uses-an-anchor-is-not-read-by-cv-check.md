# A compose file that uses an anchor is not read by cv check, and one such file makes the whole run exit 2

**Status:** open

Filed 23 November 2026 by the cato-pipeline session, at the owner's word. Found by running `cv check` (built from
06f5032) over a repository with two Dockerfiles and one compose file.

**What was seen.** `cv check` read both Dockerfiles, then said `Could not read rehearsal/docker-compose.yml: the file
uses a YAML anchor, alias, or tag`, and ended with exit status 2. The compose file uses an anchor twice, in the two
ways compose files usually do: a list of network names written once and used by two services (`aliases: &names`, then
`aliases: *names`), and one service's `environment` reused by another through a merge key (`environment: &agent`, then
`<<: *agent`).

**This is as designed, and the design has a reason.** `crates/sv-yaml/src/lib.rs` refuses an anchor, an alias, or a
tag "rather than followed", and exit 2 for a deployment file that could not be read is ADR-029's rule and ADR-157's,
part 2. Refusing is honest: the file is said to be unread, and nothing in it is credited. This item does not ask for
that to change without the owner's word.

**What it costs.** Anchors and merge keys are how Docker's own documentation says to share settings between
services (its "fragments" and "extensions", the `x-…` blocks). So a compose file of any size is likely to use one, and
then:

- none of that file is read, though most of its services may use no alias at all;
- the whole run exits 2, so a pipeline that treats 2 as "look at this" is told to on every run, for as long as the
  file is written that way; and
- the person is told the file "could not be read", which reads as a fault in their file. It is a valid compose file.

**To ask the owner, with a recommendation.**
- **Option A, read what does not depend on an alias.** A service whose settings use no alias or merge key is read
  as now; one that does is *not assessed*, named, with the reason. The file is then "read in part", and the run exits 2 only
  with `--fail-on not-assessed`, as for any other setting `cv` cannot settle. Recommended: it keeps the rule that an
  alias is never followed, and stops one anchor hiding every service.
- **Option B, follow an alias within limits.** Aliases that point at a mapping or a list in the same file, to a
  depth and a size limit, with no tag ever honored. It reads more, and is the reading where a mistake could credit a setting the
  deployment does not have; it would want its own record.
- **Option C, leave it, and say it better.** Keep refusing the file, and change the sentence to say the file is
  valid and why `cv` does not read it.

Whichever is chosen is a "Later" entry on ADR-157, and `sv-check` reads YAML through the same crate, so the answer is
`sv`'s too.
