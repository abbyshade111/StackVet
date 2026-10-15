# Prompt 5: who uses the app and what data it holds set the level, so they are the person's answers, asked first

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502). For the prompt library (0370).

**What was seen.** Antigravity wrote `audience = "customers"` on its own for a family app, which set ASVS level 2
(196 requirements against about 70); Cursor and VS Code chose `my-team` without asking either. The rest of the
"ask before deciding" list (where it runs, who signs in) was asked in all three.

**What to change.** Add "who uses it (`audience`) and what data it holds (`[data] categories`), which set the level,"
to the list of things to ask before deciding, with one plain sentence on what the level means.

**Done, 14 October 2026:** rule 1 of the build prompt (the guide's step 4 and the setup page's step 5, word for word)
now says, after the existing "Ask me before you decide where the app runs …": "**Ask me, too, who uses the app
(`audience`) and what it keeps about people (`[data] categories`):** they decide whether it is held to ASVS level 1 or
the stricter level 2." That is what `Manifest::target_level` does: a customer or public audience, or sensitive or
unlisted data, means level 2. `build_prompt_rules.rs` holds the sentence; with it taken out of the setup page, it and
the word-for-word test went red. Four sentences elsewhere in the guide were shortened, saying the same, to keep it
within its 4,200 words.
