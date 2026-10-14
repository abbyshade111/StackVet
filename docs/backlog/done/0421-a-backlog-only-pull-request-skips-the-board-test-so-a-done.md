# A backlog-only pull request skips the board test, so a done note on a queued item turns main red after a green check

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: CI and `tools/`. Severity: medium. Verified against the
code by the reviewing session before filing (`python3 tools/coordination.py check` passes today; the queue was read
from the board).

**What is wrong.** ADR-104's short path runs, for a pull request that changes only backlog items, the tests that read
them: `backlog_board`, `docs_page`, `docs_page_tabs`, `decision_records`, `record_numbers`, `ci_scope`, `ci_shards`.
`crates/sv-cli/tests/coordination.rs` also reads the backlog at run time: `the_real_board_agrees_with_the_backlog`
runs `coordination.py check` on the real board, and `check` fails on "{number} is queued and done". The board is
closed (0273) but still queues ten items: 0289, 0290, 0322, 0324, 0325, 0341, 0369, 0338, 0263, and 0365. A pull
request that only runs `backlog.py done 0325` (a done note on an overtaken item, which CLAUDE.md asks for) changes
files under `docs/backlog/` alone, takes the short path, and is green; the merge then fails `coordination.rs` on
`main`, and the owner's rule makes that session fix `main` within the hour.

**Where.** `.github/workflows/rust.yml:309-317`; `crates/sv-cli/tests/coordination.rs:42-48`; `tools/coordination.py:396-398`.

**Fix.** Add `--test coordination` to the short path's list (it needs only `python3` and `git`, both on the runner)
and a Later line on ADR-104 naming it, as its Later entry of 12 October did for `ci_shards.rs`. Alternatively the
queue could be emptied when the board closes (0416's `close` command), but the test is the right guard either way.
Must not lose: the short path staying short; the full suite on `main`.

**Record.** ADR-104 governs which tests the short path runs: a Later line.
