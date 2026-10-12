#!/usr/bin/env python3
"""The coordination board: one session hands out the work, and every hand-out is a line in git (backlog 0273; ADR-061,
Later, 12 October 2026). The role is docs/COORDINATION.md; this tool is the only thing that writes the board,
docs/coordination/board.md, and it reads the board back for anyone.

The board has three parts, each a list of plain lines so a person reads it as it is: the sessions (name, arm, what each
holds), the queue (backlog numbers in the order they go out, each ready or on hold with a reason), and the log (one
dated line per thing the orchestrator did, with its reason). A claim is still made with tools/backlog.py, and `assign`
makes it on the builder's behalf, so the backlog stays the record and the board only orders it.

    python3 tools/coordination.py open --by <session>                    # writes an empty board (refused when one exists)
    python3 tools/coordination.py session add <name> --arm full|none     # a building session and its arm (the two arms of 0273)
    python3 tools/coordination.py session remove <name>
    python3 tools/coordination.py queue add 0263 [--hold "why it waits"]  # a backlog item, in order; --hold keeps it from assign
    python3 tools/coordination.py queue release 0263                     # lifts the hold
    python3 tools/coordination.py queue drop 0263
    python3 tools/coordination.py assign 0263 --to <name> --why "..."    # claims the item for the session and logs why
    python3 tools/coordination.py handout 0263                           # prints the message to give the builder: item and footer
    python3 tools/coordination.py finish 0263 [--pr URL]                 # the builder is free again; the item's own done note is the record
    python3 tools/coordination.py board                                  # prints the board
    python3 tools/coordination.py check                                  # the board agrees with the backlog (a test runs it)
    python3 tools/coordination.py measure --since 2026-10-13 [--until 2026-10-14] [--arms]
    python3 tools/coordination.py setup-no-context ~/stackvet-trial-noctx  # the clone for the no-context arm, on this computer
    python3 tools/coordination.py cloud-setup-script                     # the same for a cloud environment: paste it in its dialog
    python3 tools/coordination.py --self-test

`measure` counts from git what backlog 0273 says to judge the trial by: pull requests merged, those that only record a
claim (every file under docs/backlog/), merges of main back into branches, and numbers taken twice in the tree. Pull
requests refused after passing are on GitHub only and are not counted here. With --arms it also reads the board's log
and, where the `gh` program is present, each finished pull request's commits, for the rules no check enforces.
"""

import argparse
import datetime
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import backlog  # noqa: E402  (tools/backlog.py: the items, their status lines, and the claim)

ROOT = Path(__file__).resolve().parent.parent
BOARD = "docs/coordination/board.md"
ARMS = ("full", "none")

HEADER = """# The coordination board

Written only by the orchestrator, with `python3 tools/coordination.py`; the role is `docs/COORDINATION.md` (backlog
0273; ADR-061, Later). A building session reads this file and takes the item named beside its name, and nothing else:
a claim is still the record (`tools/backlog.py`), and this board only says who takes what, in which order, and why.
"""

# The same five lines end every hand-out, to either arm (backlog 0273: a small amount of context by design).
FOOTER = """Rules that hold whatever else you know, from the owner:
- Delete nothing, force-push nothing, and rewrite no history.
- Spend none of the owner's money, and change no setting of the repository or of this computer.
- Open a pull request for the work and let its checks decide; never merge a red one.
- When something is the owner's to decide, stop and ask the owner.
- When you are done, run `python3 tools/backlog.py done <number>` and say what was built and what was not."""

# A session's line names the item it holds and, when the orchestrator has given it one ahead, the item it takes next
# once its pull request has merged (the owner's choice of 12 October 2026: after the first paste, sessions take their
# work from the board, so their next item has to be there before they finish).
SESSION = re.compile(r"^- (?P<name>[A-Za-z0-9_-]+) — arm: (?P<arm>full|none) — holds: (?P<holds>nothing|\d{4} \(since \S+\))"
                     r"(?: — next: (?P<next>\d{4}))?$")
QUEUE = re.compile(r"^\d+\. (?P<number>\d{4}) (?P<title>.*?) — (?P<state>ready|hold: .+)$")
LOG = re.compile(r"^- (?P<when>\d{4}-\d{2}-\d{2} \d{2}:\d{2}) — (?P<text>.+)$")


def write_text(path, text):
    """UTF-8 with Unix line endings on every system (crates/sv-cli/tests/tools_text.rs)."""
    Path(path).parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as out:
        out.write(text)


def now():
    return datetime.datetime.now().strftime("%Y-%m-%d %H:%M")


