#!/usr/bin/env python3
"""Fails a pull request that changes what a decision record governs without saying what that means for the record.

Each of `sv`'s decision records (docs/adr/ADR-015.md onward) lists, on its `**Governs:**` line, the files whose
change can change the decision. A pull request that touches one of them must either change that record (a new
"Later" entry, a correction, a superseding record named in it) or say in its description, on a line of its own:

    ADR-026: unchanged, because the change only renames a test helper

with a reason of at least ten characters. The point is not to slow small changes down: it is that every change to a
decision's code is a moment somebody looks at the record, so the record stops falling behind what was built (the
owner's decision of 4 October 2026; BACKLOG, "Decision records written with the change, not after it").

A `*` in a pattern matches any run of characters, `/` included.

A pattern may name a part of a file, `path::NAME` (ADR-103): it matches only when the pull request's diff adds or removes
a line that contains NAME as a whole word (letters, digits and underscores on either side do not count), so a record can
govern one table or constant in a large shared file without every change to the file owing a line.

A Dependabot pull request whose every added or removed line is a pin (`PIN_LINES`: a step's commit fingerprint, a base
image's digest, a requirement's version and hashes, a lock file) owes nothing (ADR-128): the diff is what qualifies it,
so a commit pushed onto its branch that changes anything else makes the check ask as it does for anyone.

Usage:
    python3 tools/adr_check.py --base origin/main --head HEAD --event "$GITHUB_EVENT_PATH"
    python3 tools/adr_check.py --base origin/main --head HEAD --body-file description.md
    python3 tools/adr_check.py --self-test
"""

import argparse
import fnmatch
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ADR_DIR = "docs/adr"
UNCHANGED = re.compile(r"^\s*[-*]?\s*`?ADR-(\d{3})`?:\s*unchanged,\s*because\s+(.+?)\s*$", re.IGNORECASE | re.MULTILINE)
DEPENDABOT = "dependabot[bot]"
# The lines Dependabot writes when it moves a pin, and nothing else (ADR-128). Each is matched against a whole line.
PIN_LINES = [
    # A workflow step or an Action's step, pinned to a commit with its version beside it.
    re.compile(r"\s*(-\s+)?uses:\s+[\w.-]+/[\w./-]+@[0-9a-f]{40}\s+#\s+v?[\w.+-]+\s*"),
    # A base or helper image, pinned to a digest, with or without a tag and a stage name.
    re.compile(r"FROM\s+(--platform=\S+\s+)?[\w./-]+(:[\w.-]+)?@sha256:[0-9a-f]{64}(\s+AS\s+[\w-]+)?\s*", re.IGNORECASE),
    # A hashed requirement: its name and version, and each hash line under it.
    re.compile(r"[A-Za-z0-9][\w.-]*==[\w.+!-]+(\s*;[^\\]*)?\s*\\?\s*"),
    re.compile(r"\s+--hash=sha256:[0-9a-f]{64}\s*\\?\s*"),
]


def governs(text):
    """The patterns on a record's **Governs:** paragraph."""
    start = text.find("**Governs:**")
    if start < 0:
        return []
    paragraph = text[start:].split("\n\n")[0]
    # The list ends where the standing sentence begins, however the line was wrapped: ". A\npull request" is the same
    # sentence, and its footer names docs/adr/README.md, which is not a file the record governs (backlog 0267).
    listed = re.split(r"\.\)?\s+A\s+pull\s+request", paragraph)[0]
    return re.findall(r"`([^`]+)`", listed)


def records(read):
    """Record number to its patterns, for `sv`'s records. `read(path)` gives a file's text, or None."""
    out = {}
    for path in read.listing(ADR_DIR):
        m = re.fullmatch(r"docs/adr/ADR-(\d{3})\.md", path)
        if m and int(m.group(1)) >= 15:
            out[int(m.group(1))] = governs(read(path) or "")
    return out


def word_in(text, name):
    """Whether `name` appears in `text` as a whole word: no letter, digit or underscore on either side."""
    return re.search(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])", text or "") is not None


def touches(changed, pattern, lines):
    """The changed files a pattern reaches. `path::NAME` reaches a file only when a changed line has NAME as a word;
    with no changed lines to look at (`lines` is None) it reaches every file the path does, the safe side."""
    path, _, name = pattern.partition("::")
    reached = [f for f in changed if fnmatch.fnmatchcase(f, path)]
    if not name or lines is None:
        return reached
    return [f for f in reached if word_in(lines.get(f, ""), name)]


