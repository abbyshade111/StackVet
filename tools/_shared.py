"""What the scripts in tools/ share, so a change to how they write a file or name one is made once (backlog 0464).

Not a tool of its own: nothing runs it. A script imports it after putting its own folder first on `sys.path`, since
`python3 -I`, as CI runs the tools, leaves the script's folder off it:

    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from _shared import write_text  # noqa: E402

`pwned_passwords.py` keeps its own `write_text`: CodeQL's alerts on the public password it writes were dismissed
where they stand, in that file (its comment says more).

Each script's `git` helper stays its own: the three differ in what they return and how they fail, and their callers
rely on each (coordination.py catches CalledProcessError; merge_main.py returns the whole result and can be asked not
to fail; record_numbers.py exits with git's message).
"""

import re
from pathlib import Path


def write_text(path, text, make_folder=False):
    """Writes `text` as UTF-8 with Unix line endings on every system. Path.write_text uses the system's own encoding
    and, on Windows, CRLF, which would make a generated file differ from the one committed (backlog 0120;
    crates/sv-cli/tests/tools_text.rs). With `make_folder`, the folder it goes in is made first."""
    if make_folder:
        Path(path).parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as out:
        out.write(text)


def slug(title):
    """A title made into a file name: lowercase letters and digits, joined by hyphens, whole words up to 60
    characters (the first word cut at 60 when it alone is longer), as backlog items and design entries are named."""
    words = [w for w in re.split(r"[^a-z0-9]+", title.lower()) if w]
    name = ""
    for word in words:
        if name and len(name) + 1 + len(word) > 60:
            break
        name = f"{name}-{word}" if name else word[:60]
    return name


def self_test():
    assert slug("A title, with: punctuation") == "a-title-with-punctuation"
    assert len(slug("x" * 80)) == 60, "a first word over 60 characters is cut"
    long = slug("word " * 20)
    assert len(long) <= 60 and not long.endswith("-") and long.startswith("word-word"), long
    import tempfile
    with tempfile.TemporaryDirectory() as tmp:
        path = Path(tmp) / "new" / "f.txt"
        write_text(path, "a\nb\n", make_folder=True)
        assert path.read_bytes() == b"a\nb\n", "UTF-8, LF, and the folder made"
    print("_shared self-test: ok")


if __name__ == "__main__":
    self_test()
