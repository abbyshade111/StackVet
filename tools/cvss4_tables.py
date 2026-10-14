#!/usr/bin/env python3
"""Writes the CVSS v4 tables `sv` scores with, copied from FIRST's reference calculator, and the reference scores
its tests are held to.

CVSS v4 is not a formula: a vector is placed in one of 270 groups whose scores FIRST's experts assigned, and those
scores have to be copied (ADR-033). This copies them from a checkout of github.com/FIRSTdotorg/cvss-v4-calculator
into crates/sv-check/src/cvss4_tables.rs, keeping the copyright notice and the BSD-2-Clause license, and asks the
calculator's own JavaScript to score a fixed sample of vectors into crates/sv-check/tests/fixtures/cvss4-reference.txt.
With --all it scores every base vector, with and without each threat value, to a file of your choosing instead, for
a check of every vector there is (`sv`'s test reads it when SV_CVSS4_ALL names it).

Needs `node`, and the network only to clone the calculator. Not run by any test but its own --self-test.

The checkout must be at the commit the committed tables name in their first lines, the one they were copied from:
a clone at another commit would write different tables with that other commit in the header, and nothing would
notice (backlog 0322). To copy from a newer commit on purpose, name it with --commit, which must also be the
checkout's own; the new tables then name it, and the next run holds to that one.

Usage:
    git clone https://github.com/FIRSTdotorg/cvss-v4-calculator /tmp/cvss4
    git -C /tmp/cvss4 checkout <the commit in crates/sv-check/src/cvss4_tables.rs>
    python3 tools/cvss4_tables.py /tmp/cvss4
    python3 tools/cvss4_tables.py /tmp/cvss4 --all /tmp/cvss4-all.txt
    python3 tools/cvss4_tables.py /tmp/cvss4 --commit <a newer commit, checked out>   # an update, on purpose
    python3 tools/cvss4_tables.py --self-test
"""
import itertools
import json
import pathlib
import random
import re
import subprocess
import sys


def write_text(path, text):
    """Writes `text` as UTF-8 with Unix line endings on every system. Path.write_text uses the
    system's own encoding and, on Windows, CRLF, which would make a generated file differ from the
    one committed (backlog 0120)."""
    with open(path, "w", encoding="utf-8", newline="\n") as out:
        out.write(text)

ROOT = pathlib.Path(__file__).resolve().parent.parent
TABLES = ROOT / "crates/sv-check/src/cvss4_tables.rs"
FIXTURE = ROOT / "crates/sv-check/tests/fixtures/cvss4-reference.txt"

BASE = [
    ("AV", "NALP"), ("AC", "LH"), ("AT", "NP"), ("PR", "NLH"), ("UI", "NPA"),
    ("VC", "HLN"), ("VI", "HLN"), ("VA", "HLN"), ("SC", "HLN"), ("SI", "HLN"), ("SA", "HLN"),
]
THREAT = ["", "/E:A", "/E:P", "/E:U"]

# Loads the calculator's files into one scope, as its page does, and scores each vector given on stdin the way
# its page does: every metric at its default, then the vector's own values over them.
SCORER = r"""
const fs = require('fs'), vm = require('vm'), path = require('path');
const dir = process.argv[1];
const ctx = {console};
vm.createContext(ctx);
for (const f of ['cvss_config.js', 'cvss_lookup.js', 'max_composed.js', 'max_severity.js', 'cvss_score.js']) {
  vm.runInContext(fs.readFileSync(path.join(dir, f), 'utf8'), ctx);
}
vm.runInContext(`
  function defaults() {
    const s = {};
    for (const t of Object.values(cvssConfig))
      for (const g of Object.values(t.metric_groups))
        for (const m of Object.values(g)) s[m.short] = m.selected;
    return s;
  }
  function scoreOf(vector) {
    const s = defaults();
    for (const part of vector.split('/').slice(1)) { const [k, v] = part.split(':'); s[k] = v; }
    return cvss_score(s, cvssLookup_global, maxSeverity, macroVector(s));
  }`, ctx);
const lines = fs.readFileSync(0, 'utf8').split('\n').filter(Boolean);
process.stdout.write(lines.map((v) => v + ' ' + ctx.scoreOf(v).toFixed(1)).join('\n') + '\n');
"""