def is_pin(path, line):
    """Whether one line a change adds or removes in `path` is a pin Dependabot moves (ADR-128)."""
    name = path.rsplit("/", 1)[-1]
    if name == "Cargo.lock":
        return True
    # The requirements file's "# via" lines say which package pulled another in; a comment anywhere else asks.
    if name.endswith("requirements.txt") and re.fullmatch(r"\s+# via [\w.\- ,]+\s*", line):
        return True
    return any(p.fullmatch(line) for p in PIN_LINES)


def author(event):
    """The pull request's author, from the GitHub event file, or None."""
    return ((event.get("pull_request") or {}).get("user") or {}).get("login")


def pin_only(lines):
    """Whether a change's every added or removed line is a pin: `lines` maps a file to the text of those lines, as
    `changed_lines` gives it. A change with no lines to read (a rename, a mode change) is not pin-only: the safe side."""
    if not lines:
        return False
    return all(is_pin(path, line) for path, text in lines.items() for line in text.splitlines())


def owed(changed, patterns_by_record, body, lines=None):
    """The records a change touches and neither changes nor explains, with the files that touched each."""
    stated = {}
    for number, reason in UNCHANGED.findall(body or ""):
        if len(reason.strip()) >= 10:
            stated[int(number)] = reason.strip()
    out = {}
    for number, patterns in sorted(patterns_by_record.items()):
        record = f"{ADR_DIR}/ADR-{number:03d}.md"
        touched = []
        for pattern in patterns:
            touched += [f for f in touches(changed, pattern, lines) if f not in touched]
        if touched and record not in changed and number not in stated:
            out[number] = touched
    return out


class Git:
    """Reads files as they are at `head`."""

    def __init__(self, head):
        self.head = head

    def __call__(self, path):
        r = subprocess.run(["git", "show", f"{self.head}:{path}"], cwd=ROOT, capture_output=True, text=True, encoding="utf-8")
        return r.stdout if r.returncode == 0 else None

    def listing(self, folder):
        r = subprocess.run(["git", "ls-tree", "--name-only", f"{self.head}:{folder}"], cwd=ROOT,
                           capture_output=True, text=True, encoding="utf-8", check=True)
        return [f"{folder}/{name}" for name in r.stdout.split()]


def changed_files(base, head):
    r = subprocess.run(["git", "diff", "--name-only", f"{base}...{head}"], cwd=ROOT, capture_output=True,
                       text=True, encoding="utf-8", check=True)
    return [line for line in r.stdout.splitlines() if line]


def changed_lines(base, head, cwd=ROOT):
    """File to the text of the lines the change adds or removes there, for the `path::NAME` form of a pattern."""
    r = subprocess.run(["git", "diff", "-U0", "--no-color", f"{base}...{head}"], cwd=cwd, capture_output=True,
                       text=True, encoding="utf-8", errors="replace", check=True)
    out, current, removed, in_header = {}, None, None, False
    for line in r.stdout.splitlines():
        if line.startswith("diff --git "):
            in_header, current, removed = True, None, None
        elif in_header and line.startswith("--- "):
            removed = line[6:] if line.startswith("--- a/") else None
        elif in_header and line.startswith("+++ "):
            current = line[6:] if line.startswith("+++ b/") else removed
        elif line.startswith("@@"):
            in_header = False
        elif not in_header and current and line[:1] in ("+", "-"):
            out[current] = out.get(current, "") + line[1:] + "\n"
    return out


def report(missing, patterns_by_record):
    lines = ["This pull request changes what a decision record governs, and does not say what that means for it:", ""]
    for number, files in missing.items():
        lines.append(f"  ADR-{number:03d} (docs/adr/ADR-{number:03d}.md) governs: " + ", ".join(files))
    lines += [
        "",
        "For each, either change the record (a dated \"Later\" entry, a correction, or a new record that replaces it),",
        "or add a line to the pull request's description, such as:",
        "",
    ]
    for number in missing:
        lines.append(f"  ADR-{number:03d}: unchanged, because <what this change does, and why the decision stands>")
    lines += ["", "The reason needs at least ten characters. See docs/adr/README.md."]
    return "\n".join(lines)


