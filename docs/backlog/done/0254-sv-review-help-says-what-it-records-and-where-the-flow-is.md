# sv review --help says what it records and where the flow is written down

**Status:** done, 12 October 2026

Found by the feedback on the Cursor build (11 October 2026). `sv review --help` prints one usage line and nothing on what
the review records. The report refers to `[[finding-review]]` entries and to set-asides, and the help does not explain
either. Someone who runs the help gets no pointer to the guide section that explains the flow.

What to do: make the help say, in a few lines, what `sv review` writes (set-asides of findings, confirmed answers, the
owner's own answers), and name the guide section, `docs/GETTING-STARTED.md` section 5, by its path. Keep the help short,
since the full flow is in the guide.

**Done 12 October 2026:** `sv review --help` adds, after its usage line, what the review goes through (a finding set
aside, an AI tool's proposal confirmed, the owner's own answers in security-notes.md), that each is written back with a
name, the date, and a seal, that it runs only in the person's own terminal, and the guide section by path and title.
Other commands' help is unchanged. Tests: `crates/sv-cli/tests/review_help.rs` (two, one of them that the guide's section
5 is still there). Broken on purpose: the lines not printed (1 red); printed for every command (1 red).
