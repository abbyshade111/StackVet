#!/usr/bin/env python3
"""Says whether a pull request needs the whole test suite, from the list of files it changes (ADR-104).

Until 12 October 2026 the required `test` check ran the whole suite, about a quarter of an hour, on every pull
request. A claim, one changed line in one backlog item, waited as long as a build, and the build waited on the claim.
A pull request that changes nothing but backlog items (Markdown files directly in `docs/backlog/` or
`docs/backlog/done/`) now runs only the tests that read them. From 14 October 2026 the coordination board and its
waiting files counted as backlog items too; the board and its test, `coordination.rs`, were removed on 20 October 2026,
and with nothing on the short path reading them, a change under `docs/coordination/` runs everything again (ADR-104,
Later). Anything else, an empty list included, runs everything.
The whole suite still runs on every commit to `main` (ADR-051), so anything the short path misses shows up there.

It reads the changed paths, one per line, on standard input, and prints `true` when the whole suite must run and
`false` when the short path is enough. It never fails toward the short path: a line it cannot read, a path elsewhere,
or no paths at all print `true`.

    git diff --no-renames --name-only HEAD^1 HEAD | python3 tools/ci_scope.py
    python3 tools/ci_scope.py --self-test
"""

import re
import sys

# A backlog item, open or done, and nothing deeper: a folder inside either is not one the tools read.
BACKLOG_ITEM = re.compile(r"docs/backlog/(done/)?[^/]+\.md")


def needs_everything(paths):
    """True unless every path is a backlog item and there is at least one."""
    paths = [p.strip() for p in paths if p.strip()]
    if not paths:
        return True
    return not all(BACKLOG_ITEM.fullmatch(p) for p in paths)


def self_test():
    cases = [
        ([], True),
        ([""], True),
        (["docs/backlog/0272-a-thing.md"], False),
        (["docs/backlog/0001-a.md", "docs/backlog/done/0002-b.md"], False),
        (["docs/backlog/0001-a.md", "docs/BACKLOG.md"], True),
        (["docs/backlog/0001-a.md", "crates/sv-cli/src/main.rs"], True),
        (["docs/backlog/notes.txt"], True),
        (["docs/backlog/sub/0001-a.md"], True),
        (["docs/backlog/done/sub/0001-a.md"], True),
        (["docs/backlogs/0001-a.md"], True),
        (["xdocs/backlog/0001-a.md"], True),
        (["docs/backlog/0001-a.md.bak"], True),
        # The board's files were on the short path until the board was removed (ADR-104, Later, 20 October 2026).
        (["docs/coordination/board-closed-2026-10-13.md"], True),
        (["docs/backlog/0001-a.md", "docs/coordination/waiting/0466.md"], True),
        (["docs/adr/ADR-104.md"], True),
        (["CLAUDE.md"], True),
    ]
    for paths, want in cases:
        got = needs_everything(paths)
        assert got == want, f"{paths}: wanted {want}, got {got}"
    print("ci_scope self-test: ok")


def main():
    if sys.argv[1:] == ["--self-test"]:
        self_test()
        return
    if sys.argv[1:]:
        sys.exit("usage: ... | python3 tools/ci_scope.py, or --self-test")
    print("true" if needs_everything(sys.stdin.read().splitlines()) else "false")


if __name__ == "__main__":
    main()