class Board:
    """The board as data: sessions, queue, log; read from and written to one Markdown file."""

    def __init__(self, path):
        self.path = Path(path)
        self.sessions = []  # {name, arm, holds (number or None), since, next (number or None)}
        self.queue = []  # {number, title, hold (reason or None)}
        self.log = []  # {when, text}

    @classmethod
    def read(cls, path):
        board = cls(path)
        if not board.path.exists():
            raise SystemExit(f"no board at {path}; `open --by <session>` writes one")
        with open(board.path, encoding="utf-8") as f:
            text = f.read()
        section = None
        for line in text.splitlines():
            if line.startswith("## "):
                section = line[3:].strip()
                continue
            if not line.strip():
                continue
            if section == "Sessions":
                m = SESSION.match(line)
                if m:
                    holds = None if m["holds"] == "nothing" else m["holds"][:4]
                    since = None if holds is None else m["holds"][12:-1]
                    board.sessions.append({"name": m["name"], "arm": m["arm"], "holds": holds, "since": since,
                                           "next": m["next"]})
                elif line.startswith("- "):
                    raise SystemExit(f"{path}: a session line in no form the board knows: {line}")
            elif section == "Queue":
                m = QUEUE.match(line)
                if m:
                    hold = None if m["state"] == "ready" else m["state"][6:]
                    board.queue.append({"number": m["number"], "title": m["title"], "hold": hold})
                elif re.match(r"^\d+\. ", line):
                    raise SystemExit(f"{path}: a queue line in no form the board knows: {line}")
            elif section == "Log":
                m = LOG.match(line)
                if m:
                    board.log.append({"when": m["when"], "text": m["text"]})
                elif line.startswith("- "):
                    raise SystemExit(f"{path}: a log line in no form the board knows: {line}")
        return board

    def text(self):
        out = [HEADER, "## Sessions", ""]
        if not self.sessions:
            out.append("(none yet: `session add <name> --arm full|none`)")
        for s in self.sessions:
            holds = "nothing" if s["holds"] is None else f"{s['holds']} (since {s['since']})"
            ahead = f" — next: {s['next']}" if s.get("next") else ""
            out.append(f"- {s['name']} — arm: {s['arm']} — holds: {holds}{ahead}")
        out += ["", "## Queue", ""]
        if not self.queue:
            out.append("(empty)")
        for n, q in enumerate(self.queue, 1):
            state = "ready" if q["hold"] is None else f"hold: {q['hold']}"
            out.append(f"{n}. {q['number']} {q['title']} — {state}")
        out += ["", "## Log", ""]
        for entry in self.log:
            out.append(f"- {entry['when']} — {entry['text']}")
        return "\n".join(out) + "\n"

    def write(self):
        write_text(self.path, self.text())

    def note(self, text, when=None):
        self.log.append({"when": when or now(), "text": text})

    def session(self, name):
        for s in self.sessions:
            if s["name"] == name:
                return s
        raise SystemExit(f"no session {name!r} on the board; `session add {name} --arm full|none` adds one")

    def queued(self, number):
        for q in self.queue:
            if q["number"] == number:
                return q
        return None

    def holder(self, number):
        for s in self.sessions:
            if s["holds"] == number:
                return s
        return None

    def taker(self, number):
        """The session that holds `number`, or has it next."""
        for s in self.sessions:
            if number in (s["holds"], s.get("next")):
                return s
        return None


def key_of(number):
    """A backlog number as the board writes it: four digits."""
    if not re.fullmatch(r"\d{1,4}", number):
        raise SystemExit(f"{number!r} is not a backlog number")
    return f"{int(number):04d}"


def title_of(folder, number):
    return backlog.find(folder, number).title


# ---------------------------------------------------------------------------------------------------------------
# The commands.

def open_board(board_path, session, when=None):
    if Path(board_path).exists():
        raise SystemExit(f"{board_path} exists; the board is opened once, and read from then on")
    board = Board(board_path)
    board.note(f"opened by {session}", when)
    board.write()
    print(f"opened {board_path}")


def cmd_session(board_path, action, name, arm=None, when=None):
    board = Board.read(board_path)
    if action == "add":
        if arm not in ARMS:
            raise SystemExit(f"--arm is one of {', '.join(ARMS)}: full keeps the standing context, none has only the repository")
        if any(s["name"] == name for s in board.sessions):
            raise SystemExit(f"{name} is on the board already")
        board.sessions.append({"name": name, "arm": arm, "holds": None, "since": None, "next": None})
        board.note(f"session {name} added, arm {arm}", when)
    elif action == "remove":
        s = board.session(name)
        if s["holds"] or s.get("next"):
            raise SystemExit(f"{name} holds {s['holds'] or s['next']}; `finish` it first")
        board.sessions.remove(s)
        board.note(f"session {name} removed", when)
    else:
        raise SystemExit("session add|remove")
    board.write()
    print(board.log[-1]["text"])


