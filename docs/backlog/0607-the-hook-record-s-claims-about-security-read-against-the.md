# The hook record's claims about security, read against the requirements each speaks to (from 0565)

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Filed by wording, 15 October 2026, building backlog 0565, whose last paragraph this is: "Claims found in the
transcript ('rate limiting added', 'XSS fixed') land in the same section, worded as 0538 asks (what the code shows,
never a word to make go away)." 0565 was built on the hook record (ADR-122, the owner's choice of 15 October) for the
one claim the record can be read against directly, that the app's tests pass.

**What to build.** More kinds of claim in `sv_cli::hook_record::claims_in`, each kept by kind and never in the tool's
words, each mapped to the requirements it speaks to, a mapping read from each requirement's text first (a citation is a
claim): rate limiting, cross-site scripting, the forgery token, password storage, and the like. The report's hook
section then says, for each such turn, what this report shows for those requirements, worded as 0538 asks: what the
code shows, never a word the tool could make go away by rewriting. It credits nothing.

**Record.** ADR-122, a Later entry.
