# BACKLOG.md's roadmap still describes the one-file-per-item decision as open

**Status:** claimed by builder-none, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: documentation. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** Built on 8 October (ADR-061); the file's own lines 7-13 describe it. The roadmap says it is rewritten only when the order changes, but this sentence describes a closed decision as open.

**Where.** `docs/BACKLOG.md:112-114`: "the owner's decision on 'One file per backlog item, with a status line', which would make the status above a line in each item's file rather than a reading of its prose."

**Evidence.** The sentence against ADR-061's status line.

**How it bites.** A session takes an open decision from the roadmap that was made four days earlier.

**Fix.** Delete the clause or mark it done, with the date.

**Record.** ADR-061 governs `docs/BACKLOG.md`: an "unchanged, because" line.
