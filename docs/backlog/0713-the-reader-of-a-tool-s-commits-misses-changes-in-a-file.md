# The reader of a tool's commits misses changes in a file whose name git writes in quotes

**Status:** open

Found on 21 November 2026 while testing `av report` (backlog 0705, part 2), by session "Claude Code session
(AgentVet)". It is a gap in what `sv`'s report section "What the AI coding tool did that is worth a look" (ADR-131)
and `av report` see, not in a check of the app.

**What happens.** `sv_check::git::changes` (`crates/sv-check/src/git_changes.rs`) reads a tool's commit with `git
show --raw --patch`. Git writes a file name that holds a letter outside ASCII, a quote, a backslash, or a control
character in quotes with escapes (`"caf\303\251.py"`), in the list of files and in the patch's headers alike. The
reader takes the name as written, so the lines the commit added to that file are not matched to it, and nothing the
reader looks for in added lines is seen there.

**Tried.** A commit whose message names a tool adds `import subprocess  # nosec` in `plain.py`: `av report` lists "a
marker that tells a check to look away (nosec) was added in plain.py". The same line in `café.py`: nothing is listed,
and the section says "Nothing was seen that is worth a look".

**Why it matters.** A tool that names a file with one accented letter keeps its look-away comments, skipped tests,
and changed expectations out of the section. The section says it is information only, but it is the one place they
are shown.

**What to do.** Read the names as git holds them: `-z` for the list of files (as `git::ls_files` and
`git::ever_added` already do, for the same reason: the review of 6 October, item 11), and `-c core.quotePath=false`
with the patch's header names unquoted where git still quotes them (a quote, a backslash, a control character). A
test beside `crates/sv-check/src/git_changes_tests.rs` with each kind of name, which fails today. Until it is fixed,
the section could say when a commit changed a file whose name it could not read, rather than nothing.
