# Read a CodeQL results file from the owner's own CI, credited only for the commit being checked

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Moved here on 12 October 2026 from backlog 0147 part 7, when 0147 was closed: the one thing in 0147 approved and never
built. **The owner's decision, 9 October 2026** (recorded in 0147, asked by session securevibe-e2): yes to reading a
CodeQL SARIF file from the owner's own CI, as recommended. A clean result is credited only when the file's commit is
the commit of the code being checked; any other file is reported and credits nothing. Nobody had claimed it.

What `sv` does today, for comparison: it runs CodeQL itself, offline, for JavaScript and Ruby (ADR-078), and reads
the SARIF it makes. Go and Java stay out, for the reasons in 0147 part 7. A file from the owner's CI would reach the
languages CI builds, at the cost of trusting that the file is CI's own and of the commit it says.

Before building: a decision record for what is read and from where (it opens no connection; the owner gives the file),
and how a file whose commit cannot be read is reported.
