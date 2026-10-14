# The questions to the person are asked in the standard's own jargon, and only the AI tool stands between it and them

**Status:** claimed by honesty, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: usability. Severity: medium. Verified against the data
files by the reviewing session before filing.

**What is wrong.** `data/design-questions.json` asks V8.3.1 as "Does the application enforce its authorization rules
at a trusted service layer ... rather than relying on controls an untrusted consumer could manipulate"; V7.2.1 says
"trusted backend service", V2.2.2 "trusted service layer"; `data/security-notes.json` says "credential stuffing" and
"malicious account lockout" for V6.1.1, and its `howToFindOut` is "Look in your sign-in code". The plain wording is
delegated to the tool (`interview.rs:27`, "in plain words"), and 0397 showed what the tool's own reading is worth: on
four questions its tip did not survive the lines being shown. The `title` and `asks` are held by a test to share
words with the requirement, which pushes them toward OWASP's words; nothing in the data is written for the person,
though the guide promises "the questions only you can answer" in words they know.

**Where.** `data/design-questions.json`, `data/security-notes.json`, `data/human-checks.json`; printed to the person in
"What only you can check" (`crates/sv-report/src/markdown.rs:493-506`) and to the tool in `interview.rs:172-183`.

**Fix.** A `plain` field per question, one sentence a non-programmer can answer without the tool's paraphrase, beside
`asks`, which stays the standard's words and keeps its test. V8.3.1: "When somebody tries to open a page or do
something they are not allowed to, is it the server that stops them, or only a button that is hidden?" V6.1.1: "After
how many wrong passwords in a row does the app slow down or lock the account, and for how long? (Say the numbers you
want, even if the app does not do it yet.)" The interview prints `plain` first and `asks` under it as "the standard's
words"; the report's "What only you can check" prints `plain`. A test holds every question to having one. Must not
lose: `asks` stays the citation (a citation is a claim); `plain` never softens a "no" into a "not sure". Sibling of
0372 (plain lines for the *requirements*) and 0397 (the code beside each question); these are the *questions'* words.

**Record.** None; `data/human-checks.json` is in ADR-022's Governs list, so an unchanged line.
