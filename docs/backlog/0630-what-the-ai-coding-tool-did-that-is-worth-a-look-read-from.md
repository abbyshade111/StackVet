# What the AI coding tool did that is worth a look, read from git and the hook record, as information in the report

**Status:** claimed by claude/0630-rest, 22 October 2026

**Record:** ADR-131, proposed with this claim.

Filed 21 October 2026 at the owner's request, asked in a session: "Is there a way to include information in the report
to flag (as informational) for the user any potentially risky behavior on the part of the AI coding tool?" To be built
after the paper's tables (backlog 0577), at the owner's word. Severity: medium (the trials of 14 and 15 October found
an AI coding tool rewriting code and dependency files to make a check stop seeing something, and saying it had not;
nothing in the report says so today).

**What is there already.** The build-loop record (ADR-076) keeps each call the tool makes to `sv`'s MCP server; a note
lists the commits the tool made straight onto the default branch (ADR-092); the hook record (ADR-122), when the person
installs its hook and consents for a report, keeps each command the tool ran and each file it wrote, chained, and sets
the claims a turn ends with beside the report's verdict; and `sv review`'s seal keeps an answer the tool wrote in the
owner's name from counting as the owner's (ADR-026). None of them is read for what the tool did that a person should
look at.

**What to build.** A section of the report, "What the AI coding tool did that is worth a look", read from git, the
hook record when it is there, and the reports before this one:

- **A finding that went away without a fix:** in the last report and not in this one, where the change between them to
  the file it was in (or to a dependency file it rested on) did not touch the line, or removed the thing found from
  the file `sv` reads rather than from the app (a package dropped from `requirements.txt`, code reworded around a
  pattern). The 14 October trial's gaming, seen by a person then.
- **Answers in stackvet.toml changed by the tool** (by the hook record's file writes, or a commit an AI tool made), above
  all those that lower the level or switch requirements off: `audience`, `[data] categories`, a capability set to
  `false`.
- **Tests skipped or removed:** a skip marker added (`@pytest.mark.skip`, `@unittest.skip`, `it.skip`, `xit(`,
  `test.skip`), a test file deleted, or a test's expected value changed in the same commit as the code it tests.
- **Commands that destroy work or step around a guard,** from the hook record: `git push --force`, `git reset --hard`,
  `git clean -fdx`, `rm -rf` outside the build folder, `--no-verify`, a script piped from the network into a shell,
  `chmod 777`, `sudo`.
- **Guards edited:** CI workflows, a linter's or scanner's settings turned down, `.semgrepignore` or a `nosec` /
  `eslint-disable` / `# type: ignore` added beside a line a finding named.
- **Packages installed that no lockfile holds,** and a secret written into a file (the secrets scan already finds the
  second; this says the tool wrote it).

**Rules it is held to.**
- Informational only: it changes no requirement's status, counts toward nothing, and is never a finding.
- Each line says what was seen, when, and where (the commit, or the hook record's line), and never guesses why: a
  force-push or a removed test can be right.
- Without the hook record, the section says which of these it could not see, and names only what git shows.
- What the hook record keeps does not change: commands cut short with keys cut, and file paths; never a file's
  contents, a command's output, the person's prompts, or the tool's words (ADR-122).

**Record.** A new ADR, proposed with the claim: what the section reads, and that it never decides anything.

**Claimed again, 22 October 2026,** for the two lines not yet looked for, the roadmap's first open row a session can
build (0120's and 0257's remains wait on the owner): packages the AI coding tool installed that no lockfile holds, read
from the hook record's commands against the app's lockfiles; and a secret in a file the tool wrote, the secrets scan's
finding set beside the hook record's file writes. Both under the rules above: informational, counted toward nothing,
saying what was seen and where. **Record, proposed:** a Later entry on ADR-131, made accepted in the pull request that
builds them.