def cmd_queue(board_path, folder, action, number, hold=None, when=None):
    board = Board.read(board_path)
    number = key_of(number)
    if action == "add":
        if board.queued(number):
            raise SystemExit(f"{number} is in the queue already")
        if board.taker(number):
            raise SystemExit(f"{number} is held by {board.taker(number)['name']}")
        item = backlog.find(folder, number)
        if item.kind == "done":
            raise SystemExit(f"{number} is done; nothing to queue")
        board.queue.append({"number": number, "title": item.title, "hold": hold})
        board.note(f"queued {number}" + (f", on hold: {hold}" if hold else ""), when)
    elif action in ("release", "drop"):
        q = board.queued(number)
        if q is None:
            raise SystemExit(f"{number} is not in the queue")
        if action == "release":
            q["hold"] = None
            board.note(f"released {number}", when)
        else:
            board.queue.remove(q)
            board.note(f"dropped {number} from the queue", when)
    else:
        raise SystemExit("queue add|release|drop")
    board.write()
    print(board.log[-1]["text"])


def cmd_assign(board_path, folder, number, to, why, when=None, date=None):
    """Gives a queued item to a session: the claim first (the record), then the board. A free session holds it at
    once; a session already holding one gets it as its next, which `finish` moves into its hands; one ahead at most."""
    board = Board.read(board_path)
    number = key_of(number)
    if not why or len(why.strip()) < 10:
        raise SystemExit("--why says, in at least ten characters, why this item goes to this session now")
    q = board.queued(number)
    if q is None:
        raise SystemExit(f"{number} is not in the queue; `queue add {number}` first")
    if q["hold"]:
        raise SystemExit(f"{number} is on hold: {q['hold']}; `queue release {number}` first")
    s = board.session(to)
    if s["holds"] and s.get("next"):
        raise SystemExit(f"{to} holds {s['holds']} and has {s['next']} next; one ahead at most, `finish {s['holds']}` first")
    date = date or backlog.today()
    if not backlog.claim(folder, number, to, date):
        raise SystemExit(f"the claim of {number} for {to} was refused; the board is unchanged")
    board.queue.remove(q)
    if s["holds"]:
        s["next"] = number
        board.note(f"assigned {number} to {to} ({s['arm']}) as next: {why.strip()}", when)
    else:
        s["holds"] = number
        s["since"] = (when or now())[:10]
        board.note(f"assigned {number} to {to} ({s['arm']}): {why.strip()}", when)
    board.write()
    arms = {arm: sum(1 for e in board.log if f"({arm})" in e["text"] and e["text"].startswith("assigned")) for arm in ARMS}
    print(board.log[-1]["text"])
    print(f"hand-outs so far: full {arms['full']}, none {arms['none']}")


def handout_text(folder, number, board=None):
    item = backlog.find(folder, key_of(number))
    holder = board.holder(item.path.name[:4]) if board else None
    lines = [
        f"Your next item is backlog {item.path.name[:4]}: {item.title}",
        f"(docs/backlog/{item.path.name}; claimed for you, so build it, in a branch and a pull request of its own).",
        "",
        item.text.strip(),
        "",
        FOOTER,
    ]
    if holder:
        lines.insert(2, f"Session on the board: {holder['name']}.")
    return "\n".join(lines) + "\n"


def cmd_handout(board_path, folder, number):
    board = Board.read(board_path) if Path(board_path).exists() else None
    print(handout_text(folder, number, board), end="")


def cmd_finish(board_path, number, pr=None, when=None):
    board = Board.read(board_path)
    number = key_of(number)
    s = board.holder(number)
    if s is None:
        raise SystemExit(f"nobody holds {number}")
    following = s.get("next")
    s["holds"], s["since"], s["next"] = following, ((when or now())[:10] if following else None), None
    board.note(f"finished {number} by {s['name']} ({s['arm']})" + (f": {pr}" if pr else "")
               + (f"; now holds {following}" if following else ""), when)
    board.write()
    print(board.log[-1]["text"])


