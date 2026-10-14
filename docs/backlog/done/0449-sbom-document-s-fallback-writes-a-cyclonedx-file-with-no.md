# `sbom_document`'s fallback writes a CycloneDX file with no components and no caveat, though its comment says the file says so

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-run`, `sv-report`, `sv-scan`. Severity: low. Verified against the code by the reviewing session before filing.

- Where: `crates/sv-report/src/lib.rs:1613-1621`: `/// ... An empty CycloneDX document, saying so, for a report made without one.` then `None => "{\"bomFormat\": \"CycloneDX\", \"specVersion\": \"1.5\", \"version\": 1, \"components\": []}\n"`.
- What is wrong: The fallback document carries no `metadata.properties` entry saying the bill was not made; a
  reader of `sbom.cdx.json` alone sees a valid document with zero components, which is exactly what 0402 found
  misleading for an app on the standard library alone. `sbom_line` returns `None` in this case, so the report pages
  do not name the file at all, while the file is still written (it is in `REPORT_FILES` and sealed). Today every
  path that renders report files goes through `assemble`, which sets `Some`, so the fallback is only reached by
  tests; the comment and the behaviour still disagree.
- What to do: Either make `Report.sbom` required (the fallback goes away), or put a property in the fallback
  (`securevibe:complete = false` and a `securevibe:not-made` reason) and have `sbom_line` say the file holds no bill,
  so the page and the file agree.
- How verified: read (`lib.rs`, `crates/sv-cli/src/assemble.rs:2738-2754`, `crates/sv-cli/src/report_files.rs:63-65`).
- Overlap: 0382 and 0402 (done) are about the file and its content; neither covers the fallback.
- Needs a decision record?: no, unless the file's content changes (ADR for 0382 governs what `sv` writes beside the report; a Later line then).
