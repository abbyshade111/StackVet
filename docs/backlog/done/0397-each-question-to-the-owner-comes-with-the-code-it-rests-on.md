# Each question to the owner comes with the code it rests on

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Asked for by the owner on 13 October 2026, during the first build in the Antigravity trial: investigate whether each
of StackVet's questions to the owner can come with the code it rests on, the lines themselves or at least a link to
the file and line, so the person can check before answering.

What the trial showed (the owner's notes, kept outside the repository until the owner decides what is published):
for each question, `stackvet_check` with section `questions` gave the AI tool a question, and the tool added a "code
observation tip" saying what the code does. The owner asked each time for the exact lines. Several tips did not hold
once the lines were shown:
- **V8.3.1.** "Unapproved users are rejected" rested on a name the caller supplies, with no sign-in.
- **V8.1.1.** The code offered as proof showed two routes with no check at all.
- **V15.3.1.** A test offered as evidence could not fail.
- **V2.3.1.** A test asserted the very overwrite the question warns about.

A person who answered from the tips alone would have put their name to answers the code contradicts.

Two parts to look into:
- **The AI tool cites its evidence.** What StackVet sends with the questions (the `questions` section, and the build
  prompt) could require every tip to quote the exact lines it rests on, with the file and line, and to say plainly
  that a tip is the tool's reading of the code, not a check. This is wording, and cheap; whether tools follow it is
  what a trial would show.
- **StackVet supplies its own.** Where one of StackVet's own checks read the code a question is about, the question
  could carry the file and line it read, as a finding does since backlog 0378 ("The code"). Find out which questions
  that applies to; for most of them, which ask the owner's decisions, no check reads anything, and the item should say
  so rather than invent a location.

Neither may make a tip count as evidence: a tip with lines is still the AI tool's reading, and an answer stays the
tool's word until the owner records it with `sv review` (ADR-026).

**Done, 14 October 2026:**
- **The AI tool cites its evidence.** `HOW_TO_ASK` (`crates/sv-report/src/interview.rs`), given with every `questions`
  section and `sv questions`, now asks for each tip to quote the exact lines it rests on, with the file and line, and to
  say plainly it is the tool's reading of the code, not a check; with no lines to quote, to say so. Whether tools follow
  it is what the next trial shows.
- **StackVet supplies its own.** That is backlog 0487, merged the same day: a question can carry lines a locator found by
  their names, tried on V8.1.1 and V8.3.1, with the list of questions that never get a pointer in 0487's text. The
  fourteen proposals that extend it (0488 to 0499) wait on what the trial shows, as the owner decided.
- **Neither credits anything:** a tip with lines is still the tool's word until the owner records it with `sv review`.
