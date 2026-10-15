# Prompts 1 and 3: run in a chat on this computer, print the system and folder, and paste the settings written into the chat

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502): Antigravity (prompts 1 to 3 left no record) and
Cursor (prompts 1 to 4 ran in a cloud agent on Linux, whose settings pointed at `/usr/bin/docker`, so the Mac's spawn
failed with ENOENT four times). For the prompt library (0370).

**What to change.** Prompt 1: "in a chat that runs on this computer, not a cloud or background agent; print the
system you are on (`uname -a`) and the folder you are in." Prompt 3: "paste the settings you wrote into this chat, so
the record keeps them." The page's "Tried so far" table takes the three trials.

**The owner's choice, 14 October 2026:** the setup page's "Tried so far" reads: Cursor and VS Code with Copilot did the prompts as written, each with one gap now fixed in the prompts; Antigravity with Gemini 3.8 Flash is still not recommended, because it rewrote code to make a check pass and then said it had not (one sentence, true to the record in docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md).

**Done, 15 October 2026:** prompt 1 now opens by saying it "has to happen in a chat that runs on this computer, not in
a cloud or background agent: if you are one of those, tell me and stop", and asks for what `uname -a` prints on a Mac
or Linux and what `pwd` prints. Prompt 3, in both its Mac-and-Linux and its Windows versions, ends step 9 with "Paste
the `stackvet` entry you wrote into this chat, and nothing else from the file, so the chat keeps a record": the entry
only, since an account-wide settings file can hold keys for other programs. The setup page's "Tried so far" reads as
the owner chose, linking the trial's record, and the guide's says the same within its word budget (4,197 of 4,200);
the page's "Next:" no longer says the trial is to come. A sibling test, `crates/sv-cli/tests/setup_prompt_record.rs`,
holds the prompt sentences; `setup_prompts_guide.rs` still holds the guide's paragraph to the page's, and went red
with the old guide against the new page. Not changed: the website's home-page copy, which still has a placeholder for
the tools the prompts were tried in (`docs/website/home-page.md`).
