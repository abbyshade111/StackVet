# The setup page's prompts carry a date and commit line, so a trial record says which text the tool read

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: low, for the prompt library (0370).

**What was seen.** The prompts pasted in the Opus run match the page's text of 14 October, before 0545 to 0557 landed:
none of this week's rules (the audience asked, no `confirmed`, silencing includes rewriting, the `.gitignore` line, the
sandbox clause, "until I run sv review") was in what the tool read, which a reader could tell only by diffing the
pasted text against the page's history.

**Fix.** Each prompt block on the page ends with one quiet line, "StackVet setup prompts, <date>, <short commit>",
written by the test that holds the page to the guide, so a pasted prompt carries its version; the trial-reading
scripts read it.

**Record.** None.

**Done, 15 October 2026.** Each of the eight prompts on the setup page, and the guide's copy of prompt 5, ends with one
line in italics, inside the quote so it is pasted with the prompt: "*(StackVet setup prompt 5, text of 15 October 2026,
fingerprint 21439403)*". One change from the item: the line names a fingerprint of the prompt's words (the first eight
hex characters of their SHA-256, the stamp left out) instead of a commit, since a commit cannot name itself; `git log -S
"fingerprint 21439403" -- docs/prompts/setup.md` finds the commit that wrote it. `tools/prompt_stamps.py` writes the
stamps (`--check` lists those out of date), and `crates/sv-cli/tests/setup_prompt_stamps.rs` holds them, computing the
fingerprint the same way; its failure names the script. A word changed in a prompt without its stamp, and the guide's
stamp taken out, each turned it red. The page says, above its table, what the line is and to copy it with the rest.

Not done: the item's "the trial-reading scripts read it". There is no such script in `tools/`; trial records are read
by hand, and the line is there for that reading to find.