TABLE_DUMP = r"""
const fs = require('fs'), vm = require('vm'), path = require('path');
const dir = process.argv[1];
const ctx = {};
vm.createContext(ctx);
for (const f of ['cvss_lookup.js', 'max_composed.js', 'max_severity.js']) {
  vm.runInContext(fs.readFileSync(path.join(dir, f), 'utf8'), ctx);
}
console.log(JSON.stringify({lookup: ctx.cvssLookup_global, composed: ctx.maxComposed, severity: ctx.maxSeverity}));
"""


def node(script, checkout, stdin=""):
    out = subprocess.run(["node", "-e", script, str(checkout)], input=stdin, capture_output=True, text=True, encoding="utf-8")
    if out.returncode != 0:
        sys.exit(out.stderr)
    return out.stdout


def pinned_commit(tables_text):
    """The commit the committed tables say they were copied from, or None when their header names none."""
    found = re.search(r"cvss-v4-calculator at commit ([0-9a-f]{40})\b", tables_text)
    return found.group(1) if found else None


def refusal(head, pinned, asked):
    """Why a checkout at `head` must not be copied from, or None when it may. `pinned` is the committed tables'
    commit, `asked` the one --commit names (None when not given)."""
    if not head:
        return "the checkout's commit could not be read (`git rev-parse HEAD` gave nothing): is it a git clone?"
    if asked is not None:
        if head != asked:
            return f"--commit names {asked}, but the checkout is at {head}; check out the commit you name."
        return None
    if pinned is None:
        return ("the committed tables name no commit to hold the checkout to; name the checkout's own with "
                f"--commit {head} if it is the one to copy from.")
    if head != pinned:
        return (f"the checkout is at {head}, and the committed tables were copied from {pinned}. Check out "
                f"{pinned} to write them again, or pass --commit {head} to copy from this commit on purpose.")
    return None


def self_test():
    a, b = "a" * 40, "b" * 40
    header = f"//! github.com/FIRSTdotorg/cvss-v4-calculator at commit {a} (ADR-033). Do not edit by hand.\n"
    assert pinned_commit(header) == a
    assert pinned_commit("//! no commit here\n") is None
    assert refusal(a, a, None) is None, "the pinned commit is copied from"
    assert refusal(b, a, None) is not None, "another commit is refused"
    assert refusal(b, a, b) is None, "another commit named with --commit is copied from"
    assert refusal(a, a, b) is not None, "--commit must be the checkout's own"
    assert refusal("", a, None) is not None, "an unreadable checkout is refused"
    assert refusal(a, None, None) is not None, "tables naming no commit want --commit"
    assert pinned_commit(TABLES.read_text(encoding="utf-8")) is not None, "the committed tables name their commit"
    print("cvss4_tables.py: self-test passed")


def every_base():
    for values in itertools.product(*(v for _, v in BASE)):
        yield "CVSS:4.0/" + "/".join(f"{k}:{x}" for (k, _), x in zip(BASE, values))


