#!/usr/bin/env python3
"""The numbers two pull requests can both take, seen before either merges (backlog 0265; ADR-061, Later).

Three kinds of file are numbered, and each number belongs to one file: decision records (docs/adr/ADR-NNN.md),
backlog items (docs/backlog/NNNN-*.md, done/ included), and design entries (docs/design/NNNN-*.md). Each session takes
the next number from what it can see, so two open pull requests could take the same one, and only the second to merge
found out, when `main` was merged in and the two files met. Two files with different names and one number do not even
conflict in git: on 12 October 2026 two design entries numbered 0376 merged side by side.

`check` fails a pull request that
  - holds two files with one number, one of them not on the base branch (a collision brought in by a merge; the
    36 numbers taken twice on `main` before 12 October 2026 stay as they are, since their files are cited by path),
  - adds a number the base branch already holds, or
  - adds a number that another open pull request, opened before it, adds too (with --github, in CI),
naming the file or the pull request that holds it. The one opened first keeps the number.

`next` prints the next number free on this branch and every branch pushed to origin, after `git fetch`. Neither
sees a number a session holds without having pushed it: a claim pushed early is what makes a number visible.

Usage:
    python3 tools/record_numbers.py next adr|backlog|design
    python3 tools/record_numbers.py check --base origin/main [--head HEAD]
    python3 tools/record_numbers.py check --base "$BASE_SHA" --head "$HEAD_SHA" --event "$GITHUB_EVENT_PATH" --github
    python3 tools/record_numbers.py --self-test
"""

import argparse
import json
import os
import re
import subprocess
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

KINDS = {
    "adr": re.compile(r"^docs/adr/ADR-(\d{3})\.md$"),
    "backlog": re.compile(r"^docs/backlog/(?:done/)?(\d{4})-[^/]+\.md$"),
    "design": re.compile(r"^docs/design/(\d{4})-[^/]+\.md$"),
}
FOLDERS = ["docs/adr", "docs/backlog", "docs/design"]


def name(kind, number):
    return f"ADR-{number:03d}" if kind == "adr" else f"{kind} {number:04d}"


def numbered(paths):
    """Each numbered file among `paths`, as {(kind, number): [paths]}."""
    found = {}
    for path in paths:
        for kind, pattern in KINDS.items():
            m = pattern.match(path)
            if m:
                found.setdefault((kind, int(m.group(1))), []).append(path)
    return found


def git(*args, cwd=None):
    r = subprocess.run(["git", *args], cwd=cwd or ROOT, capture_output=True, text=True, encoding="utf-8")
    if r.returncode != 0:
        raise SystemExit(f"git {' '.join(args)}: {r.stderr.strip()}")
    return r.stdout


def files_at(ref, cwd=None):
    return [p for p in git("ls-tree", "-r", "--name-only", ref, "--", *FOLDERS, cwd=cwd).splitlines() if p]


def new_numbers(base, head, cwd=None):
    """The numbers `head` adds: those it holds that the commit it branched from did not. An item only moved (to
    done/, say) keeps its number and is not new, so another pull request moving it never stops this one."""
    start = git("merge-base", base, head, cwd=cwd).strip()
    return set(numbered(files_at(head, cwd))) - set(numbered(files_at(start, cwd)))


