# av checkpoint adds the app to this computer's trusted keys without saying so

**Status:** open

Found by the ninth weekly review of the decision records (0742, 23 November 2026), reading ADR-159 against `main`.

**What happens.** `av checkpoint` (`crates/av-cli/src/checkpoint.rs`, built in #2506) calls
`sv_check::signed::trust_here` before it signs. When the owner's key is not yet trusted for this app on this computer,
that adds a line to `allowed_signers` in `sv`'s settings folder, making the folder if it is not there
(`crates/sv-check/src/signed.rs`, `trust_here`). `av checkpoint` drops the answer that says whether a line was added,
so it prints nothing about it. `sv review` does the same thing and prints a paragraph saying so
(`crates/sv-cli/src/review.rs`).

**Why it matters.** The list is what `sv report` reads to decide whose signed answers count for an app. A line added
there is a write into the person's home folder that changes what a later report concludes, and until this review no
record said `av` makes it: ADR-159 part 7 said only that a signature "is checked against the keys this computer trusts
for it", ADR-043 said the list is one "which `sv review` only ever adds to", and the fence page said `av checkpoint`
"writes that signature beside the records". The review wrote the fact into ADR-159 and ADR-043 as dated entries. The
only reason written anywhere is the code's comment: "so a report here can check what it signs".

**To do.** Either `av checkpoint` says, as `sv review` does, that it has named this key as trusted for this app and
where, or it stops adding the line and tells the owner to run `sv review` once first. Which is the owner's choice
(it is their list); the first is the smaller change. A test in a sibling file that fails when the line is added
with nothing printed. The fence page's sentence on `av checkpoint` gains the list either way.