def main():
    if "--self-test" in sys.argv:
        self_test()
        return
    checkout = pathlib.Path(sys.argv[1])
    commit = subprocess.run(["git", "-C", str(checkout), "rev-parse", "HEAD"], capture_output=True,
                            text=True, encoding="utf-8").stdout.strip()
    asked = sys.argv[sys.argv.index("--commit") + 1] if "--commit" in sys.argv else None
    why = refusal(commit, pinned_commit(TABLES.read_text(encoding="utf-8")), asked)
    if why:
        sys.exit(f"cvss4_tables.py: refused: {why}")
    if "--all" in sys.argv:
        out = pathlib.Path(sys.argv[sys.argv.index("--all") + 1])
        vectors = [v + t for v in every_base() for t in THREAT]
        write_text(out, node(SCORER, checkout, "\n".join(vectors)))
        print(f"{len(vectors)} vectors scored to {out}")
        return

    data = json.loads(node(TABLE_DUMP, checkout))
    license_text = (checkout / "LICENSE").read_text(encoding="utf-8").strip()
    lines = [
        f"//! CVSS v4 tables, copied by tools/cvss4_tables.py from FIRST's reference calculator,",
        f"//! github.com/FIRSTdotorg/cvss-v4-calculator at commit {commit} (ADR-033). Do not edit by hand.",
        "//!",
    ] + [("//! " + l).rstrip() for l in license_text.splitlines()] + [
        "",
        "/// The score of each of the 270 macrovectors, by its six digits.",
        "pub(crate) const LOOKUP: &[(&str, f64)] = &[",
    ]
    for key in sorted(data["lookup"]):
        lines.append(f'    ("{key}", {float(data["lookup"][key])!r}),')
    lines.append("];")
    lines.append("")
    lines.append("/// The most severe vectors of each equivalence class's level, as parts of a vector.")
    for eq in ["eq1", "eq2", "eq4", "eq5"]:
        lines.append(f"pub(crate) const MAX_{eq.upper()}: &[&[&str]] = &[")
        for level in sorted(data["composed"][eq], key=int):
            parts = ", ".join(json.dumps(p) for p in data["composed"][eq][level])
            lines.append(f"    &[{parts}],")
        lines.append("];")
    lines.append("/// EQ3 with EQ6: by the EQ3 level, then the EQ6 level; empty where the pair does not exist.")
    lines.append("pub(crate) const MAX_EQ3_EQ6: &[[&[&str]; 2]; 3] = &[")
    for level in sorted(data["composed"]["eq3"], key=int):
        row = []
        for six in ["0", "1"]:
            parts = ", ".join(json.dumps(p) for p in data["composed"]["eq3"][level].get(six, []))
            row.append(f"&[{parts}]")
        lines.append(f"    [{', '.join(row)}],")
    lines.append("];")
    lines.append("")
    lines.append("/// The depth of each level, in steps of 0.1: how far its least severe vector is from its most severe.")
    for eq in ["eq1", "eq2", "eq4"]:
        values = ", ".join(str(data["severity"][eq][k]) for k in sorted(data["severity"][eq], key=int))
        lines.append(f"pub(crate) const DEPTH_{eq.upper()}: &[u32] = &[{values}];")
    rows = []
    for level in sorted(data["severity"]["eq3eq6"], key=int):
        pair = data["severity"]["eq3eq6"][level]
        rows.append("[" + ", ".join(str(pair.get(six, 0)) for six in ["0", "1"]) + "]")
    lines.append(f"pub(crate) const DEPTH_EQ3_EQ6: &[[u32; 2]; 3] = &[{', '.join(rows)}];")
    write_text(TABLES, "\n".join(lines) + "\n")

    # The sample: every base value at least once, the corners, and a fixed random draw, each with every threat
    # value, so the test holds the scoring to the reference without carrying all 419,904 pairs.
    rng = random.Random(20261005)
    every = list(every_base())
    sample = set(every[:3] + every[-3:])
    sample.update(rng.sample(every, 400))
    vectors = sorted(v + t for v in sample for t in THREAT)
    FIXTURE.parent.mkdir(parents=True, exist_ok=True)
    header = (f"# CVSS v4 vectors and their scores from FIRST's reference calculator, commit {commit}, "
              f"written by tools/cvss4_tables.py.\n")
    write_text(FIXTURE, header + node(SCORER, checkout, "\n".join(vectors)))
    print(f"{TABLES.relative_to(ROOT)} and {len(vectors)} reference scores written")


if __name__ == "__main__":
    main()