def cmd_board(board_path):
    board = Board.read(board_path)
    print(board.text(), end="")
    held = sum(1 for s in board.sessions if s["holds"])
    ready = sum(1 for q in board.queue if q["hold"] is None)
    print(f"\n{len(board.sessions)} sessions, {held} holding an item; {len(board.queue)} queued, {ready} ready; {len(board.log)} log lines")


def problems(board, folder):
    """Where the board and the backlog disagree."""
    out = []
    numbers = {i.path.name[:4]: i for i in backlog.items(folder)}
    seen = set()
    for s in board.sessions:
        if s["arm"] not in ARMS:
            out.append(f"{s['name']}: arm {s['arm']!r} is neither full nor none")
        if s.get("next") and not s["holds"]:
            out.append(f"{s['name']} has {s['next']} next and holds nothing")
        for slot, number in (("holds", s["holds"]), ("has next", s.get("next"))):
            if not number:
                continue
            if number in seen:
                out.append(f"{number} is held twice")
            seen.add(number)
            item = numbers.get(number)
            if item is None:
                out.append(f"{s['name']} {slot} {number}, which is no item")
            elif item.kind == "done":
                # The builder's own `done` is the last step its hand-out names, and it lands in the builder's pull
                # request before the orchestrator's `finish`: the normal hand-back, not a disagreement (found on the
                # trial's first finished item, 0280, whose pull request this check turned red).
                continue
            elif item.kind != "claimed" or s["name"] not in item.sessions:
                out.append(f"{s['name']} {slot} {number}, but the item says: {item.status_text}")
    for q in board.queue:
        if q["number"] in seen:
            out.append(f"{q['number']} is both queued and held")
        seen.add(q["number"])
        item = numbers.get(q["number"])
        if item is None:
            out.append(f"{q['number']} is queued and is no item")
        elif item.kind == "done":
            out.append(f"{q['number']} is queued and done")
    return out


def cmd_check(board_path, folder):
    board = Board.read(board_path)
    found = problems(board, folder)
    for p in found:
        print(p)
    print(f"board: {len(found)} problems" if found else "board: agrees with the backlog")
    return 0 if not found else 1


# ---------------------------------------------------------------------------------------------------------------
# The measures (backlog 0273, "How to judge it").

def git(repo, *args):
    return subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True, encoding="utf-8", check=True).stdout


def branch_main(repo):
    for name in ("origin/main", "main"):
        if subprocess.run(["git", "-C", str(repo), "rev-parse", "--verify", "-q", name], capture_output=True).returncode == 0:
            return name
    return "HEAD"


def measure(repo, since, until=None, branch=None):
    """The counts from git for the days from `since` to `until` (both dates, inclusive)."""
    branch = branch or branch_main(repo)
    span = ["--since", f"{since} 00:00", "--until", f"{until or since} 23:59:59"]
    merged = [l.split("\t", 1) for l in git(repo, "log", "--first-parent", "--merges", *span, "--format=%H\t%s", branch).splitlines()]
    claim_only = 0
    for sha, _subject in merged:
        files = git(repo, "diff", "--name-only", f"{sha}^1", sha).split()
        if files and all(f.startswith("docs/backlog/") for f in files):
            claim_only += 1
    back = [l for l in git(repo, "log", "--merges", *span, "--format=%s", branch).splitlines()
            if re.match(r"^Merge (branch 'main'|remote-tracking branch 'origin/main')", l)]
    tree = git(repo, "ls-tree", "-r", "--name-only", branch).splitlines()
    twice = 0
    for folder, pattern in (("docs/adr/", r"ADR-(\d{3})\.md$"), ("docs/backlog/", r"(\d{4})-"), ("docs/design/", r"(\d{4})-")):
        seen = {}
        for f in tree:
            if f.startswith(folder):
                m = re.search(pattern, f.rsplit("/", 1)[-1])
                if m:
                    seen[m.group(1)] = seen.get(m.group(1), 0) + 1
        twice += sum(1 for n in seen.values() if n > 1)
    return {"merged": len(merged), "claim_only": claim_only, "main_into_branch": len(back), "numbers_twice": twice}


