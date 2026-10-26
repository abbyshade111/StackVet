# A fetched project's report says the level comes from stackvet.toml and tells the reader to fix and confirm, as if it were their own app

**Status:** claimed by claude/0676, 26 October 2026

**Asked for by the owner, 26 October 2026:** "yes, file it as a backlog item and go ahead and starting working them",
after the first report on a project fetched from GitHub (`sv report https://github.com/OWNER/PROJECT`, `sv` 2044b9c,
the feature of backlog 0670 and ADR-154).

**What happened.** Every page opens with the fetched project's banner, but three passages under it were written for a
person reading a report on their own app, and one of them says something that is not true.

1. **The level says `stackvet.toml` gave an answer it never gave.** `compliance.md` read "Level 2 because
   stackvet.toml says customers use it, and it does not say what information the app holds about people: answers in
   stackvet.toml, which your AI coding tool usually writes and nobody has confirmed, so check them, and confirm them
   with `sv review`." The project carried no `stackvet.toml`; the one `sv` wrote holds comments alone. The audience is
   `customers` because that is `Audience`'s default when nothing says otherwise, and `Manifest::level_because`
   (`crates/sv-manifest/src/lib.rs`) words the default as the file's answer. That is wrong for an app of the person's
   own too, whenever `[app] audience` is left out. The level should say when it rests on the default, not on an answer;
   and for a fetched project it should say nobody answered, not tell the reader to check and confirm answers that are
   the author's to give (`level_why_sentence`, `crates/sv-report/src/bluf.rs`).
   **Part status:** open
2. **Each finding's next step tells the reader to fix the code and confirm a review.** `next_step`
   (`crates/sv-report/src/lib.rs`) reads "Fix it in the code and check again. If it is a false alarm, write a
   `[[finding-review]]` entry for it ... and confirm it with `sv review` in your own terminal". For a fetched project
   the banner says fixing is the author's to do, and a seal the reader made would count only in the folder it was made
   for. The step should say what the reader can do: weigh it, and look at the code it names.
   **Part status:** open
3. **The build history speaks of "while this app was built."** `build_loop.rs` reads "Nothing shows that sv was used
   while this app was built: no call from an AI coding tool to sv's MCP server is recorded for it." Of a stranger's
   project, that is a sentence about a build the reader had no part in. For a fetched project it should be left out, or
   say only that the project carries no record of how it was built.
   **Part status:** open

**Done when.** A test reports on a fetched project with no `stackvet.toml` of its own and reads every page: the level
names the default as a default, not as the file's answer, and asks nobody to confirm it with `sv review`; no finding is
told to be fixed or reviewed; and no page speaks of while this app was built. A test of an app of the person's own,
with `[app] audience` left out, sees the level say so. Each is broken on purpose and seen red.
