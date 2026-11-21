#!/usr/bin/env python3
"""Fails a session's pull request that the owner's own account opened and that changes a code-owned file (backlog 0710).

A cloud session opens pull requests through the owner's own GitHub connection, so GitHub counts the pull request as
the owner's, and the owner, as code owner, as having reviewed it: the review `.github/CODEOWNERS` asks for on the
workflows and the scripts that judge the checks is never asked for (ADR-158, Later, 21 November 2026). Sessions on
the owner's Mac open theirs as the sessions' App, whose pull requests do wait for the review. CLAUDE.md says a cloud
session pushes its branch and leaves the pull request to a session on the Mac; this is the check that catches it when
one does not.

It reads the pull request's event file: when the pull request was opened by the repository owner's account from a
`claude/` branch, and changes a file CODEOWNERS names, it fails, naming the files and saying to reopen it as the App.
CODEOWNERS is read line by line, and a pattern this script cannot match exactly (a `*`, a path without a leading `/`)
stops it, rather than being skipped.

What it does not do: tell the owner from a session acting as the owner. A session set on getting around it could use
another branch name; the gates on publishing, releasing, and deploying are what hold whoever the author is.

It reads files and runs `git diff`; it opens no connection. It is a tool for this repository's upkeep, not part of
`sv`.

    python3 -I tools/cloud_session_check.py --base SHA --head SHA --event "$GITHUB_EVENT_PATH"
    python3 -I tools/cloud_session_check.py --self-test
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CODEOWNERS = ROOT / ".github" / "CODEOWNERS"
SESSION_BRANCH = "claude/"


class Unreadable(Exception):
    """A CODEOWNERS line this script cannot match exactly."""


def patterns(text):
    """The paths CODEOWNERS names: `/a/b/` for a folder and everything under it, `/a/b` for one file."""
    found = []
    for n, line in enumerate(text.splitlines(), start=1):
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        path = line.split()[0]
        if not path.startswith("/") or any(c in path for c in "*?[]!\\") or path == "/":
            raise Unreadable(f"CODEOWNERS line {n}: {path!r} is not a path this check can match exactly")
        found.append(path[1:])
    return found


def owned(files, paths):
    """The files, of those changed, that a CODEOWNERS path covers."""
    return sorted(f for f in files if any(f.startswith(p) if p.endswith("/") else f == p for p in paths))


def verdict(event, files, paths):
    """None when the pull request may go on; otherwise what a person reads."""
    pull = event.get("pull_request") or {}
    author = (pull.get("user") or {}).get("login", "")
    owner = ((event.get("repository") or {}).get("owner") or {}).get("login", "")
    branch = (pull.get("head") or {}).get("ref", "")
    if not (author and author == owner and branch.startswith(SESSION_BRANCH)):
        return None
    hit = owned(files, paths)
    if not hit:
        return None
    return (
        f"This pull request, from the session branch {branch}, was opened by the owner's own account ({owner}) and "
        f"changes {len(hit)} code-owned file(s): {', '.join(hit)}. GitHub counts the owner's own pull request as "
        "reviewed by its code owner, so the review CODEOWNERS asks for would never be asked for. Close it, and open "
        "it again from a session on the owner's Mac as the sessions' App: python3 -I tools/session_token.py run -- "
        "gh pr create ... (CLAUDE.md; ADR-158, Later, 21 November 2026; backlog 0710)."
    )


def changed(base, head):
    out = subprocess.run(["git", "diff", "--name-only", "--no-renames", f"{base}...{head}"], cwd=ROOT,
                         capture_output=True, text=True, encoding="utf-8", check=True)
    return [line for line in out.stdout.splitlines() if line]


def self_test():
    paths = patterns("# a comment\n\n/.github/workflows/  @o\n/tools/a.py @o # trailing\n")
    assert paths == [".github/workflows/", "tools/a.py"], paths
    for bad in ("*.py @o", "tools/a.py @o", "/tools/*.py @o", "/ @o"):
        try:
            patterns(bad)
            raise AssertionError(f"read {bad!r}")
        except Unreadable:
            pass
    assert owned([".github/workflows/x.yml", "tools/a.py", "tools/a.py.bak", "tools/b.py"], paths) == [
        ".github/workflows/x.yml", "tools/a.py"]

    def event(author, branch, owner="o"):
        return {"pull_request": {"user": {"login": author}, "head": {"ref": branch}},
                "repository": {"owner": {"login": owner}}}

    hit = ["tools/a.py"]
    said = verdict(event("o", "claude/x"), hit, paths)
    assert said and "tools/a.py" in said and "session_token.py" in said, said
    # The App's pull request, the owner's own branch, another person's, or no code-owned file: each goes on.
    assert verdict(event("app/stackvet-sessions", "claude/x"), hit, paths) is None
    assert verdict(event("o", "fix-typo"), hit, paths) is None
    assert verdict(event("someone", "claude/x"), hit, paths) is None
    assert verdict(event("o", "claude/x"), ["docs/a.md"], paths) is None
    # An event with no pull request in it, or no owner, is never read as the owner's.
    assert verdict({}, hit, paths) is None
    assert verdict(event("", "claude/x", owner=""), hit, paths) is None
    # The committed CODEOWNERS is read whole, and names this script, so a change to it is reviewed too.
    committed = patterns(CODEOWNERS.read_text(encoding="utf-8"))
    assert "tools/cloud_session_check.py" in committed, committed
    print("cloud_session_check self-test: ok")
    return 0


def main(argv):
    parser = argparse.ArgumentParser(description=__doc__.strip().splitlines()[0])
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--base")
    parser.add_argument("--head")
    parser.add_argument("--event")
    args = parser.parse_args(argv)
    if args.self_test:
        return self_test()
    if not (args.base and args.head and args.event):
        parser.error("--base, --head, and --event are needed")
    try:
        paths = patterns(CODEOWNERS.read_text(encoding="utf-8"))
    except Unreadable as why:
        print(f"cloud_session_check: {why}; nothing was checked", file=sys.stderr)
        return 1
    event = json.loads(Path(args.event).read_text(encoding="utf-8"))
    said = verdict(event, changed(args.base, args.head), paths)
    if said:
        print(said, file=sys.stderr)
        return 1
    print("cloud_session_check: not a session's pull request opened by the owner's account that changes a code-owned "
          "file")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