def arms_measure(board, gh=shutil.which("gh")):
    """Per arm, from the board's log: items assigned and finished, and, with gh, the commits of each finished pull
    request that lack the Assisted-by or Co-Authored-By line (ADR-092), and the merges of main among them."""
    per = {arm: {"assigned": 0, "finished": 0, "prs": [], "commits": 0, "unnamed_tool": 0, "main_merges": 0} for arm in ARMS}
    for e in board.log:
        m = re.match(r"^assigned \d{4} to \S+ \((full|none)\)", e["text"])
        if m:
            per[m.group(1)]["assigned"] += 1
        m = re.match(r"^finished \d{4} by \S+ \((full|none)\)(?:: ([^\s;]+))?", e["text"])
        if m:
            per[m.group(1)]["finished"] += 1
            if m.group(2):
                per[m.group(1)]["prs"].append(m.group(2))
    if gh:
        for arm in ARMS:
            for url in per[arm]["prs"]:
                out = subprocess.run([gh, "pr", "view", url, "--json", "commits", "-q", ".commits[].messageBody"],
                                     capture_output=True, text=True, encoding="utf-8")
                if out.returncode != 0:
                    continue
                for body in out.stdout.split("\n\n"):
                    if not body.strip():
                        continue
                    per[arm]["commits"] += 1
                    if not re.search(r"^(Assisted-by|Co-Authored-By):", body, re.M | re.I):
                        per[arm]["unnamed_tool"] += 1
                    if re.match(r"^Merge (branch 'main'|remote-tracking branch 'origin/main')", body):
                        per[arm]["main_merges"] += 1
    return per, bool(gh)


def cmd_measure(repo, board_path, since, until, arms):
    counts = measure(repo, since, until)
    print(f"{since} to {until or since}, on {branch_main(repo)}:")
    print(f"  pull requests merged:            {counts['merged']}")
    print(f"  of them only recording a claim:  {counts['claim_only']}")
    print(f"  merges of main into a branch:    {counts['main_into_branch']}")
    print(f"  numbers taken twice in the tree: {counts['numbers_twice']}")
    print("  refused after passing:           not countable from git (GitHub only)")
    if arms:
        board = Board.read(board_path)
        per, with_gh = arms_measure(board)
        for arm in ARMS:
            p = per[arm]
            line = f"  arm {arm}: {p['assigned']} assigned, {p['finished']} finished"
            if with_gh:
                line += f"; {p['commits']} commits, {p['unnamed_tool']} without the tool's name, {p['main_merges']} merges of main"
            print(line)
        if not with_gh:
            print("  (gh is not on this computer, so the commits of each pull request were not read)")
        print("  by hand: American English, plain language, the session's context size and tokens per item, questions to the owner")


# ---------------------------------------------------------------------------------------------------------------
# The no-context arm's clone.

STUB = ("# StackVet\n\nTake your work from the coordination board, `docs/coordination/board.md`, and nothing else is said here.\n"
        "This file stands in for the project guide on purpose, for the coordination trial (backlog 0273); nothing was tampered with.\n")


def setup_no_context(source, target, origin_url=None):
    """A clone with a two-line CLAUDE.md that git is told to ignore, and so, started there, a session with an empty
    memory folder (the folder is keyed on the path) and none of the standing instructions."""
    target = Path(target).expanduser()
    if target.exists():
        raise SystemExit(f"{target} exists; choose a new folder")
    free = shutil.disk_usage(target.parent if target.parent.exists() else Path.home()).free
    if free < 2 * 1024**3:
        raise SystemExit(f"{free // 1024**2} MB free; the clone wants room (CLAUDE.md: look before a long run)")
    if "CLAUDE.md" not in subprocess.run(["git", "-C", str(source), "ls-files", "CLAUDE.md"], capture_output=True, text=True, encoding="utf-8").stdout:
        raise SystemExit(f"{source} has no CLAUDE.md in git; nothing to stand in for")
    subprocess.run(["git", "clone", "-q", str(source), str(target)], check=True)
    if origin_url:
        subprocess.run(["git", "-C", str(target), "remote", "set-url", "origin", origin_url], check=True)
    write_text(target / "CLAUDE.md", STUB)
    subprocess.run(["git", "-C", str(target), "update-index", "--skip-worktree", "CLAUDE.md"], check=True)
    return target


# The owner chose cloud sessions for both arms (12 October 2026), so the no-context arm is a cloud environment of its own
# whose setup script does what setup_no_context does here. The script starts in /home/user, not in the clone (the owner's
# first test, 12 October 2026), so it looks for the clone; whether the clone exists when it runs, and whether git's index
# survives the environment cache, is not in the documentation: a test session that prints CLAUDE.md comes first.
CLOUD_SCRIPT = """#!/bin/bash
# The no-context arm of the coordination trial (backlog 0273; docs/COORDINATION.md): CLAUDE.md is a two-line stub,
# and git is told to ignore the change, so no commit can carry it. Runs once, before the session starts.
set -e
# The script starts in /home/user, not in the clone (seen on 12 October 2026), so it looks for the clone itself.
found=
for d in "$PWD" /home/user/*/ /workspace/*/; do
  if [ -f "$d/CLAUDE.md" ] && [ -d "$d/.git" ]; then cd "$d"; found=1; break; fi
done
[ -n "$found" ] || { echo "no clone with a CLAUDE.md under $PWD, /home/user, or /workspace; $PWD holds: $(ls -A | xargs)"; exit 1; }
echo "clone at $PWD"
printf '%s' '{stub}' > CLAUDE.md
git update-index --skip-worktree CLAUDE.md
echo "no-context arm: CLAUDE.md is the stub, and git ignores the change"
"""


