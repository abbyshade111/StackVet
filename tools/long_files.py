#!/usr/bin/env python3
"""The Rust files and functions long enough to split, largest first (backlog 0662; ADR-152).

Run each Monday by `.github/workflows/long-files.yml`, and by hand. It lists every Rust file in `crates/` over
`FILE_LINES` lines and every function over `FUNCTION_LINES` lines, with test files, test modules, and test fakes
listed apart, since a long test fake is a different job from a long module. It reports and never fails: a file on
the list is work a session takes the normal way, a backlog item each, and nothing here files one.

A function's length is read by its braces, with strings, characters, and comments taken out first, so a brace in
`"{}"`, in `'{'`, or in a comment is not counted. `#[cfg(test)]` before a module, and `#[test]` before a function,
mark test code; so does a file whose name ends in `tests.rs`, sits in a `tests/` folder, or names a fake.

Usage:
    python3 tools/long_files.py                    # the list, as Markdown, for a person (and the run's summary)
    python3 tools/long_files.py --json             # the same, as JSON, for the test
    python3 tools/long_files.py --thresholds       # the two numbers, as JSON
    python3 tools/long_files.py --root DIR         # another tree, for the test
"""

import json
import os
import re
import sys
from pathlib import Path

# The two numbers backlog 0324 used, in the one place the job and its test both read.
FILE_LINES = 3000
FUNCTION_LINES = 150

ROOT = Path(__file__).resolve().parent.parent

FN = re.compile(
    r"^[ \t]*(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|async|unsafe|extern(?:\s+\"[^\"]*\")?)\s+)*fn\s+([A-Za-z_][A-Za-z0-9_]*)",
    re.M,
)
TEST_MODULE = re.compile(r"#\[cfg\((?:all\()?test[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*\{")
TEST_ATTRIBUTE = re.compile(r"#\[(?:test|tokio::test[^\]]*|test_case[^\]]*)\]\s*$")


def stripped(code):
    """`code` with every string, raw string, character, and comment made spaces, newlines kept, so its braces and
    its line numbers can be read as they are."""
    out = []
    i, n = 0, len(code)

    def blank(text):
        return "".join("\n" if c == "\n" else " " for c in text)

    while i < n:
        c = code[i]
        if code.startswith("//", i):
            end = code.find("\n", i)
            end = n if end == -1 else end
            out.append(blank(code[i:end]))
            i = end
        elif code.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if code.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif code.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            out.append(blank(code[i:j]))
            i = j
        elif c in "rb" and re.match(r'b?r#*"', code[i:i + 300]) and (i == 0 or not (code[i - 1].isalnum() or code[i - 1] == "_")):
            m = re.match(r'b?r(#*)"', code[i:])
            close = '"' + m.group(1)
            end = code.find(close, i + m.end())
            end = n if end == -1 else end + len(close)
            out.append(blank(code[i:end]))
            i = end
        elif c == '"' or (c == "b" and code.startswith('b"', i) and (i == 0 or not code[i - 1].isalnum())):
            j = i + (2 if c == "b" else 1)
            while j < n and code[j] != '"':
                j += 2 if code[j] == "\\" else 1
            out.append(blank(code[i:j + 1]))
            i = j + 1
        elif c == "'":
            # A character (`'{'`, `'\''`, `'\u{7b}'`), or else a lifetime, which is left as it is.
            m = re.match(r"'(?:\\u\{[0-9a-fA-F]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", code[i:])
            if m:
                out.append(blank(m.group(0)))
                i += m.end()
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


def closing(text, at):
    """The index just past the brace that closes the one at `at`, or `len(text)` when none does."""
    depth = 0
    for j in range(at, len(text)):
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            depth -= 1
            if depth == 0:
                return j + 1
    return len(text)


def line_of(text, at):
    return text.count("\n", 0, at) + 1


def is_test_file(relative):
    name = relative.name
    return (
        name.endswith("tests.rs")
        or "tests" in relative.parts[:-1]
        or "fake" in name
    )


def functions(code, test_file):
    """Each function in `code` as `(name, first line, lines, in test code)`."""
    text = stripped(code)
    test_spans = [(m.end() - 1, closing(text, m.end() - 1)) for m in TEST_MODULE.finditer(text)]
    out = []
    for m in FN.finditer(text):
        # The body's opening brace, past the signature; a `;` first is a declaration with no body.
        j = m.end()
        while j < len(text) and text[j] not in "{;":
            j += 1
        if j >= len(text) or text[j] == ";":
            continue
        end = closing(text, j)
        first, last = line_of(text, m.start()), line_of(text, end - 1)
        before = text[: m.start()].rstrip().rsplit("\n", 1)[-1]
        in_test = (
            test_file
            or any(a <= m.start() < b for a, b in test_spans)
            or bool(TEST_ATTRIBUTE.search(before))
        )
        out.append((m.group(1), first, last - first + 1, in_test))
    return out


def long_ones(root):
    """What is over the lines under `root`'s `crates/`: `{"files": [...], "functions": [...]}`, largest first, each
    with whether it is test code."""
    files, fns = [], []
    for path in sorted((root / "crates").rglob("*.rs")):
        relative = path.relative_to(root)
        if "target" in relative.parts:
            continue
        code = path.read_text(encoding="utf-8", errors="replace")
        test_file = is_test_file(relative)
        lines = code.count("\n") + (0 if code.endswith("\n") or not code else 1)
        if lines > FILE_LINES:
            files.append({"file": str(relative), "lines": lines, "test": test_file})
        for name, first, length, in_test in functions(code, test_file):
            if length > FUNCTION_LINES:
                fns.append({"file": str(relative), "function": name, "line": first, "lines": length, "test": in_test})
    files.sort(key=lambda f: (-f["lines"], f["file"]))
    fns.sort(key=lambda f: (-f["lines"], f["file"], f["line"]))
    return {"files": files, "functions": fns}


def markdown(found):
    out = [
        "# Long files and functions to split",
        "",
        f"Rust files in `crates/` over {FILE_LINES:,} lines, and functions over {FUNCTION_LINES} lines, largest "
        "first. A long one is work to plan, a backlog item each, not a fault: this list never fails the run.",
        "",
    ]

    def section(title, rows, show):
        out.append(f"## {title}")
        out.append("")
        if not rows:
            out.append("None.")
        for row in rows:
            out.append(show(row))
        out.append("")

    file_row = lambda f: f"- `{f['file']}`: {f['lines']:,} lines"
    fn_row = lambda f: f"- `{f['function']}` in `{f['file']}`, line {f['line']}: {f['lines']} lines"
    section("Files", [f for f in found["files"] if not f["test"]], file_row)
    section("Functions", [f for f in found["functions"] if not f["test"]], fn_row)
    section("Test files and fakes", [f for f in found["files"] if f["test"]], file_row)
    section("Functions in test code", [f for f in found["functions"] if f["test"]], fn_row)
    return "\n".join(out)


def main(argv):
    root = ROOT
    if "--root" in argv:
        root = Path(argv[argv.index("--root") + 1]).resolve()
    if "--thresholds" in argv:
        print(json.dumps({"file_lines": FILE_LINES, "function_lines": FUNCTION_LINES}))
        return 0
    found = long_ones(root)
    if "--json" in argv:
        print(json.dumps(found, indent=2))
        return 0
    text = markdown(found)
    print(text)
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a", encoding="utf-8") as f:
            f.write(text + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
