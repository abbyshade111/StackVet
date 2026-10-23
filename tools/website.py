#!/usr/bin/env python3
"""Builds stackvet.dev from the repository: the home page, How StackVet works, the reports page, StackVet checks
itself, and the credits, around the framework pages (backlog 0376, ADR-132).

    python3 tools/website.py                 # writes exports/site/ (git ignores exports/)
    python3 tools/website.py --out DIR       # writes DIR instead
    python3 tools/website.py --self-test     # checks the small pieces against samples, and stops

Building it publishes nothing: where the site is hosted, and when it goes live, are the owner's to decide.

Every word is read from where it is kept, never copied here, so the site cannot say more than the repository does:

- the home page and the credits from `docs/website/home-page.md`, the reports page from
  `docs/website/reports-page.md`. In the home page, a section's last list, written after its words, is the notes on
  them (what each line rests on), and is left out; a section that holds only a line in italics is a placeholder, and
  is left out too. Buttons are the "Button:" lines, each linked from BUTTONS below; one not made yet is shown as
  coming, and a button this script does not know stops the build;
- the coverage numbers in the home page's "What it checks" from `docs/COVERAGE.md`'s level tables, and the AI tools
  the setup prompts were tried in from `docs/prompts/setup.md`'s "Tried so far". The home page holds the numbers of the
  day it was written; the site puts the file's in their place, and stops if the words they sit in have changed;
- How StackVet works from `docs/website/how-it-works-page.md`, then `docs/paper/HOW-SV-WORKS.md`'s description;
- How a build goes, the click-through of one real build, from `docs/website/build-walkthrough-page.md` and its
  screenshots in `docs/website/walkthrough/`: one step to a section, each with a link to the next, so it needs no
  script;
- StackVet checks itself from `docs/website/self-assessment/`, StackVet's own report on its own repository;
- the framework pages from `tools/framework_pages.py`, into `standards/`.

The report files the pages link to are copied beside them, and each Markdown one is also shown as a page, by
`tools/docs_page.py`'s Markdown reader, since a browser shows Markdown as plain text. The pages are self-contained: no
script, style sheet, font, or image is fetched from anywhere, and the type falls back to the system's when Atkinson
Hyperlegible is not installed. It writes only into a folder it marks as its own, inside the repository only under
`exports/`, and never through a link.

It also writes `_headers`, the response headers Cloudflare sends with every page (the owner chose Cloudflare on 22
October 2026, ADR-132): a content security policy that lets a page run only the scripts this build wrote into it, by
their hashes, and load nothing from anywhere else, with the headers that stop other sites framing a page, guessing a
file's type, or being told where a reader came from.
"""

import argparse
import base64
import hashlib
import html
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
import framework_pages  # noqa: E402  (tools/framework_pages.py)
from _shared import write_text  # noqa: E402  (tools/_shared.py)
from docs_page import render  # noqa: E402  (tools/docs_page.py, its Markdown reader)

MARK = ".stackvet-site"
DEFAULT_OUT = ROOT / "exports" / "site"
GITHUB = "https://github.com/abbyshade111/StackVet"
WEBSITE = ROOT / "docs" / "website"
SELF = WEBSITE / "self-assessment"

# The pages, in the order the top of every page lists them: (file, the name in the list).
PAGES = {
    "home": ("index.html", "Home"),
    "how": ("how-it-works.html", "How it works"),
    "standards": ("standards/index.html", "What it checks"),
    "reports": ("reports.html", "The report"),
    "itself": ("checks-itself.html", "StackVet checks itself"),
    "credits": ("credits.html", "Credits"),
}

# Where each button the home page names goes. None is a button for something not made yet: it is shown as coming.
BUTTONS = {
    "Get started": "#get-started",
    "See how a build goes": "how-a-build-goes.html",  # backlog 0373
    "Open the setup prompts": f"{GITHUB}/blob/main/docs/prompts/setup.md",
    "Read the guide": f"{GITHUB}/blob/main/docs/GETTING-STARTED.md",
}

# The home page's sections that are not shown on it: the look is how it is drawn, the credits are a page of their
# own, and the list of what is not yet true is for the owner.
NOT_ON_THE_HOME_PAGE = ("The look", "Credits", "Not yet true")