def cloud_setup_script():
    return CLOUD_SCRIPT.replace("{stub}", STUB.replace("\n", "\\n").replace("'", "'\\''")).replace("printf '%s'", "printf '%b'")


def cmd_setup_no_context(target):
    url = git(ROOT, "remote", "get-url", "origin").strip()
    target = setup_no_context(ROOT, target, url)
    print(f"cloned to {target}, origin {url}")
    print("CLAUDE.md there is a two-line stub, and git ignores that change (update-index --skip-worktree)")
    print(f"start the no-context session in {target}; its memory folder is new because the path is new")


# ---------------------------------------------------------------------------------------------------------------

def self_test():
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        folder = tmp / "docs" / "backlog"
        folder.mkdir(parents=True)
        for n, title in ((1, "First item"), (2, "Second item"), (3, "Third item"), (4, "Done item"), (5, "Fifth item")):
            backlog.write(folder, n, title, "done, 1 October 2026" if n == 4 else "open", "Its text.\n")
        board_path = tmp / BOARD
        when = "2026-10-13 09:00"

        open_board(board_path, "orchestrator", when)
        failed = False
        try:
            open_board(board_path, "orchestrator", when)
        except SystemExit:
            failed = True
        assert failed, "a second open is refused"
        cmd_session(board_path, "add", "a", "full", when)
        cmd_session(board_path, "add", "b", "none", when)
        for bad in (("add", "a", "full"), ("add", "c", "some")):
            try:
                cmd_session(board_path, *bad, when)
                assert False, bad
            except SystemExit:
                pass
        cmd_queue(board_path, folder, "add", "1", None, when)
        cmd_queue(board_path, folder, "add", "2", "the loop trials", when)
        cmd_queue(board_path, folder, "add", "3", None, when)
        cmd_queue(board_path, folder, "add", "5", None, when)
        try:
            cmd_queue(board_path, folder, "add", "4", None, when)
            assert False, "a done item is refused"
        except SystemExit:
            pass
        for args in (("2", "a", "held: it waits"), ("1", "a", "short"), ("9", "a", "not in the queue at all")):
            try:
                cmd_assign(board_path, folder, *args, when=when, date="13 October 2026")
                assert False, args
            except SystemExit:
                pass
        cmd_assign(board_path, folder, "1", "a", "the first ready item, pair of 0003", when=when, date="13 October 2026")
        assert backlog.find(folder, "1").status_text == "claimed by a, 13 October 2026", "assign claimed the item"
        # A busy session is given its next item ahead, claimed for it, and one ahead at most.
        cmd_assign(board_path, folder, "3", "a", "a's next, so it is there when a finishes", when=when, date="13 October 2026")
        assert backlog.find(folder, "3").status_text == "claimed by a, 13 October 2026", "a next item is claimed too"
        try:
            cmd_assign(board_path, folder, "5", "a", "a second one ahead for a busy session", when=when, date="13 October 2026")
            assert False, "one ahead at most"
        except SystemExit:
            pass
        assert backlog.find(folder, "5").kind == "open", "a refused assign claims nothing"
        cmd_queue(board_path, folder, "release", "2", None, when)
        cmd_assign(board_path, folder, "2", "b", "the pair of 0001, to the other arm", when=when, date="13 October 2026")
        board = Board.read(board_path)
        assert board.session("a")["holds"] == "0001" and board.session("a")["next"] == "0003"
        assert board.session("b")["holds"] == "0002" and board.session("b")["next"] is None
        assert "- a — arm: full — holds: 0001 (since 2026-10-13) — next: 0003" in board.text(), board.text()
        assert [q["number"] for q in board.queue] == ["0005"]
        assert problems(board, folder) == [], problems(board, folder)
        try:
            cmd_queue(board_path, folder, "add", "3", None, when)
            assert False, "an item a session has next is not queued again"
        except SystemExit:
            pass
        # The board is read back as it was written.
        again = Board(board_path)
        again.sessions, again.queue, again.log = board.sessions, board.queue, board.log
        assert again.text() == board.text()
        text = handout_text(folder, "1", board)
        assert "backlog 0001: First item" in text and "Its text." in text and FOOTER in text and "Session on the board: a." in text
        # The builder marks its item done before the orchestrator records the finish: the normal hand-back.
        backlog.mark_done(folder, "1", "13 October 2026", None)
        assert problems(Board.read(board_path), folder) == [], problems(Board.read(board_path), folder)
        # A board that disagrees for real: a session holds an item another session claimed.
        backlog.claim(folder, "5", "someone-else", "13 October 2026")
        stale = Board.read(board_path)
        stale.session("b")["holds"], stale.session("b")["since"] = "0005", "2026-10-13"
        stale.queue = [q for q in stale.queue if q["number"] != "0005"]
        found = problems(stale, folder)
        assert len(found) == 1 and "b holds 0005, but the item says: claimed by someone-else" in found[0], found
        # And a next item with nothing held before it.
        stale = Board.read(board_path)
        stale.session("a")["holds"] = None
        found = problems(stale, folder)
        assert any("a has 0003 next and holds nothing" in f for f in found), found
        # `finish` moves the next item into the session's hands.
        cmd_finish(board_path, "1", "https://example.test/pull/1", when)
        try:
            cmd_finish(board_path, "1", None, when)
            assert False, "nobody holds it now"
        except SystemExit:
            pass
        board = Board.read(board_path)
        assert board.session("a")["holds"] == "0003" and board.session("a")["next"] is None
        assert board.log[-1]["text"].endswith("; now holds 0003"), board.log[-1]
        assert problems(board, folder) == [], problems(board, folder)
        cmd_finish(board_path, "3", None, when)
        assert Board.read(board_path).session("a")["holds"] is None
        cmd_session(board_path, "remove", "a", None, when)
        try:
            cmd_session(board_path, "remove", "b", None, when)
            assert False, "b holds 0002"
        except SystemExit:
            pass
        per, _ = arms_measure(Board.read(board_path), gh=None)
        assert per["full"] == {"assigned": 2, "finished": 2, "prs": ["https://example.test/pull/1"], "commits": 0, "unnamed_tool": 0, "main_merges": 0}, per["full"]
        assert per["none"]["assigned"] == 1 and per["none"]["finished"] == 0

        # The measures, on a history made here: three pull requests, one of them a claim only, and one merge of main
        # into a branch; one backlog number taken twice.
        repo = tmp / "repo"
        repo.mkdir()
        env = {**os.environ, "GIT_AUTHOR_DATE": "2026-10-13T10:00:00", "GIT_COMMITTER_DATE": "2026-10-13T10:00:00",
               "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@t", "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@t"}

        def g(*args):
            return subprocess.run(["git", "-C", str(repo), *args], check=True, capture_output=True, text=True, encoding="utf-8", env=env).stdout

        def commit(path, text, message):
            (repo / path).parent.mkdir(parents=True, exist_ok=True)
            write_text(repo / path, text)
            g("add", "-A")
            g("commit", "-q", "-m", message)

        g("init", "-q", "-b", "main")
        write_text(repo / "CLAUDE.md", "# Rules\n\nMany.\n")
        commit("README.md", "a\n", "start")
        g("checkout", "-q", "-b", "claim")
        commit("docs/backlog/0001-one.md", "# One\n\n**Status:** claimed by x, 13 October 2026\n", "Claim 0001")
        g("checkout", "-q", "main")
        g("merge", "-q", "--no-ff", "claim", "-m", "Merge pull request #1 from x/claim")
        g("checkout", "-q", "-b", "build")
        commit("crates/a.rs", "fn a() {}\n", "Build a")
        g("checkout", "-q", "main")
        commit("docs/adr/ADR-015.md", "x\n", "A record")
        g("checkout", "-q", "build")
        g("merge", "-q", "--no-ff", "main", "-m", "Merge branch 'main' into build")
        g("checkout", "-q", "main")
        g("merge", "-q", "--no-ff", "build", "-m", "Merge pull request #2 from x/build")
        g("checkout", "-q", "-b", "twice")
        commit("docs/backlog/0001-one-again.md", "# One again\n\n**Status:** open\n", "Backlog 0001 again")
        g("checkout", "-q", "main")
        g("merge", "-q", "--no-ff", "twice", "-m", "Merge pull request #3 from x/twice")
        counts = measure(repo, "2026-10-13")
        assert counts == {"merged": 3, "claim_only": 2, "main_into_branch": 1, "numbers_twice": 1}, counts
        assert measure(repo, "2026-10-14") == {"merged": 0, "claim_only": 0, "main_into_branch": 0, "numbers_twice": 1}

        # The no-context clone: the stub is there, and git does not see it as a change.
        clone = setup_no_context(repo, tmp / "noctx")
        with open(clone / "CLAUDE.md", encoding="utf-8") as f:
            assert f.read() == STUB
        flags = subprocess.run(["git", "-C", str(clone), "ls-files", "-v", "CLAUDE.md"], capture_output=True, text=True, encoding="utf-8").stdout
        assert flags.startswith("S "), flags
        status = subprocess.run(["git", "-C", str(clone), "status", "--porcelain"], capture_output=True, text=True, encoding="utf-8").stdout
        assert status == "", status
        # The cloud script writes the same stub and the same git flag, and runs as a shell would run it.
        script = cloud_setup_script()
        assert "update-index --skip-worktree CLAUDE.md" in script and "set -e" in script
        shell = tmp / "shell"
        shell.mkdir()
        subprocess.run(["git", "clone", "-q", str(repo), str(shell)], check=True)
        run = subprocess.run(["bash", "-c", script], cwd=shell, capture_output=True, text=True, encoding="utf-8")
        assert run.returncode == 0, run.stderr
        with open(shell / "CLAUDE.md", encoding="utf-8") as f:
            assert f.read() == STUB, repr(f.read())
        flags = subprocess.run(["git", "-C", str(shell), "ls-files", "-v", "CLAUDE.md"], capture_output=True, text=True, encoding="utf-8").stdout
        assert flags.startswith("S "), flags
    print("coordination self-test: ok")


