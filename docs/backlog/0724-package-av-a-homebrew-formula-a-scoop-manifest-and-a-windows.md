# Package av: a Homebrew formula, a Scoop manifest, and a Windows zip of its own, in the next release

**Status:** claimed by Claude Code session (AgentVet), 22 November 2026

Filed 22 November 2026, at the owner's word ("file and claim packaging av"). AgentVet's `av` (ADR-159) is built only
from source today: the owner's trials ran it by its full path in a checkout. ConfigVet's `cv` was packaged by backlog
0694, and this follows it: ADR-157's "one release, two packages" becomes three.

`av` belongs on the person's own computer, not in `sv`'s container: `av proxy` starts the tool server (Docker
itself, in the owner's set-up), `av report` reads records kept in the person's own folders, and `av checkpoint` is
run at their terminal with their key. So the packages are the ones a person installs: Homebrew, Scoop, and the
Windows zip.

**Who can do which part.** The release workflow is a code-owned file, and the tap and the bucket are other
repositories. A cloud session writes a change to a code-owned file on its branch, pushes it, and says so; a session
on the owner's Mac opens that pull request as the sessions' App (CLAUDE.md). The parts say which is which.

**For the owner to choose, before part 1 is built:** which release `av` first ships in (v0.3.0 is published without
it), and whether `av` gets a zip of its own as `cv` did, or rides in `sv`'s.

1. **The release workflow builds, checks, zips, signs, and attaches `av`.** As it does `cv`: `av.exe` with the data
   it reads (`av` cuts keys with `sv`'s secret rules, so it needs `data/` beside it; check which files, ADR-036) and
   the licence, its SHA-256, and a step that shows the zipped `av` starts and finds its data (`av --version`, and an
   `av report` of an empty folder, which exits 2 and says what it did not look at). And a check that PowerShell has
   no command named `av` already, as was done for `cv`. Code-owned: a cloud session pushes the branch, a Mac session
   opens the pull request, and the owner reviews it.
   **Part status:** open
2. **`av` says which commit it was built from.** `av --version` prints the version alone; `sv --version` names the
   commit, which a person reporting a fault needs, and which the second trial had to work out by hand. Not
   code-owned.
   **Part status:** open
3. **Homebrew: `Formula/av.rb` in the tap,** head-only first with an install job of its own on a Mac and on Linux,
   then the release's source archive once it is tagged. The tap's ruleset and `.github/rulesets.json` name the
   install jobs, so they change with it, at the owner's word (backlog 0690). Another repository: a Mac session.
   **Part status:** open
4. **Scoop: `bucket/av.json` in the bucket,** after the release is published, with the zip's SHA-256 checked against
   a fresh download. Another repository: a Mac session.
   **Part status:** open
5. **`docs/INSTALL.md` gains `av`'s section** once each package works, with what the hook and the pass-through need:
   the hook `sv connect claude --hook` prints runs a copy of `sv` installed on the computer, not the container's
   (ADR-122), which the owner's trial met.
   **Part status:** open