# The report's words for what is known of a requirement, and the color each is shown in (docs/brand/README.md).
STATUS_CLASS = {"needs attention": "found", "checked": "settle"}

NUMBER_WORDS = ["no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]

CSS = framework_pages.CSS + """
.hero { display: grid; grid-template-columns: 1.1fr 1fr; gap: 32px; align-items: start; margin: 24px 0 8px; }
.hero h1 { font-size: 40px; margin-top: 16px; }
.buttons { display: flex; flex-wrap: wrap; gap: 12px; margin: 20px 0; }
.button { display: inline-flex; align-items: center; min-height: 44px; padding: 8px 20px; border-radius: 22px;
  background: var(--accent); color: #FFFFFF; text-decoration: none; font-weight: 700; }
.button.secondary { background: var(--panel); color: var(--accent-text); border: 1px solid var(--line); }
.button.coming { background: var(--panel); color: var(--caption); border: 1px dashed var(--line); font-weight: 400; }
.chat { background: var(--panel); border: 1px solid var(--line); border-radius: 14px; padding: 16px; }
.chat .label { margin: 0 0 10px; color: var(--caption); font-size: 15px; }
.turn { margin: 10px 0; padding: 10px 14px; border-radius: 12px; max-width: 92%; }
.turn .who { display: block; font-size: 13px; color: var(--caption); font-family: "JetBrains Mono", ui-monospace,
  Menlo, Consolas, monospace; }
.turn.you { margin-left: auto; background: var(--page); }
.turn.tool { background: var(--panel); border: 1px solid var(--line); }
.turn.asks { background: var(--bubble); color: var(--accent-text); }
section.home { margin: 8px 0; }
section.home h2 { border-top: 0; }
section.band { background: var(--band); color: var(--band-text); margin: 40px -16px 0; padding: 8px 16px 24px;
  border-radius: 14px; }
section.band a { color: var(--attention); }
section.band .button { color: var(--text); background: var(--attention); }
ul.statuses { list-style: none; padding: 0; margin: 16px 0; }
ul.statuses li { display: grid; grid-template-columns: 150px 1fr; gap: 12px; padding: 10px 14px; margin: 6px 0;
  border-radius: 10px; background: var(--panel); color: var(--text); }
section.band ul.statuses li { background: #182A23; color: var(--band-text); }
.badge.plainstatus { color: inherit; }
.table { overflow-x: auto; }
.table table, .md table { border-collapse: collapse; width: 100%; background: var(--panel); margin: 12px 0; }
.table th, .table td { text-align: left; padding: 8px 10px; border-bottom: 1px solid var(--line); vertical-align: top; }
figure { margin: 20px 0; }
section.step { scroll-margin-top: 16px; padding-bottom: 8px; border-bottom: 1px solid var(--line); }
section.step h2 { border-top: 0; }
figure img { max-width: 100%; height: auto; background: var(--panel); border-radius: 10px; }
figcaption { color: var(--caption); font-size: 15px; }
.md { overflow-wrap: anywhere; }
.md pre { background: var(--panel); border: 1px solid var(--line); border-radius: 8px; padding: 12px; overflow-x: auto; }
.note { background: var(--bubble); border-radius: 10px; padding: 12px 16px; color: var(--text); }
@media (max-width: 760px) { .hero { grid-template-columns: 1fr; } .hero h1 { font-size: 30px; }
  ul.statuses li { grid-template-columns: 1fr; gap: 4px; } }
"""


def e(text):
    return html.escape(str(text), quote=True)


def die(message):
    sys.exit(f"tools/website.py: {message}")


def md(text, here="index.md"):
    """Markdown to HTML, by docs_page.py's reader."""
    return render(text, here)[0]


# ---------------------------------------------------------------- reading what the site says
def sections(text):
    """(title, body) for each `## ` section, in order."""
    parts = re.split(r"^## (.+)$", text, flags=re.M)
    return [(parts[i].strip(), parts[i + 1]) for i in range(1, len(parts), 2)]


def blocks(text):
    return [b.strip("\n") for b in re.split(r"\n\s*\n", text) if b.strip()]


def without_notes(bl):
    """A home page section's blocks without its notes: the list after its words."""
    bl = list(bl)
    while bl and bl[-1].lstrip().startswith("- "):
        bl.pop()
    return bl


def is_placeholder(bl):
    return all(re.fullmatch(r"\*[^*].*[^*]\*", " ".join(b.split()), re.S) for b in bl)


def levels(coverage, heading):
    """A level table of docs/COVERAGE.md: {"L1": (requirements, can settle, can be credited), ...}."""
    found = re.search(rf"^## {re.escape(heading)}\n(.*?)(?=^## |\Z)", coverage, re.M | re.S)
    if not found:
        die(f"docs/COVERAGE.md has no section {heading!r}")
    rows = {m[1]: (int(m[2]), int(m[3]), int(m[4]))
            for m in re.finditer(r"^\| (L[123]) \| (\d+) \| (\d+) \| (\d+) \|", found[1], re.M)}
    if sorted(rows) != ["L1", "L2", "L3"]:
        die(f"docs/COVERAGE.md's {heading!r} does not have the rows L1, L2, and L3")
    return rows


def counts(coverage):
    """What the home page's table and the sentences under it say, from docs/COVERAGE.md."""
    asvs, aisvs = levels(coverage, "ASVS 5.0 by level"), levels(coverage, "AISVS 1.0 by level")
    codeql = re.search(r"Of those, (\d+) only CodeQL reaches", coverage)
    if not codeql:
        die("docs/COVERAGE.md no longer says how many requirements only CodeQL reaches")

    def rows(table, keys):
        return tuple(sum(table[k][i] for k in keys) for i in range(3))

    return {
        "ASVS 5.0, levels 1 and 2": rows(asvs, ["L1", "L2"]),
        "ASVS 5.0, level 3": rows(asvs, ["L3"]),
        "AISVS 1.0, levels 1 and 2": rows(aisvs, ["L1", "L2"]),
        "AISVS 1.0, level 3": rows(aisvs, ["L3"]),
        "asvs more": sum(r[1] - r[2] for r in asvs.values()),
        "aisvs more": sum(r[1] - r[2] for r in aisvs.values()),
        "codeql": int(codeql[1]),
    }


def word(n, capital=False):
    w = NUMBER_WORDS[n] if n < len(NUMBER_WORDS) else str(n)
    return w[0].upper() + w[1:] if capital else w


def once(pattern, repl, text, what):
    new, n = re.subn(pattern, repl, text, flags=re.M)
    if n != 1:
        die(f"docs/website/home-page.md: {what} is no longer written as this script expects ({n} found), so its "
            "number from docs/COVERAGE.md has nowhere to go. Bring the two back in line.")
    return new


def with_counts(text, c):
    """The home page's "What it checks", with docs/COVERAGE.md's numbers in place of those written there."""
    for label in ("ASVS 5.0, levels 1 and 2", "ASVS 5.0, level 3", "AISVS 1.0, levels 1 and 2", "AISVS 1.0, level 3"):
        req, _settle, credited = c[label]
        text = once(rf"^(\| {re.escape(label)} \|[^|]*\|) \d+ \| \d+ \|$", rf"\g<1> {req} | {credited} |", text,
                    f"the table's row {label!r}")
    text = once(r"a check for \d+ more of the ASVS requirements and \d+ more of the AISVS ones",
                f"a check for {c['asvs more']} more of the ASVS requirements and {c['aisvs more']} more of the AISVS "
                "ones", text, "the sentence on the requirements a check can only show missing")
    text = once(r"^\w+ of the ASVS ones counted above only CodeQL can check",
                f"{word(c['codeql'], True)} of the ASVS ones counted above only CodeQL can check", text,
                "the sentence on what only CodeQL can check")
    return once(r"those \w+ stay not\s+verified", f"those {word(c['codeql'])} stay not verified", text,
                "the sentence on what stays not verified without CodeQL")


def tried_in(setup):
    """The AI tools docs/prompts/setup.md says its prompts were tried in: "Cursor and VS Code with GitHub Copilot"."""
    m = re.search(r"\*\*Tried so far:\*\* in (.+?), on a ", " ".join(setup.split()))
    if not m:
        die("docs/prompts/setup.md no longer says where the prompts were tried (\"**Tried so far:** in ...\")")
    return re.sub(r"\band in\b", "and", m[1])


def with_tools_tried(text, tools):
    return re.sub(r"\*\[the tools they have really been tried in[^\]]*\]\*", tools, text)


# ---------------------------------------------------------------- the pieces of a page
def buttons(block):
    out = []
    # What is in brackets is a note on a button ("*not yet made*"), not a button.
    for label in re.findall(r"\*([^*]+)\*", re.sub(r"\([^)]*\)", "", block)):
        if label not in BUTTONS:
            die(f"the home page has a button this script does not know: {label!r}. Add it to BUTTONS.")
        href = BUTTONS[label]
        if href is None:
            out.append(f'<span class="button coming" aria-disabled="true">{e(label)}'
                       '<span class="small">&nbsp;(coming)</span></span>')
        else:
            out.append(f'<a class="button{" secondary" if out else ""}" href="{e(href)}">{e(label)}</a>')
    return f'<div class="buttons">{"".join(out)}</div>'


def conversation(block, label):
    turns = []
    for line in block.splitlines():
        line = re.sub(r"^\s*>\s?", "", line)
        m = re.match(r"\*\*(.+?):\*\*\s*(.*)$", line)
        if m:
            turns.append([m[1], m[2]])
        elif turns:
            turns[-1][1] += " " + line.strip()
    out = [f'<div class="chat" aria-label="{e(label)}"><p class="label">{e(label)}</p>']
    for who, said in turns:
        kind = "you" if who == "You" else "asks" if "asks" in who else "tool"
        out.append(f'<div class="turn {kind}"><span class="who">{e(who)}</span>{md(said)[3:-4]}</div>')
    out.append("</div>")
    return "".join(out)


def status_lines(block):
    """The report's example lines, a table with an empty header: each a status beside what it says."""
    items = []
    for row in block.splitlines()[2:]:
        status, said = [c.strip() for c in row.strip().strip("|").split("|", 1)]
        kind = STATUS_CLASS.get(status, "plainstatus")
        items.append(f'<li><span><span class="badge {kind}">{e(status)}</span></span><span>{md(said)[3:-4]}'
                     "</span></li>")
    return f'<ul class="statuses">{"".join(items)}</ul>'


def figure(block, prefix=""):
    m = re.fullmatch(r"!\[(.+?)\]\(([^)\s]+)\)", " ".join(block.split()), re.S)
    alt, src = m[1], m[2]
    return f'<figure><img src="{e(prefix + src)}" alt="{e(alt)}"></figure>'


def block_html(b, here="index.md", prefix=""):
    if re.match(r"Buttons?:", b):
        return buttons(b)
    if b.startswith("!["):
        return figure(b, prefix)
    if b.startswith("| |"):
        return status_lines(b)
    lines = b.splitlines()
    if len(lines) > 1 and re.fullmatch(r"\*\*[^*]+\*\*", lines[0].strip()):
        # A line in bold above its words is a small heading of its own.
        return f"<h3>{e(lines[0].strip()[2:-2])}</h3>" + md("\n".join(lines[1:]), here)
    return md(b, here)


def blocks_html(bl, here="index.md", prefix=""):
    """A run of blocks. A line in italics ending in a colon, above a conversation quoted line by line, is its
    label."""
    out, i = [], 0
    while i < len(bl):
        b = bl[i]
        if i + 1 < len(bl) and bl[i + 1].lstrip().startswith("> **") and b.rstrip().endswith(":*"):
            out.append(conversation(bl[i + 1], " ".join(b.split()).strip("*").rstrip(":")))
            i += 2
            continue
        out.append(block_html(b, here, prefix))
        i += 1
    return "".join(out)


def heading_of(bl):
    """A section's first block, its headline in bold, and the rest."""
    if bl and re.fullmatch(r"\*\*[^*]+\*\*", bl[0].strip()):
        return bl[0].strip()[2:-2], bl[1:]
    return None, bl


def section_id(title):
    return re.sub(r"[^a-z0-9]+", "-", re.sub(r"^\d+\.\s*", "", title).lower()).strip("-")


# ---------------------------------------------------------------- pages
def shell(title, here, body, commit, depth=0, credit=""):
    up = "../" * depth
    nav = "".join(f'<a href="{up}{f}"{" class=here" if k == here else ""}>{e(name)}</a>'
                  for k, (f, name) in PAGES.items() if k != "home")
    return (f"<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n"
            f"<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{e(title)}</title>\n"
            f"<link rel=\"icon\" href=\"{up}favicon.svg\" type=\"image/svg+xml\">\n<style>{CSS}</style>\n</head>\n<body>\n<header class=\"top\"><div class=\"wrap\">"
            f"<a class=\"wordmark\" href=\"{up}index.html\">StackVet</a><nav>{nav}"
            f"<a href=\"{GITHUB}\">GitHub</a></nav></div></header>\n"
            f"<main class=\"wrap\">\n{body}\n</main>\n<footer><div class=\"wrap\">"
            f"<p>StackVet is free and open source, under the MIT license, on <a href=\"{GITHUB}\">GitHub</a>. "
            f"<a href=\"{up}credits.html\">Credits</a>: the standards and tools it builds on. {credit}</p>"
            f"<p>Built from the repository at commit <span class=mono>{e(commit)}</span> by "
            "<span class=mono>tools/website.py</span>.</p></div></footer>\n</body>\n</html>\n")


def home_page(home_md, coverage, setup, commit):
    text = with_tools_tried(with_counts(home_md, counts(coverage)), tried_in(setup))
    out = []
    for title, body in sections(text):
        if title.startswith(NOT_ON_THE_HOME_PAGE):
            continue
        bl = without_notes(blocks(body))
        if not bl or is_placeholder(bl):
            continue
        headline, rest = heading_of(bl)
        ident = section_id(title)
        if ident == "the-opening":
            at = next((i for i, b in enumerate(rest) if b.lstrip().startswith(">")), None)
            if at is None or at == 0:
                die("the home page's opening has no conversation in it")
            words = blocks_html(rest[:at - 1])
            chat = blocks_html(rest[at - 1:at + 1])
            if 'class="chat"' not in chat:
                die("the home page's conversation has no label above it, in italics, ending in a colon")
            after = blocks_html(rest[at + 1:])
            out.append(f'<section class="hero" id="{ident}"><div><h1>{e(headline)}</h1>{words}</div>'
                       f"<div>{chat}</div></section>{after}")
            continue
        inner = blocks_html(rest)
        extra = {
            "the-report": '<div class="buttons"><a class="button" href="reports.html">What each file of a report is '
                          "for, with a real one</a></div>",
            "what-it-checks": '<figure><img src="coverage-by-level.svg" alt="ASVS 5.0 by level: what a check can '
                              'reach"></figure><div class="buttons"><a class="button" href="standards/index.html">'
                              "Every requirement, and what StackVet does about it</a></div>",
            "what-it-does-with-your-code": '<div class="buttons"><a class="button secondary" href="how-it-works.html">'
                                           "How StackVet works, in detail</a></div>",
        }.get(ident, "")
        band = " band" if ident == "the-report" else ""
        out.append(f'<section class="home{band}" id="{ident}"><h2>{e(headline or title)}</h2>{inner}{extra}</section>')
    return shell("StackVet: vibe-code your app with security in mind", "home", "\n".join(out), commit,
                 credit=not_endorsed(home_md))


def credits_section(home_md):
    for title, body in sections(home_md):
        if title.startswith("Credits"):
            return without_notes(blocks(body))
    die("docs/website/home-page.md has no Credits section")


def not_endorsed(home_md):
    return next((e(" ".join(b.split())) for b in credits_section(home_md) if b.startswith("StackVet is not made by")),
                "")


def credits_page(home_md, commit):
    headline, rest = heading_of(credits_section(home_md))
    body = f"<h1>{e(headline)}</h1>" + blocks_html(rest)
    return shell("Credits — StackVet", "credits", f'<div class="md">{body}</div>', commit)


def reports_page(reports_md, commit):
    out = []
    for title, body in sections(reports_md):
        if title.startswith("Not yet"):
            continue
        headline, rest = heading_of(blocks(body))
        level = "h1" if not out else "h2"
        out.append(f'<section id="{section_id(title)}"><{level}>{e(headline or title)}</{level}>'
                   + blocks_html(rest, "reports.md") + "</section>")
    return shell("The report — StackVet", "reports", f'<div class="md">{"".join(out)}</div>', commit)


def how_page(page_md, how_md, commit):
    found = re.search(r"^## Description\n(.*?)(?=^## |\Z)", how_md, re.M | re.S)
    if not found:
        die("docs/paper/HOW-SV-WORKS.md has no Description section")
    out = []
    for title, body in sections(page_md):
        headline, rest = heading_of(without_notes(blocks(body)))
        level = "h1" if not out else "h2"
        out.append(f'<section id="{section_id(title)}"><{level}>{e(headline or title)}</{level}>'
                   + blocks_html(rest, "how.md") + "</section>")
    out.append(f'<div class="md">{md(found[1], "how.md")}</div>')
    return shell("How StackVet works — StackVet", "how", "".join(out), commit)


def build_page(page_md, commit):
    """One step to a section: its heading, its caption, its screen, and a link on to the next."""
    steps = []
    for title, body in sections(page_md):
        headline, rest = heading_of(without_notes(blocks(body)))
        steps.append((section_id(title), headline or title, rest))
    if len(steps) < 2:
        die("docs/website/build-walkthrough-page.md has no steps")
    out = []
    for n, (ident, headline, rest) in enumerate(steps):
        level = "h1" if n == 0 else "h2"
        number = "" if n == 0 else f'<p class="small">Step {n} of {len(steps) - 1}</p>'
        if n + 1 < len(steps):
            on = f'<a class="button" href="#{steps[n + 1][0]}">{"Start" if n == 0 else "Next"}: {e(steps[n + 1][1])}</a>'
        else:
            on = '<a class="button secondary" href="index.html">Back to the home page</a>'
        out.append(f'<section class="step" id="{ident}">{number}<{level}>{e(headline)}</{level}>'
                   f'{blocks_html(rest, "build.md")}<div class="buttons">{on}</div></section>')
    return shell("How a build goes — StackVet", "", "".join(out), commit)


def self_files():
    names = sorted(p.name for p in SELF.iterdir() if p.is_file() and p.name != "README.md")
    if "compliance.md" not in names:
        die("docs/website/self-assessment/ has no compliance.md")
    return names


def itself_page(commit):
    compliance = (SELF / "compliance.md").read_text(encoding="utf-8")
    produced = re.search(r"^Produced by .*$", compliance, re.M)
    known = re.search(r"^Of the requirements that apply to this app:\n\n((?:- .*\n)+)", compliance, re.M)
    held = re.search(r"^Held to ASVS level \d\..*$", compliance, re.M)
    if not (produced and known and held):
        die("docs/website/self-assessment/compliance.md no longer opens as this script expects")
    readme = (SELF / "README.md").read_text(encoding="utf-8").split("\n", 1)[1]
    files = "".join(
        f'<li><a href="self-assessment/{e(n[:-3] + ".html" if n.endswith(".md") else n)}">{e(n)}</a></li>'
        for n in self_files())
    body = ("<h1>StackVet checks itself</h1>"
            '<p class="lede">StackVet\'s own report on its own code, held to the highest level there is, ASVS and '
            "AISVS level 3. It is shown as it came out, with nothing set aside.</p>"
            f'<div class="panel"><p>{md(produced[0])[3:-4]}</p><p>Of the requirements that apply:</p>'
            f'{md(known[1])}<p class="small">{md(held[0])[3:-4]}</p></div>'
            f'<h2>The report\'s files</h2><ul>{files}</ul>'
            f'<div class="md">{md(readme, "self.md")}</div>'
            f'<h2>The code it checked, and the earlier report</h2><p>The code is on <a href="{GITHUB}">GitHub</a>. '
            "The research paper's own report of StackVet, of 4 October 2026, is kept as it was, at "
            f'<a href="{GITHUB}/tree/main/docs/paper/self-assessment-v2">docs/paper/self-assessment-v2</a>.</p>')
    return shell("StackVet checks itself — StackVet", "itself", body, commit)


def markdown_page(src, title, commit, depth):
    """A report file written in Markdown, shown as a page, with a link to the file itself."""
    body = (f'<p class="small"><a href="{e(src.name)}">{e(src.name)}</a>, as a page.</p>'
            f'<div class="md">{md(src.read_text(encoding="utf-8"), src.name)}</div>')
    return shell(f"{src.name}: {title} — StackVet", "", body, commit, depth)


# ---------------------------------------------------------------- writing
def safe_out(out):
    out = Path(out).expanduser()
    if out.is_symlink():
        die(f"{out} is a link; nothing is written through it.")
    old = out / framework_pages.MARK
    if out.exists() and not (out / MARK).is_file() and not old.is_file() and any(out.iterdir()):
        die(f"{out} is already there and was not made by this tool, so it is left as it is. Choose --out.")
    resolved = out.resolve()
    if resolved.is_relative_to(ROOT) and not resolved.is_relative_to(ROOT / "exports"):
        die(f"{out} is inside the repository outside exports/; the site is never committed. Choose --out.")
    if old.is_file():
        # The framework pages alone, as tools/framework_pages.py wrote them here before the site had a home page:
        # its own files make way for the site's, which holds them in standards/.
        for f in [framework_pages.MARK, "index.html"] + [p[0] for p in framework_pages.PAGES.values()]:
            (out / f).unlink(missing_ok=True)
    return out


def inline_scripts(page):
    """The text of each script in a page, however its tags are written (`<SCRIPT>`, `</script >`), so none is left
    out of the policy."""
    return re.findall(r"<script\b[^>]*>(.*?)</script\b[^>]*>", page, re.S | re.I)


def headers(out):
    """`_headers` for Cloudflare: every inline script the build wrote is allowed by its hash, and nothing else runs.
    Inline styles are allowed, since the pages and the example report set them in `style` attributes too."""
    scripts = set()
    for page in sorted(out.rglob("*.html")):
        scripts.update(inline_scripts(page.read_text(encoding="utf-8")))
    hashes = " ".join(sorted("'sha256-" + base64.b64encode(hashlib.sha256(s.encode("utf-8")).digest()).decode()
                             + "'" for s in scripts)) or "'none'"
    policy = (f"default-src 'none'; script-src {hashes}; style-src 'unsafe-inline'; img-src 'self'; "
              "base-uri 'none'; form-action 'none'; frame-ancestors 'none'")
    return ("/*\n"
            f"  Content-Security-Policy: {policy}\n"
            "  X-Content-Type-Options: nosniff\n"
            "  Referrer-Policy: no-referrer\n"
            "  Permissions-Policy: camera=(), microphone=(), geolocation=()\n"
            "  Cross-Origin-Opener-Policy: same-origin\n"
            "  Strict-Transport-Security: max-age=63072000; includeSubDomains; preload\n")


def copy(src, dst):
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(src, dst)


def build(out):
    out = safe_out(out)
    commit = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--short", "HEAD"], capture_output=True,
                            text=True, encoding="utf-8").stdout.strip() or "unknown"
    read = lambda rel: (ROOT / rel).read_text(encoding="utf-8")  # noqa: E731
    home_md = read("docs/website/home-page.md")
    pages = {
        "index.html": home_page(home_md, read("docs/COVERAGE.md"), read("docs/prompts/setup.md"), commit),
        "credits.html": credits_page(home_md, commit),
        "reports.html": reports_page(read("docs/website/reports-page.md"), commit),
        "how-a-build-goes.html": build_page(read("docs/website/build-walkthrough-page.md"), commit),
        "how-it-works.html": how_page(read("docs/website/how-it-works-page.md"), read("docs/paper/HOW-SV-WORKS.md"),
                                      commit),
        "checks-itself.html": itself_page(commit),
    }
    out.mkdir(parents=True, exist_ok=True)
    write_text(out / MARK, "Made by tools/website.py. Run it again to bring the site up to date.\n")
    for name, text in pages.items():
        write_text(out / name, text)
    copy(ROOT / "docs/coverage-by-level.svg", out / "coverage-by-level.svg")
    for shot in sorted((WEBSITE / "walkthrough").glob("*.png")):
        copy(shot, out / "walkthrough" / shot.name)
    for svg in sorted((WEBSITE / "diagrams").glob("*.svg")):
        copy(svg, out / "diagrams" / svg.name)
    copy(ROOT / "docs/brand/favicon.svg", out / "favicon.svg")
    reports = WEBSITE / "reports"
    for src in sorted(p for p in reports.rglob("*") if p.is_file()):
        rel = src.relative_to(reports)
        copy(src, out / "reports" / rel)
        if src.suffix == ".md":
            write_text(out / "reports" / rel.with_suffix(".html"),
                       markdown_page(src, "an example report", commit, len(rel.parts)))
    for name in self_files():
        copy(SELF / name, out / "self-assessment" / name)
        if name.endswith(".md"):
            write_text(out / "self-assessment" / (name[:-3] + ".html"),
                       markdown_page(SELF / name, "StackVet's report on itself", commit, 1))
    framework_pages.build(out / "standards", home="../index.html")
    write_text(out / "_headers", headers(out))
    print(f"wrote {out}: " + ", ".join(pages) + ", the report files, and standards/")


