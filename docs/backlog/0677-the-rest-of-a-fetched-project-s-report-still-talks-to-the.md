# The rest of a fetched project's report still talks to the app's owner, and three capabilities read as answered when nobody answered them

**Status:** claimed by claude/0677, 26 October 2026

**Asked for by the owner, 26 October 2026:** "yes, file it", of what backlog 0676 left: the first report on a project
fetched from GitHub, read again after 0676's fix, still speaks to an app's owner in many places, and in one says
something that is not true.

**What was seen,** in the report on `wiz-sec-public/secure-rules-files`, a project that carried no `stackvet.toml`
(so nobody answered anything), with 0676's fix in:

1. **Three capabilities read as answered when nobody answered them.** In `compliance.md`'s table of the answers,
   `tls`, `internet`, and `level2` read "yes | not recorded, so read as your AI coding tool's | nothing | Taken on the
   manifest's word", while every other row reads "Nobody has said". They come from defaults (as the audience did,
   0676 part 1), not from anything the file says. This is the one that matters most: it presents a default as an
   answer, for an app of the person's own too whenever those are left out. Find where each is derived, and have the
   row say it is the default, and on whose word, as 0676 did for the level.
   **Part status:** open
2. **"Your AI coding tool" throughout.** About 160 times in `compliance.md`, 28 in `information.md`, 20 in
   `your-decisions.md`, and once or twice on each other page: chiefly "a test of the app's could (ask your AI coding
   tool to write one)" on each requirement not verified, and "not recorded, so read as your AI coding tool's" on each
   answer. The reader of a fetched project has no AI coding tool on it; the test and the answers are its author's.
   **Part status:** open
3. **The checklist and the page headers.** `before-going-live.md` tells the reader to fix each finding or set it aside
   in `sv review`, and to tick lines in `sv review`, of an app they will not put live; `fix-now.md`'s header says it is
   "For: You or your AI coding tool, first." Each page's "For:" line, and the checklist as a whole, should say what
   the reader of a fetched project can do with it, or say the page is the author's.
   **Part status:** open

**Done when.** A test of a fetched project with no `stackvet.toml` of its own reads every page and finds no answer
read as given when nobody gave it, and none of the owner's words named here; the same report of an app of the
person's own keeps them; a test of an app of the person's own with those answers left out sees the defaults named as
defaults. Each is broken on purpose and seen red, and the report is run again on a fetched project.
