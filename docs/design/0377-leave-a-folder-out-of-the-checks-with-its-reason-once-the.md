# Leave a folder out of the checks, with its reason, once the owner confirms it (12 October 2026)

Backlog 0249, ADR-099. The owner's answers of 12 October 2026: a folder left out is still searched for real keys, and
the owner confirms what is left out in `sv review`.

**The problem.** A real report of 11 October 2026 had most of its noise from the owner's own test folder (backlog
0248). `not-the-app` sets a folder apart but still counts its findings; nothing let the owner say "do not grade this
folder, and here is why". stackvet.toml is written by the AI coding tool, so a setting that takes findings out of the
count is the easiest way for a tool to make a report look better than the app is.

**What was built.** `[[left-out]]` in stackvet.toml (`sv_manifest::LeftOut`): `folder` and `why`, and `by`, `on` and
`seal` once confirmed. `Manifest::left_out` refuses an entry as a `not-the-app` entry is refused (the two now share
`folder_entry`) and for an empty reason. `Manifest::set_apart` is both kinds of folder, and every command's scan reads
the app through it, so code in a left-out folder never switches a requirement on or off, and ADR-031's guard covers
both together. `crates/sv-cli/src/left_out.rs` decides which entries count (the owner's, with a seal that holds here),
splits the findings (`split`: a finding holding a key or password counts wherever it is), and writes the report's line
for each entry. `static_scan::settle` sets the findings aside after the reviews, so `sv check` and `sv report` agree.
`sv review` offers each usable entry not yet confirmed (`Waiting::LeftOut`), refuses a reason under 40 characters,
and seals the owner's `owner` (`sv_check::seal::left_out_fields`); it signs an older seal again as it does the others.
The spec `sv init` writes says what the setting is, and that it counts only once the owner confirms it.

**Tests.** `crates/sv-cli/tests/left_out.rs` (five, end to end: waiting, confirmed, reason edited after, not the
owner's, the start file, all the code, and what the folder uses), `crates/sv-cli/src/left_out/tests.rs` (two),
`crates/sv-cli/src/review_left_out_tests.rs` (three), and `crates/sv-manifest/tests/left_out.rs` (three).

**Broken on purpose, each put back:** keys set aside with the rest (2 red); any entry counted, sealed or not (4 red);
`by` not checked (1 red, the test written for it after the first run showed nothing caught it); the scan's own list
ignored (1 red, then 2 with the end-to-end test added for it); no reason accepted (1 red); a short reason sealed (1
red); `settle` setting nothing aside (1 red); left-out folders not set apart (1 red, then 3 with the end-to-end test
added for it); the reason left out of the seal (1 red).

**Not built.** `sv audit` reads the folders as `not-the-app`: their packages are listed apart and still counted.