def problems(head, base, new, others, me):
    """What stops this pull request, in words. `head` and `base` are {(kind, n): [paths]} for the branch and for the
    base branch's tip; `new` the (kind, n) this pull request adds (those not where it branched from); `others` the
    other open pull requests, each {"number", "created_at", "paths"}; `me` this one's {"number", "created_at"}."""
    said = []
    for (kind, number), paths in sorted(head.items()):
        # Numbers taken twice before this check existed stay as they are: their files are cited by path. Only a
        # pair this branch brings in, one file of which the base branch does not have, is said.
        if len(paths) > 1 and any(p not in base.get((kind, number), []) for p in paths):
            said.append(f"{name(kind, number)} is taken by {len(paths)} files here: {', '.join(paths)}. "
                        "Renumber all but one.")
    for key in sorted(new):
        kind, number = key
        on_base = [p for p in base.get(key, []) if p not in head.get(key, [])]
        if on_base:
            said.append(f"{name(kind, number)} is new here, and the base branch already has it: {on_base[0]}. "
                        "Renumber this one.")
            continue
        for other in sorted(others, key=lambda o: (o["created_at"], o["number"])):
            if other["number"] == me["number"]:
                continue
            first = (other["created_at"], other["number"]) < (me["created_at"], me["number"])
            if first and key in numbered(other["paths"]):
                said.append(f"{name(kind, number)} is also added by pull request #{other['number']}, opened first, "
                            f"so it keeps the number. Renumber this one before it merges "
                            f"(`python3 tools/record_numbers.py next {kind}`).")
                break
    return said


def open_pulls(repository, token):
    """Every open pull request in `repository`, with the files each adds or renames into place, from GitHub."""
    def get(url):
        out, page = [], 1
        while True:
            req = urllib.request.Request(f"{url}{'&' if '?' in url else '?'}per_page=100&page={page}", headers={
                "Accept": "application/vnd.github+json", "Authorization": f"Bearer {token}",
                "X-GitHub-Api-Version": "2022-11-28"})
            with urllib.request.urlopen(req, timeout=30) as r:
                got = json.load(r)
            out += got
            if len(got) < 100:
                return out
            page += 1
    api = f"https://api.github.com/repos/{repository}"
    pulls = []
    for pr in get(f"{api}/pulls?state=open"):
        files = get(f"{api}/pulls/{pr['number']}/files")
        pulls.append({"number": pr["number"], "created_at": pr["created_at"],
                      "paths": [f["filename"] for f in files if f.get("status") in ("added", "renamed", "copied")]})
    return pulls


def check(base, head, event=None, github=False):
    head_files = numbered(files_at(head))
    base_files = numbered(files_at(base))
    new = new_numbers(base, head)
    me, others = {"number": 0, "created_at": "9999"}, []
    if event:
        pr = json.loads(Path(event).read_text(encoding="utf-8")).get("pull_request") or {}
        me = {"number": pr.get("number", 0), "created_at": pr.get("created_at", "9999")}
    if github:
        others = open_pulls(os.environ["GITHUB_REPOSITORY"], os.environ["GITHUB_TOKEN"])
    said = problems(head_files, base_files, new, others, me)
    if said:
        print("Two files cannot share a number:\n\n  " + "\n  ".join(said))
        return 1
    print(f"Record numbers: {len(new)} new here, none taken twice"
          + (f" ({len(others)} open pull requests read)." if github else "."))
    return 0


def next_free(kind):
    """One past the highest number of `kind` on this branch and on every branch pushed to origin, and the branch
    that holds that highest number."""
    refs = ["HEAD"] + [r for r in git("for-each-ref", "--format=%(refname)", "refs/remotes/origin").splitlines()
                       if not r.endswith("/HEAD")]
    best, where = 0, "HEAD"
    for ref in refs:
        for k, number in numbered(files_at(ref)):
            if k == kind and number > best:
                best, where = number, ref
    return best + 1, where


