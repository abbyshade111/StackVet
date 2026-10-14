# V2.2.3 and V13.2.5 get a question of their own, saying why nothing in the folder can show them

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found while building backlog 0487 (14 October 2026). 0487's list of the questions that never get a pointer, because
nothing in the app's folder can show them, names two requirements that have no question at all:

- **V2.2.3**, the app's own business rules;
- **V13.2.5** without infrastructure files, the host's rules.

The other six in that list already have a hand check in `data/human-checks.json` whose text says why the folder cannot
show it. These two should get one too: a hand check, or a design question, whose words say why the code cannot answer
it. Read each requirement's text again before writing its entry (a citation is a claim). 0487 built the pointer
mechanism and its V8 trial only, as the owner decided on 13 October 2026, so this was left for an item of its own.

**Built 14 October 2026, by honesty.** Each requirement's text was read again first. V2.2.3 asks that combinations of
related data items are reasonable "according to the pre-defined rules"; V13.2.5 asks that the server is configured
with an allowlist of where it may send requests or load data or files from. Both now have a hand check in
`data/human-checks.json` that starts "`sv` does not check this:" and says why: which details must fit together is a
rule of the owner's own business, which nothing in the code names; and the outbound list is set where the app is
hosted, whose settings `sv` does not read. Each then says what to write down and what to try. Nothing is credited;
the report still shows both as not verified, with the instruction beside them. The new test
`crates/sv-check/tests/rules_only_the_owner_knows.rs` holds both entries and their reason; removing the V13.2.5 entry
turned it red, and nothing else, since nothing held these two before.