def main(argv):
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("command", nargs="?", default="board")
    parser.add_argument("args", nargs="*")
    parser.add_argument("--by", help="with open: the orchestrating session")
    parser.add_argument("--arm", help="with session add: full or none")
    parser.add_argument("--hold", help="with queue add: why the item waits")
    parser.add_argument("--to", help="with assign: the session")
    parser.add_argument("--why", help="with assign: why this item, this session, now")
    parser.add_argument("--pr", help="with finish: the pull request")
    parser.add_argument("--since", help="with measure: the first day, 2026-10-13")
    parser.add_argument("--until", help="with measure: the last day; the first alone otherwise")
    parser.add_argument("--arms", action="store_true", help="with measure: the two arms, from the board's log")
    parser.add_argument("--root", default=str(ROOT), help="the repository (tests)")
    parser.add_argument("--self-test", action="store_true")
    a = parser.parse_args(argv)
    if a.self_test:
        self_test()
        return 0
    root = Path(a.root)
    board_path = root / BOARD
    folder = root / "docs" / "backlog"
    c, args = a.command, a.args
    if c == "open":
        if not a.by:
            raise SystemExit("open --by <session>")
        open_board(board_path, a.by)
    elif c == "session":
        if len(args) != 2:
            raise SystemExit("session add|remove <name>")
        cmd_session(board_path, args[0], args[1], a.arm)
    elif c == "queue":
        if len(args) != 2:
            raise SystemExit("queue add|release|drop <number>")
        cmd_queue(board_path, folder, args[0], args[1], a.hold)
    elif c == "assign":
        if len(args) != 1 or not a.to:
            raise SystemExit("assign <number> --to <session> --why '...'")
        cmd_assign(board_path, folder, args[0], a.to, a.why or "")
    elif c == "handout":
        if len(args) != 1:
            raise SystemExit("handout <number>")
        cmd_handout(board_path, folder, args[0])
    elif c == "finish":
        if len(args) != 1:
            raise SystemExit("finish <number> [--pr URL]")
        cmd_finish(board_path, args[0], a.pr)
    elif c == "board":
        cmd_board(board_path)
    elif c == "check":
        return cmd_check(board_path, folder)
    elif c == "measure":
        if not a.since:
            raise SystemExit("measure --since 2026-10-13 [--until 2026-10-14] [--arms]")
        cmd_measure(root, board_path, a.since, a.until, a.arms)
    elif c == "cloud-setup-script":
        print(cloud_setup_script(), end="")
    elif c == "setup-no-context":
        if len(args) != 1:
            raise SystemExit("setup-no-context <new folder>")
        cmd_setup_no_context(args[0])
    else:
        raise SystemExit(f"no command {c!r}; see the top of this file")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
