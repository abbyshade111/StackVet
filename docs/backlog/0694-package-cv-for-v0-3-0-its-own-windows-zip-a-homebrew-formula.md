# Package cv for v0.3.0: its own Windows zip, a Homebrew formula, and a Scoop manifest

**Status:** claimed by claude/cv-packaging, 21 November 2026

Filed 21 November 2026, at the owner's word ("work simultaneously on the packaging and adding a new section to
stackvet.dev"), after ConfigVet part 3 (backlog 0693) merged. ADR-157 planned "one release, two packages": a `cv`
formula and manifest beside `sv`'s in the existing `abbyshade111/homebrew-stackvet` and `abbyshade111/scoop-stack-vet`.
The v0.2.0 release built and zipped `sv` alone (`-p sv-cli` in `.github/workflows/release.yml`).

**The owner's choices, 21 November 2026, asked with a recommendation for each:**

- **Prepared now, shipped in v0.3.0.** The release workflow, the formula, and the manifest are made and tried now;
  the formula and manifest point at v0.3.0 once the owner tags it. v0.3.0 is not tagged yet.
- **A Windows zip of its own:** `cv-<version>-windows-x86_64.zip` beside `sv`'s, with its own data folder and SHA-256,
  signed where it was built as `sv`'s is, so the `cv` manifest installs `cv` alone.

**What the work is:**

- **The release workflow** builds `cv` too, checks it starts (`cv --version`, and a `cv check` of an example), zips
  `cv.exe` with `data/` and the licence, and attaches the zip and its SHA-256 beside `sv`'s. It is code-owned, so the
  owner reviews it.
- **Homebrew** (`Formula/cv.rb`): `cv` and its data in Homebrew's own folder for it, as `sv`'s formula does (ADR-036),
  `head` now, and the v0.3.0 source archive once tagged; its test runs `cv --version` and a `cv check`. The tap's
  workflow and ruleset name its install jobs, so `.github/rulesets.json` changes with them (backlog 0690).
- **Scoop** (`bucket/cv.json`): after v0.3.0 is published, its zip and SHA-256, checked against the file before relying
  on it; `checkver` and `autoupdate` as `sv`'s. Whether PowerShell names anything `cv`, as it names `sv` for
  `Set-Variable`, is checked on the bucket's Windows runner.
- **`docs/INSTALL.md`** gains `cv`'s lines once each package works. The stackvet.dev page is another session's
  (it says "coming with v0.3.0").

**Progress, 21 November 2026.** The release workflow builds, checks, signs, and attaches `cv`'s zip beside `sv`'s
(ADR-108, Later), with a step that fails if PowerShell already has a command named `cv`; its trial run is the pull
request that changes it. The tap's `Formula/cv.rb` is head-only, with a job of its own, `install-cv`, on a Mac and on
Linux; its two jobs need adding to the tap's "Protect main" ruleset, and to `.github/rulesets.json`, at the owner's
word. The bucket's `bucket/cv.json` is written and kept back until v0.3.0's zip exists. Still to do: the release itself
(the owner's tag), the formula's v0.3.0 `url` and `sha256`, the manifest's hash checked against a download, and
`docs/INSTALL.md`'s lines.