def self_test():
    patterns = {19: ["crates/sv-run/src/docker.rs"], 20: ["Cargo.toml", "crates/*/Cargo.toml"], 26: ["crates/sv-check/src/seal.rs"]}
    # Nothing governed: nothing owed.
    assert owed(["docs/BACKLOG.md"], patterns, "") == {}
    # A governed file, with nothing said: owed.
    assert owed(["crates/sv-run/src/docker.rs"], patterns, "") == {19: ["crates/sv-run/src/docker.rs"]}
    # The record changed with it: not owed.
    assert owed(["crates/sv-run/src/docker.rs", "docs/adr/ADR-019.md"], patterns, "") == {}
    # Said why, at length: not owed.
    assert owed(["crates/sv-run/src/docker.rs"], patterns, "ADR-019: unchanged, because only a log line moved") == {}
    assert owed(["crates/sv-run/src/docker.rs"], patterns, "- `ADR-019`: Unchanged, because only a log line moved") == {}
    # Said why too briefly, or about another record: still owed.
    assert 19 in owed(["crates/sv-run/src/docker.rs"], patterns, "ADR-019: unchanged, because ok")
    assert 19 in owed(["crates/sv-run/src/docker.rs"], patterns, "ADR-026: unchanged, because only a log line moved")
    # A pattern's `*` crosses folders, and each record is owed separately.
    got = owed(["crates/sv-cli/Cargo.toml", "crates/sv-check/src/seal.rs"], patterns, "")
    assert set(got) == {20, 26}, got
    # A record that changes is never owed, even when it governs itself.
    assert owed(["docs/adr/ADR-026.md"], {26: ["docs/adr/*"]}, "") == {}
    # Reading the list off a record.
    text = "**Status:** x.\n\n**Governs:** `a/b.rs`, `c/*`. A pull request that changes any of these.\n\n`d/e.rs`"
    assert governs(text) == ["a/b.rs", "c/*"], governs(text)
    assert governs("no list here") == []
    # The standing sentence wrapped across lines still ends the list, so its footer is not read as a governed file.
    wrapped = "**Governs:** `a/b.rs`. A\npull request that changes any of these changes this record too\n(`docs/adr/README.md`).\n\nx"
    assert governs(wrapped) == ["a/b.rs"], governs(wrapped)
    parenthesis = "**Governs:** `a/b.rs` (said later.) A pull request that changes these changes this record too (`docs/adr/README.md`)."
    assert governs(parenthesis) == ["a/b.rs"], governs(parenthesis)
    # A part of a file (ADR-103): it fires only when the change touches a line with the name as a whole word.
    part = {21: ["crates/a/big.rs::TABLE"]}
    quiet = {"crates/a/big.rs": "let x = 1;\n"}
    assert owed(["crates/a/big.rs"], part, "", quiet) == {}, "a change elsewhere in the file owes nothing"
    assert owed(["crates/a/big.rs"], part, "", {"crates/a/big.rs": "const TABLE: u8 = 1;\n"}) == {21: ["crates/a/big.rs"]}
    assert owed(["crates/a/big.rs"], part, "", {"crates/a/big.rs": "let LONG_TABLE = 1; let TABLE_2 = 2; TABLE_\n"}) == {}, "a longer name is not the name"
    assert owed(["crates/a/big.rs"], part, "", {"crates/a/big.rs": "x(TABLE)\n"}) == {21: ["crates/a/big.rs"]}, "next to punctuation it is the name"
    assert owed(["crates/a/other.rs"], part, "", {"crates/a/other.rs": "const TABLE: u8 = 1;\n"}) == {}, "another file owes nothing"
    assert owed(["crates/a/big.rs"], part, "") == {21: ["crates/a/big.rs"]}, "with no lines to read it asks, the safe side"
    assert owed(["crates/a/big.rs"], part, "ADR-021: unchanged, because only the table's comment moved", {"crates/a/big.rs": "TABLE\n"}) == {}
    assert owed(["crates/a/big.rs"], {21: ["crates/a/big.rs::TABLE", "crates/a/big.rs"]}, "", quiet) == {21: ["crates/a/big.rs"]}, "a whole-file pattern still asks"
    # Reading the changed lines off a diff, a deleted file's included, and a header line never taken for content.
    assert word_in("a TABLE b", "TABLE") and not word_in("aTABLE", "TABLE") and not word_in("TABLE1", "TABLE")
    # The same, through a real repository: an edited file, a deleted one, and a content line that looks like a header.
    import tempfile
    with tempfile.TemporaryDirectory() as tmp:
        def put(name, text):
            with open(Path(tmp, name), "w", encoding="utf-8", newline="\n") as f:
                f.write(text)

        def git(*a):
            subprocess.run(["git", "-c", "user.name=t", "-c", "user.email=t@t", *a], cwd=tmp, check=True, capture_output=True)
        git("init", "-q", "-b", "main")
        put("big.rs", "const A: u8 = 1;\nconst B: u8 = 2;\n-- not a header\n")
        put("gone.rs", "const GONE: u8 = 1;\n")
        git("add", "."); git("commit", "-q", "-m", "one")
        put("big.rs", "const A: u8 = 1;\nconst B: u8 = 3;\n--- still not a header\n")
        Path(tmp, "gone.rs").unlink()
        git("add", "-A"); git("commit", "-q", "-m", "two")
        got = changed_lines("HEAD~1", "HEAD", cwd=tmp)
        assert set(got) == {"big.rs", "gone.rs"}, got
        assert word_in(got["big.rs"], "B") and not word_in(got["big.rs"], "A"), got
        assert word_in(got["gone.rs"], "GONE"), got
        assert "still" in got["big.rs"] and "-- not a header" in got["big.rs"], got
    # A pin-only change (ADR-128), from lines Dependabot wrote in #2140, #2142 and #2145.
    sha = "cf430e030ddbb5b0abf93d22962f4752f3646cd9"
    digest = "7c1809255e25a77a464ed1c205c4ad0ce5d9f4f8c25e119afc11eaf7d41b3f1b"
    step = f"        uses: actions/upload-artifact@{sha} # v7.0.2\n"
    image = f"FROM docker.io/chromedp/headless-shell:156.0.8078.4@sha256:{digest} AS browser\n"
    requirement = f"pycparser==3.11 ; implementation_name != 'PyPy' \\\n    --hash=sha256:{digest} \\\n    --hash=sha256:{digest}\n"
    assert pin_only({".github/workflows/rust.yml": step, "action.yml": step.replace("upload", "download")})
    assert pin_only({"crates/sv-run/images/Dockerfile": image, "Dockerfile": f"FROM debian:trixie-slim@sha256:{digest}\n"})
    assert pin_only({"tools/py7zr-requirements.txt": requirement})
    assert pin_only({"Cargo.lock": 'name = "tree-sitter-php"\nversion = "0.25.0"\n', "fuzz/Cargo.lock": 'checksum = "ab12"\n'})
    # Anything else asks: a requirement in a manifest, a step's other lines, a tag with no digest, an unpinned step.
    assert not pin_only({}), "nothing to read is not pin-only"
    assert not pin_only({"Cargo.toml": 'tree-sitter-php = "0.25"\n', "Cargo.lock": 'version = "0.25.0"\n'})
    assert not pin_only({".github/workflows/rust.yml": step + "        run: curl https://example.com | sh\n"})
    assert not pin_only({"Dockerfile": "FROM debian:trixie-slim\n"})
    assert not pin_only({".github/workflows/rust.yml": "        uses: actions/upload-artifact@v7\n"})
    assert not pin_only({"tools/not_Cargo.lock.py": "x = 1\n"}), "a lock file's name inside another name is not a lock file"
    assert pin_only({"tools/py7zr-requirements.txt": requirement + "    # via py7zr\n"})
    assert not pin_only({".github/workflows/rust.yml": step + "        # via py7zr\n"}), "a comment elsewhere asks"
    # Only Dependabot's pull requests are read this way.
    assert author({"pull_request": {"user": {"login": DEPENDABOT}}}) == DEPENDABOT
    assert author({"pull_request": {}}) is None and author({}) is None
    print("adr_check self-test: ok")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--base")
    parser.add_argument("--head", default="HEAD")
    parser.add_argument("--event", help="the GitHub event file, for the pull request's description")
    parser.add_argument("--body-file", help="a file holding the pull request's description")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return 0
    if not args.base:
        parser.error("--base is needed")
    body, event = "", {}
    if args.event:
        event = json.loads(Path(args.event).read_text(encoding="utf-8"))
        body = (event.get("pull_request") or {}).get("body") or ""
    elif args.body_file:
        body = Path(args.body_file).read_text(encoding="utf-8")
    patterns = records(Git(args.head))
    changed = changed_files(args.base, args.head)
    named = any("::" in p for ps in patterns.values() for p in ps)
    dependabot = author(event) == DEPENDABOT
    lines = changed_lines(args.base, args.head) if named or dependabot else None
    if dependabot and pin_only(lines):
        print("Decision records: nothing owed; a Dependabot pull request whose every line is a pin (ADR-128).")
        return 0
    missing = owed(changed, patterns, body, lines)
    if missing:
        print(report(missing, patterns))
        return 1
    print(f"Decision records: nothing owed ({len(patterns)} records read).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
