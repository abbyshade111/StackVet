# A test holds the ConfigVet page's install lines to docs/INSTALL.md's

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, from the whole-site review (backlog 0696). The ConfigVet page said
"not packaged yet" after the tap had a head-only `cv` formula, and its install lines had to be rewritten by hand
when v0.3.0's packages moved. `docs/INSTALL.md` is where install commands are kept and tried.

- **What to build:** once `docs/INSTALL.md` has `cv`'s lines (the v0.3.0 packaging session adds them), a test that
  every install command on the ConfigVet page (each code block line starting `brew`, `scoop`, or `cargo`) is in
  `docs/INSTALL.md` word for word, so the guide and the site cannot disagree. The guide stays the one place the
  commands are decided.
- **Done when:** changing a `cv` command in `docs/INSTALL.md` alone turns the test red.
- **Order,** after 0696, and after `docs/INSTALL.md` has `cv`'s lines (backlog 0694).

**Done, 22 November 2026:** `crates/sv-cli/tests/website_configvet_install.rs`; it found the page's source-build line missing from `docs/INSTALL.md`, which has it now, and changing the guide's line alone turned it red. ADR-132, Later, 22 November 2026.
