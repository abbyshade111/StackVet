# The build prompt and prompt 6 take in what the Antigravity trial showed

**Status:** partly done: Try the changed prompts in Antigravity with the same model and app, and see whether the tool does better; until then the setup page does not recommend it.

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the owner's trial in Antigravity, 13 October 2026, the changes the build prompt (the guide's step 4, and the
setup page's section 5, which a test holds to it word for word) and prompt 6 need. Each is something the AI tool did
in the trial that a person who is not a programmer would not have caught. The owner's notes on each are outside the
repository.

The build prompt:
- **Ask before deciding where the app runs or who signs in.** The tool set where the app runs, `auth = false`, and
  `tls = "terminated-upstream"` in `stackvet.toml` on its own, and the deployment lines changed three times without
  asking.
- **Name the app's routes in `stackvet.toml`,** so the checks of the running app reach them (ADR-096). They tried only
  `/health` and `/`, so the API was never sent malformed input.
- **Use only the `stackvet_` tools.** Never `docker exec` into StackVet's container.
- **Never silence or work around a finding.** No `# nosec`, and no rewrite that only stops a scanner seeing it (as `""`
  for `"0.0.0.0"`). Tell the person and let them decide, as step 3 already says of findings that look wrong; the trial
  showed that is not enough.
- **A tip is the tool's reading of the code, not a check.** Quote the lines it rests on (backlog 0397 is the larger
  part). Never offer "done" for a check by hand nobody made, or write `by = "owner"` for one.
- **Say what changed** after each change, in plain words, and ask before deleting anything (backlog 0401 is the larger
  part).

Prompt 6: open `report.html` in the person's web browser for them (`open` on a Mac). In the trial the person opened it
in the AI tool, which showed its source, unreadable. Also ask before deleting the report folder, which the tool did.

Then try the changed prompts in the same app and tool, and see whether the tool does better (the setup page's "Tried so
far" says Antigravity, with that model, is not recommended until then).

**What was built (14 October 2026, session documentation).** The build prompt (the guide's step 4, and the setup
page's step 5, held to it word for word) now says: ask the person before deciding where the app runs, whether anyone
signs in, or how it is reached (`auth`, `tls`, the `[stack.run]` lines), and say when one changes; name the app's
pages and routes under `[stack.run.users]`, not only `/` and the health path (ADR-096); use StackVet only through the
`stackvet_` tools and never run commands inside its container; never silence a finding or hide it from a scanner (no
`# nosec`, no `nosemgrep`, no rewrite whose only effect is that the check stops seeing it), but tell the person and let
them decide; after each change say in plain words what changed, and ask before deleting anything; when saying what
the code does for a question, quote the lines and call it a reading, not a check; and never mark a check by hand as
done or write `by = "owner"`, which only the person can do with `sv review`. Prompt 6 now opens `report.html` in the
person's web browser (`open`, `start`, or `xdg-open`), not in the chat, and asks before deleting the report's folder or
anything else. The guide's and the setup page's "Tried so far" say the prompts were changed and not yet tried again.
Held by `crates/sv-cli/tests/build_prompt_rules.rs`, which names each rule; with `# nosec` dropped from the prompt it
went red. Backlog 0397 (the code behind each question) and 0401 (what changed, and deleting) remain the larger parts.