def self_test():
    assert sections("intro\n## A\n\none\n## B\ntwo\n") == [("A", "\n\none\n"), ("B", "\ntwo\n")]
    assert without_notes(["**Head**", "Words.", "- a note\n  that runs on", "- another"]) == ["**Head**", "Words."]
    assert without_notes(["Words.", "- **Windows:** a list in the words", "Button: *Read the guide*."]) == [
        "Words.", "- **Windows:** a list in the words", "Button: *Read the guide*."], "a list before words stays"
    assert is_placeholder(["*Left blank until the paper is\nfinished.*"]) and not is_placeholder(["*Lead:* words"])
    assert section_id("7. Get started") == "get-started" and section_id("2. Who it's for") == "who-it-s-for"
    sample = ("| ASVS 5.0, levels 1 and 2 | Web | 253 | 112 |\n| ASVS 5.0, level 3 | The same | 92 | 10 |\n"
              "| AISVS 1.0, levels 1 and 2 | AI | 146 | 26 |\n| AISVS 1.0, level 3 | The same | 45 | 1 |\n\n"
              "a check for 40 more of the ASVS requirements and 18 more of the AISVS ones\n\n"
              "Five of the ASVS ones counted above only CodeQL can check, and without it, those five stay not\n"
              "verified.")
    c = {"ASVS 5.0, levels 1 and 2": (253, 171, 122), "ASVS 5.0, level 3": (92, 22, 10),
         "AISVS 1.0, levels 1 and 2": (146, 44, 26), "AISVS 1.0, level 3": (45, 2, 1),
         "asvs more": 61, "aisvs more": 19, "codeql": 4}
    got = with_counts(sample, c)
    assert "| 253 | 122 |" in got and "61 more of the ASVS" in got and "19 more of the AISVS" in got
    assert got.count("Four of the ASVS") == 1 and "those four stay not verified" in got
    try:
        with_counts(sample.replace("40 more", "forty more"), c)
        raise AssertionError("changed words must stop the build")
    except SystemExit:
        pass
    assert tried_in("**Tried so far:** in Cursor and in VS Code with GitHub Copilot, on a Mac, on 14") == \
        "Cursor and VS Code with GitHub Copilot"
    assert 'href="#get-started"' in buttons("Buttons: *Get started*, *See how a build goes* (not yet made).")
    BUTTONS["A page not made yet"] = None
    assert "coming" in buttons("Button: *A page not made yet*."), "a button for what is not made is shown as coming"
    del BUTTONS["A page not made yet"]
    chat = conversation("> **You:** I want\n> an app.\n> **Your AI tool asks:** Who sees it?", "An example")
    assert 'class="turn you"' in chat and "I want an app." in chat and 'class="turn asks"' in chat
    assert 'class="badge found"' in status_lines("| | |\n|---|---|\n| needs attention | Too quick. |")
    assert "<h3>New?</h3><p>Start here.</p>" == block_html("**New?**\nStart here.")
    assert inline_scripts("<script>a()</script><SCRIPT type=module>b()</SCRIPT ><p>") == ["a()", "b()"]
    print("website self-test: ok")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--out", default=str(DEFAULT_OUT), help="the folder to write (default: exports/site/)")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return
    build(args.out)


if __name__ == "__main__":
    os.chdir(ROOT)
    main()