def self_test():
    paths = ["docs/adr/ADR-099.md", "docs/adr/README.md", "docs/backlog/0265-a.md", "docs/backlog/done/0249-b.md",
             "docs/design/0376-x.md", "docs/design/0376-y.md", "docs/adr/ADR-99.md", "docs/backlog/0265-a.txt"]
    got = numbered(paths)
    assert set(got) == {("adr", 99), ("backlog", 265), ("backlog", 249), ("design", 376)}, got
    assert got[("design", 376)] == ["docs/design/0376-x.md", "docs/design/0376-y.md"]
    me = {"number": 20, "created_at": "2026-10-12T10:00:00Z"}
    head = numbered(["docs/adr/ADR-104.md", "docs/design/0377-mine.md"])
    # Nothing taken: nothing said.
    assert problems(head, {}, set(head), [], me) == []
    # Two files with one number, brought in by a merge: said, though neither is new.
    two = numbered(["docs/design/0376-mine.md", "docs/design/0376-theirs.md"])
    said = problems(two, numbered(["docs/design/0376-theirs.md"]), set(), [], me)
    assert len(said) == 1 and "taken by 2 files" in said[0], said
    # Both already on the base branch, from before this check: left as they are.
    assert problems(two, two, set(), [], me) == []
    # The base branch took it since this one branched.
    said = problems(head, numbered(["docs/adr/ADR-104.md", "docs/adr/ADR-104x.md", "docs/design/0377-theirs.md"]),
                    set(head), [], me)
    assert len(said) == 1 and "design 0377" in said[0] and "0377-theirs.md" in said[0], said
    # Another open pull request, opened first, adds it: it keeps the number, and this one is told to renumber.
    earlier = {"number": 19, "created_at": "2026-10-12T09:00:00Z", "paths": ["docs/adr/ADR-104.md"]}
    said = problems(head, {}, set(head), [earlier], me)
    assert len(said) == 1 and "#19" in said[0] and "ADR-104" in said[0], said
    # Opened later: this one keeps it, and the later one is told when its own check runs.
    later = dict(earlier, number=21, created_at="2026-10-12T11:00:00Z")
    assert problems(head, {}, set(head), [later], me) == []
    said = problems(head, {}, set(head), [dict(me, paths=["docs/adr/ADR-104.md"])],
                    {"number": 21, "created_at": later["created_at"]})
    assert len(said) == 1 and "#20" in said[0], said
    # A number this pull request did not add (an item it only moved to done/) is never its to give up.
    moved = numbered(["docs/backlog/done/0249-b.md"])
    other = {"number": 19, "created_at": "2026-10-12T09:00:00Z", "paths": ["docs/backlog/done/0249-b.md"]}
    assert problems(moved, moved, set(), [other], me) == []
    # Its own entry in the list is not another pull request.
    assert problems(head, {}, set(head), [dict(earlier, number=20)], me) == []
    # What a branch adds, read from git: an item moved to done/ is not new, one written is.
    import tempfile
    with tempfile.TemporaryDirectory() as tmp:
        def g(*args):
            return git("-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", *args, cwd=tmp)
        g("init", "-q", "-b", "main")
        (Path(tmp) / "docs/backlog/done").mkdir(parents=True)
        (Path(tmp) / "docs/backlog/0001-a.md").write_text("# a\n")
        g("add", "-A"); g("commit", "-q", "-m", "base")
        g("switch", "-q", "-c", "work")
        g("mv", "docs/backlog/0001-a.md", "docs/backlog/done/0001-a.md")
        (Path(tmp) / "docs/backlog/0002-b.md").write_text("# b\n")
        g("add", "-A"); g("commit", "-q", "-m", "work")
        got = new_numbers("main", "work", cwd=tmp)
        assert got == {("backlog", 2)}, got
    print("record_numbers self-test: ok")


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("command", nargs="?", choices=["check", "next"])
    parser.add_argument("kind", nargs="?", choices=sorted(KINDS))
    parser.add_argument("--base")
    parser.add_argument("--head", default="HEAD")
    parser.add_argument("--event", help="the GitHub event file, for this pull request's number and when it opened")
    parser.add_argument("--github", action="store_true",
                        help="also read the other open pull requests (GITHUB_REPOSITORY and GITHUB_TOKEN)")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return 0
    if args.command == "next":
        if not args.kind:
            parser.error("next needs a kind: adr, backlog, or design")
        number, where = next_free(args.kind)
        print(name(args.kind, number))
        if where != "HEAD":
            print(f"(the highest, {name(args.kind, number - 1)}, is on {where.removeprefix('refs/remotes/')})",
                  file=sys.stderr)
        return 0
    if args.command == "check":
        if not args.base:
            parser.error("check needs --base")
        return check(args.base, args.head, args.event, args.github)
    parser.error("say check or next")


if __name__ == "__main__":
    sys.exit(main())
