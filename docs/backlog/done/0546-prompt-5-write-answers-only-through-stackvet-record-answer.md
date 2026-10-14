# Prompt 5: write answers only through `stackvet_record_answer`; text in the tool's own words is not read

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Cursor trial. For the prompt library (0370).

**What was seen.** The tool wrote `security-notes.md` by hand with the owner's real decisions and never called
`stackvet_record_answer`; sv counted `documented = 0`. Nothing told it other text would not be read.

**What to change.** Prompt 5: "Write security-notes.md only through stackvet_record_answer, under the question ids it
gives you; anything you write in your own words there is not read by sv."

**Done, 14 October 2026:** rule 6 of the build prompt (the guide's step 4 and the setup page's step 5, word for word)
now says "Record my answers only with `stackvet_record_answer`, under the question ids it gives you, and only what I
actually answer as mine. Text you write yourself in `security-notes.md`, outside its questions, is not read as an
answer." The item's wording, "anything you write in your own words there is not read by sv", was narrowed to what is
true: a section written by hand under a question's own heading is read, as the AI tool's word, like one recorded
through the tool; text outside the questions is kept and never read as an answer (`kept_text_is_never_an_answer` in
`crates/sv-check/src/notes.rs`, and what `stackvet_record_answer` itself tells the tool). `build_prompt_rules.rs`
holds both sentences; with the second taken out of the setup page, it and the word-for-word test went red. To stay
within the guide's 4,200-word budget, two sentences in its steps 2 and 5 were shortened, saying the same.
