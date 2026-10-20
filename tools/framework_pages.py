#!/usr/bin/env python3
"""Writes stackvet.dev's framework pages: one for each standard StackVet checks against, with every requirement and
what StackVet does about it (backlog 0619, ADR-129).

    python3 tools/framework_pages.py                 # writes exports/site/ (git ignores exports/)
    python3 tools/framework_pages.py --out DIR       # writes DIR instead
    python3 tools/framework_pages.py --self-test     # checks the small pieces against samples, and stops

Three pages and an index: OWASP ASVS 5.0; AISVS 1.0 with its Appendix C; and the Secure by Design checklist 0.5.0.
Every requirement sits under its family and section, with an anchor of its own (`asvs.html#V6.2.1`), the standard's
words, StackVet's plain words, what StackVet does about it and what each check needs to run, what a person is asked and
how, and when it applies. A filter shows one level at a time; without JavaScript every level is shown.

Everything is read from where the checks keep it, so a page cannot say more than the code does:

- the standards from `data/frameworks/`, and the plain words from `data/plain-requirements.json`;
- what StackVet does from `tools/coverage.py`, the same classification that writes `docs/REQUIREMENTS.md` and
  `docs/COVERAGE.md`, called in its `--check` mode so nothing in the repository is written; a page's counts are
  therefore `docs/COVERAGE.md`'s, and `crates/sv-cli/tests/framework_pages.rs` holds them to it;
- what a person is asked from `data/design-questions.json`, `data/security-notes.json`, `data/human-checks.json`, and
  `data/design-decisions.json`;
- when a requirement applies from `data/knowledge/applicability.json` with `data/applicability-v2.json` over it,
  resolved as `sv_frameworks::applicability` resolves it: the overlay replaces the rules at each scope it names, and
  the most specific scope wins.

The pages are self-contained: no script, style sheet, font, or image is fetched, and the type falls back to the
system's when Atkinson Hyperlegible is not installed. The only links out are to the standards' own sources. It writes
only into a folder it marks as its own, inside the repository only under `exports/`, and never through a link. Where
the site is hosted, and whether these pages are published, is the owner's to decide (backlog 0376).
"""

import argparse
import html
import json
import os
import re
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
from _shared import write_text  # noqa: E402  (tools/_shared.py)

MARK = ".stackvet-framework-pages"
DEFAULT_OUT = ROOT / "exports" / "site"

# What StackVet does about a requirement, in the site's words (docs/website/home-page.md, "What it checks"), for the
# five kinds tools/coverage.py sorts every requirement into.
COVERAGE = {
    "settle": ("StackVet can confirm or fail it",
               "A check can mark it checked, or find it needing attention. A clean result is one check satisfied, "
               "not proof the whole requirement is met."),
    "found": ("StackVet can only show it missing",
              "A check can find it not met. Finding nothing does not show it is met, so a clean run credits nothing."),
    "support": ("A check helps; a person decides",
                "A check gives supporting evidence, but the requirement asks for a decision or a document only a "
                "person can give."),
    "owner": ("The owner answers",
              "No check speaks to it. StackVet asks the owner, and records the answer through sv review: as "
              "documented, attested, or checked by hand, never as checked."),
    "none": ("No check yet",
             "Nothing in StackVet speaks to it yet. A passing test of the app's own that names this requirement's ID "
             "is shown as tested by the app's own tests; otherwise the report says not verified."),
}
SBD_COVERAGE = {
    "support": ("A check helps; a person decides",
                "A person answers it. Checks on the ASVS requirements that ask the same thing are shown beside it as "
                "supporting evidence, never as checking it."),
    "owner": ("The owner answers",
              "A person answers it, and no check speaks to it."),
}

PAGES = {
    "asvs": ("asvs.html", "OWASP ASVS 5.0", "Application Security Verification Standard 5.0.0",
             "Web apps and the services behind them.",
             "https://github.com/OWASP/ASVS/releases/tag/v5.0.0_release"),
    "aisvs": ("aisvs.html", "OWASP AISVS 1.0", "Artificial Intelligence Security Verification Standard 1.0, with its "
              "Appendix C on AI-assisted secure coding", "Apps that use AI, and code an AI tool helped write.",
              "https://github.com/OWASP/AISVS/tree/main/1.0/en"),
    "sbd": ("secure-by-design.html", "Secure by Design checklist 0.5.0", "OWASP Secure by Design Framework 0.5.0 "
            "(draft, August 2025), its checklist", "Decisions made before the code.",
            "https://github.com/OWASP/www-project-secure-by-design-framework"),
}


def load(rel):
    return json.loads((ROOT / rel).read_text(encoding="utf-8"))


# ---------------------------------------------------------------- the facts, from coverage.py and data/
def classified():
    """tools/coverage.py's rows for ASVS and AISVS, those for Appendix C, and the functions it judged them with."""
    import coverage as cov

    captured = {}
    rows_of = cov.requirement_rows

    def capture(*args):
        rows = rows_of(*args)
        captured["args"], captured["rows"] = args, rows
        return rows

    def refuse(*_a, **_k):
        raise RuntimeError("tools/framework_pages.py never writes into the repository through coverage.py")

    cov.requirement_rows, cov.write_text = capture, refuse
    argv, sys.argv = sys.argv, ["coverage.py", "--check"]
    try:
        cov.main()
    except SystemExit as e:
        if e.code not in (None, 0):
            sys.exit(f"tools/coverage.py --check failed, so a page would not match the repository: {e.code}")
    finally:
        sys.argv, cov.requirement_rows = argv, rows_of
    asvs, aisvs, ev, tiers, settles, supports_only, manual_only, credited = captured["args"]
    appendix = {}
    for fam in load("data/frameworks/aisvs-1.0-appendix-c.json")["families"]:
        for r in fam["requirements"]:
            appendix[r["id"]] = {"level": str(r["level"]), "chapter": fam["id"], "chapter_name": fam["name"],
                                 "section": fam["id"], "section_name": fam["name"], "text": r["description"]}
    appendix_rows = rows_of(appendix, {}, ev, tiers, settles, supports_only, manual_only, credited)
    for r in appendix_rows:
        r["framework"] = "AISVS 1.0 Appendix C"
    return cov, captured["rows"], appendix_rows, asvs, ev, tiers


def scopes(q):
    parts = q.split(".")
    return [q, ".".join(parts[:-1]), parts[0]] if len(parts) == 3 else [q]


def applicability():
    """The rules as sv_frameworks resolves them: the overlay replaces every scope it names."""
    base = load("data/knowledge/applicability.json")
    overlay = load("data/applicability-v2.json")["rules"]
    named = {r["scope"] for r in overlay}
    return [r for r in base["rules"] if r["scope"] not in named] + overlay


def rules_for(q, rules):
    """The rules at the most specific scope covering `q`."""
    for s in scopes(q):
        found = [r for r in rules if r["scope"] == s]
        if found:
            return found
    return []


def applies(q, rules):
    """When `q` applies, in plain words, from the rules at its most specific scope."""
    found = rules_for(q, rules)
    if not found or all(r["condition"] == "always" for r in found):
        return "To every app held to its level."
    if any(r["condition"] == "never" for r in found):
        return "StackVet does not hold any app to it."
    out = []
    for r in found:
        line = f"When the app has {condition_words(r['condition'])}."
        if r.get("notApplicableReason"):
            line += f" Otherwise the report says: “{r['notApplicableReason']}”"
        out.append(line)
    return " ".join(out)


CONDITIONS = {"auth": "sign-in", "oauth": "sign-in through another provider", "ai": "AI features",
              "email": "email", "uploads": "file uploads", "tls": "its own TLS", "jwt": "tokens it signs (JWT)",
              "mcp": "an MCP connection", "mcp-server": "an MCP server", "rag": "retrieval-augmented generation",
              "training": "model training", "payments": "payments", "external-apis": "calls to outside services",
              "multi-tenant": "several customers' data side by side", "websockets": "WebSockets",
              "graphql": "GraphQL", "xml": "XML", "saml": "SAML sign-in", "ci-cd": "a build pipeline",
              "web-search": "web search by its AI", "webrtc": "WebRTC", "ai-actions": "AI that takes actions",
              "ai-history": "stored AI conversations", "ai-moderation": "AI moderation",
              "generates-media": "AI-made images or audio", "multi-agent": "several AI agents",
              "multimodal-ai": "AI that reads images or audio", "self-hosted-model": "a model it runs itself"}


def condition_words(c):
    return CONDITIONS.get(c, c.replace("-", " "))


def person(q, catalogs):
    """What a person is asked about `q`, and how they answer, or None."""
    design, notes, decisions, hand = catalogs
    if q in design:
        d = design[q]
        return ("Answered in stackvet.toml's [design] section: yes, no, not sure, or planned, and where in the code.",
                d.get("plain") or d["asks"])
    if q in notes:
        n = notes[q]
        return ("Written down in security-notes.md, which `sv notes` starts.", n.get("plain") or n["asks"])
    if q in decisions:
        d = decisions[q]
        return (f"Written down in design-decisions.md, under “{d.get('heading', d['title'])}”.", d["asks"])
    if q in hand:
        h = hand[q]
        return ("Checked by hand, and recorded with `sv review`.", f"{h['title']}. {h['how']}")
    return None


def checks_of(row, tier_needs):
    out = []
    for ch in row["checks"]:
        what = f"{ch['label']}: {ch['words']}" if ch["words"] else ""
        if ch["tool"]:
            n = ch["rules"]
            name = f"{ch['id']}, {n} rule{'' if n == 1 else 's'}"
        else:
            name = ch["id"]
        flags = [w for w, on in (("only shows it missing", ch["finding_only"]), ("only credits it", ch["credited_only"]),
                                 ("only ever in part", ch["in_part"])) if on]
        out.append({"kind": ch["kind"], "needs": tier_needs.get(ch["kind"], ""), "name": name, "what": what,
                    "flags": flags})
    return out


def entries():
    """Every requirement of each page, in the standard's order, with what StackVet does about it."""
    cov, rows, appendix_rows, asvs, ev, tiers = classified()
    tier_name = {t: n for t, n, _ in cov.TIERS}
    tier_needs = {n: needs for _, n, needs in cov.TIERS}
    plain = load("data/plain-requirements.json")["plain"]
    catalogs = ({q["id"]: q for q in load("data/design-questions.json")["questions"]},
                {s["id"]: s for s in load("data/security-notes.json")["sections"]},
                {s["id"]: s for s in load("data/design-decisions.json")["sections"]},
                {c["id"]: c for c in load("data/human-checks.json")["checks"]})
    rules = applicability()
    crosswalk = load("data/sbd-asvs-crosswalk.json")["controls"]
    to_sbd = defaultdict(list)
    for control, counterparts in crosswalk.items():
        for a in counterparts:
            to_sbd[a].append(control)

    pages = {"asvs": [], "aisvs": [], "sbd": []}
    for r in rows + appendix_rows:
        q = r["id"]
        pages["asvs" if r["framework"] == "ASVS 5.0" else "aisvs"].append({
            "id": q, "part": r["framework"], "family": r["family"], "family_name": r["family_name"],
            "section": r["section"], "section_name": r["section_name"], "level": int(r["level"]),
            "level_note": "", "text": r["text"], "plain": plain.get(q, ""), "status": r["status"],
            "checks": checks_of(r, tier_needs), "person": person(q, catalogs), "applies": applies(q, rules),
            "located": r.get("located", []),
            "related": [("secure-by-design.html", c) for c in to_sbd.get(q, [])],
        })
    level = {r["id"]: int(r["level"]) for r in rows}
    status = {r["id"]: r for r in rows}
    for domain in load("data/frameworks/sbd-checklist-0.5.0.json")["checklistDomains"]:
        for c in domain["controls"]:
            q = f"SBD-{c['id']}"
            derived = 1 if c.get("critical") or c.get("severityIfNo") == "high" else \
                2 if c.get("severityIfNo") == "medium" else 3
            lowest = min(((level[a], a) for a in crosswalk.get(q, {})), default=None)
            if lowest is None:
                lv, note = 1, "Shown at every level: nothing in ASVS asks the same thing, and the checklist has none."
            elif lowest[0] <= derived:
                lv, note = lowest[0], f"Level {lowest[0]}, as {lowest[1]}, the ASVS requirement that asks the same."
            else:
                lv, note = derived, f"Level {derived}, from how much its absence costs."
            direct = [{"kind": tier_name.get(t, t), "needs": tier_needs.get(tier_name.get(t, t), ""), "name": ch,
                       "what": "", "flags": []} for t in tiers(q) for ch in ev[q][t]]
            supported = bool(direct) or any(tiers(a) for a in crosswalk.get(q, {}))
            pages["sbd"].append({
                "id": q, "part": "Secure by Design checklist 0.5.0", "family": domain["id"],
                "family_name": domain["name"], "section": domain["id"], "section_name": domain["name"],
                "level": lv, "level_note": note + " The checklist has no levels of its own; this one is StackVet's.",
                "text": c["statement"], "plain": plain.get(q, ""), "status": "support" if supported else "owner",
                "checks": direct, "person": person(q, catalogs) or (
                    "Answered by a person reviewing the design. StackVet records no answer for it yet, so the report "
                    "says not verified.", None),
                "applies": "To every app: it is a question about how the app is designed.",
                "related": [("asvs.html", a) for a in crosswalk.get(q, {})],
                "critical": bool(c.get("critical")), "severity": c.get("severityIfNo", ""),
                "related_status": {a: status[a]["status"] for a in crosswalk.get(q, {}) if a in status},
            })
    # The standard's own order; AISVS's Appendix C after its chapters, as the standard prints it.
    order = lambda i: (i["part"].endswith("Appendix C"),
                       [int(x) if x.isdigit() else x for x in re.findall(r"\d+|[A-Za-z]+", i["id"])])
    for k in pages:
        if k != "sbd":
            pages[k].sort(key=order)
    return pages


# ---------------------------------------------------------------- the pages
CSS = """
:root { --page: #EEF0EA; --panel: #FFFFFF; --text: #14201C; --soft: #3B4842; --caption: #5A6660; --line: #E1E3DB;
  --accent: #C2410C; --accent-text: #9A3412; --bubble: #FBEFE7; --band: #14201C; --band-text: #F3F4EF;
  --attention: #FDBA74; --checked: #A7D7BF; }
* { box-sizing: border-box; }
body { margin: 0; background: var(--page); color: var(--text); line-height: 1.55; font-size: 17px;
  font-family: "Atkinson Hyperlegible", -apple-system, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; }
.mono, .wordmark, .badge, .id { font-family: "JetBrains Mono", ui-monospace, Menlo, Consolas, monospace; }
header.top { background: var(--panel); border-bottom: 1px solid var(--line); }
.wrap { max-width: 1040px; margin: 0 auto; padding: 0 16px; }
header.top .wrap { display: flex; flex-wrap: wrap; gap: 8px 24px; align-items: center; min-height: 64px; }
.wordmark { font-weight: 700; color: var(--text); text-decoration: none; font-size: 18px; }
nav a { color: var(--soft); text-decoration: none; display: inline-block; padding: 10px 6px; min-height: 44px; }
nav a.here { color: var(--accent-text); font-weight: 700; }
a { color: var(--accent-text); }
h1 { font-size: 32px; line-height: 1.2; margin: 32px 0 8px; }
h2 { font-size: 24px; margin: 40px 0 8px; padding-top: 8px; border-top: 1px solid var(--line); }
h3 { font-size: 18px; margin: 24px 0 8px; color: var(--soft); }
.lede { color: var(--soft); max-width: 720px; }
.panel { background: var(--panel); border: 1px solid var(--line); border-radius: 10px; padding: 16px 20px; }
.grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 16px; margin: 24px 0; }
table.counts { border-collapse: collapse; width: 100%; background: var(--panel); }
table.counts th, table.counts td { text-align: left; padding: 8px 10px; border-bottom: 1px solid var(--line); }
table.counts td.n { text-align: right; font-variant-numeric: tabular-nums; }
.scroll { overflow-x: auto; }
.filter { display: flex; flex-wrap: wrap; gap: 8px; margin: 16px 0; }
.filter button { min-height: 44px; padding: 8px 16px; border: 1px solid var(--line); border-radius: 22px;
  background: var(--panel); color: var(--text); font: inherit; cursor: pointer; }
.filter button[aria-pressed="true"] { background: var(--accent); border-color: var(--accent); color: #FFFFFF; }
article.req { background: var(--panel); border: 1px solid var(--line); border-radius: 10px; padding: 14px 18px;
  margin: 12px 0; scroll-margin-top: 16px; }
article.req:target { outline: 3px solid var(--accent); }
.req header { display: flex; flex-wrap: wrap; gap: 6px 10px; align-items: baseline; }
.id { font-weight: 700; color: var(--text); text-decoration: none; }
.badge { font-size: 13px; padding: 2px 8px; border-radius: 6px; border: 1px solid var(--line); color: var(--soft); }
.badge.settle { background: var(--checked); border-color: var(--checked); color: var(--text); }
.badge.found { background: var(--attention); border-color: var(--attention); color: var(--text); }
.badge.support { background: var(--bubble); color: var(--accent-text); }
.official { margin: 8px 0; padding-left: 12px; border-left: 3px solid var(--line); color: var(--soft); }
.plain { margin: 8px 0; font-weight: 700; }
dl { margin: 8px 0 0; display: grid; grid-template-columns: minmax(120px, 180px) 1fr; gap: 6px 14px; }
dt { color: var(--caption); font-size: 15px; }
dd { margin: 0; }
details summary { cursor: pointer; color: var(--accent-text); min-height: 32px; }
ul.checks { margin: 6px 0; padding-left: 20px; }
ul.checks li { margin: 4px 0; }
.small { font-size: 15px; color: var(--caption); }
.asked { background: var(--bubble); border-radius: 10px; padding: 8px 12px; }
footer { background: var(--band); color: var(--band-text); margin-top: 48px; padding: 24px 0 32px; font-size: 15px; }
footer a { color: var(--band-text); }
@media (max-width: 640px) { h1 { font-size: 26px; } dl { grid-template-columns: 1fr; } dt { margin-top: 6px; } }
"""

FILTER_JS = """
document.querySelectorAll('.filter button').forEach(function (b) {
  b.addEventListener('click', function () {
    var level = b.getAttribute('data-level');
    document.querySelectorAll('.filter button').forEach(function (o) { o.setAttribute('aria-pressed', o === b); });
    document.querySelectorAll('article.req').forEach(function (a) {
      a.hidden = level !== 'all' && a.getAttribute('data-level') !== level;
    });
    document.querySelectorAll('section.family').forEach(function (s) {
      s.hidden = !s.querySelector('article.req:not([hidden])');
    });
  });
});
"""


def e(text):
    return html.escape(str(text), quote=True)


def code_words(text):
    """Escapes `text` and sets what is between backticks as code."""
    return re.sub(r"`([^`]+)`", r"<code>\1</code>", e(text))


def page(title, here, body, commit):
    nav = "".join(f'<a href="{f}"{" class=here" if k == here else ""}>{e(short)}</a>'
                  for k, (f, short, *_rest) in PAGES.items())
    return (f"<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n"
            f"<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{e(title)}</title>\n"
            f"<style>{CSS}</style>\n</head>\n<body>\n<header class=\"top\"><div class=\"wrap\">"
            f"<a class=\"wordmark\" href=\"index.html\">StackVet</a><nav>{nav}</nav></div></header>\n"
            f"<main class=\"wrap\">\n{body}\n</main>\n<footer><div class=\"wrap\">{footer(here, commit)}</div></footer>\n"
            f"<script>{FILTER_JS}</script>\n</body>\n</html>\n")


def footer(here, commit):
    sources = [PAGES[k] for k in (PAGES if here == "index" else [here])]
    credit = " ".join(f"The requirements' own words are from the <a href=\"{e(url)}\">{e(long)}</a>, © OWASP, "
                      "shared under CC BY-SA 4.0, as these pages are." for _f, _s, long, _w, url in sources)
    return (f"<p>{credit}</p><p>Built from StackVet's own data at commit <span class=mono>{e(commit)}</span>, by "
            "<span class=mono>tools/framework_pages.py</span>: what each page says StackVet does is what its checks "
            "say in the code that day. A clean result is never proof that a requirement is met, and an answer a "
            "person gives counts only once <span class=mono>sv review</span> records it.</p>")


def counts_table(items, statuses, words):
    by = Counter((i["status"], i["level"]) for i in items)
    head = "".join(f"<th>Level {lv}</th>" for lv in (1, 2, 3))
    rows = []
    for s in statuses:
        cells = "".join(f'<td class="n" data-count="{s}-{lv}">{by[(s, lv)]}</td>' for lv in (1, 2, 3))
        total = sum(by[(s, lv)] for lv in (1, 2, 3))
        rows.append(f"<tr><th scope=row>{e(words[s][0])}</th>{cells}<td class=\"n\" data-count=\"{s}\">{total}</td></tr>")
    per = Counter(i["level"] for i in items)
    total_row = "".join(f'<td class="n">{per[lv]}</td>' for lv in (1, 2, 3))
    return (f'<div class="scroll"><table class="counts"><thead><tr><th>What StackVet does</th>{head}<th>All</th></tr>'
            f"</thead><tbody>{''.join(rows)}<tr><th scope=row>Every requirement</th>{total_row}"
            f'<td class="n" data-count="all">{len(items)}</td></tr></tbody></table></div>')


def entry(i, words, other_page):
    status_words, meaning = words[i["status"]]
    checks = ""
    if i["checks"]:
        lis = "".join(
            f"<li><strong>{e(c['kind'])}</strong>: <span class=mono>{e(c['name'])}</span>"
            + (f". {code_words(c['what'])}" if c["what"] else "")
            + (f" <em>({e(', '.join(c['flags']))})</em>" if c["flags"] else "")
            + (f"<br><span class=small>Needs: {code_words(c['needs'])}</span>" if c["needs"] else "") + "</li>"
            for c in i["checks"])
        n = len(i["checks"])
        checks = (f"<dt>Checks</dt><dd><details><summary>"
                  f"{'1 check speaks' if n == 1 else f'{n} checks speak'} to it</summary>"
                  f"<ul class=checks>{lis}</ul></details></dd>")
    asked = ""
    if i["person"]:
        how, question = i["person"]
        asked = f"<dt>A person</dt><dd>{code_words(how)}"
        if question:
            asked += f'<div class="asked">{code_words(question)}</div>'
        if i.get("located"):
            asked += (" StackVet shows where to look in the code: "
                      + ", ".join(f"<span class=mono>{e(l)}</span>" for l in i["located"]) + " lists the places.")
        asked += "</dd>"
    related = ""
    if i["related"]:
        links = ", ".join(
            f'<a href="{e(f)}#{e(q)}">{e(q)}</a>'
            + (f" ({e(COVERAGE[i['related_status'][q]][0].lower())})" if q in i.get("related_status", {}) else "")
            for f, q in i["related"])
        related = f"<dt>{e(other_page)}</dt><dd>{links}</dd>"
    level_note = f"<dt>Level</dt><dd>{e(i['level_note'])}</dd>" if i["level_note"] else ""
    severity = ""
    if "severity" in i:
        severity = (f"<dt>If the answer is no</dt><dd>{'A critical control. ' if i['critical'] else ''}"
                    f"Severity {e(i['severity'])}.</dd>")
    part = f'<span class="badge">{e(i["part"])}</span>' if i["part"].endswith("Appendix C") else ""
    return (f'<article class="req" id="{e(i["id"])}" data-level="{i["level"]}" data-status="{i["status"]}">'
            f'<header><a class="id" href="#{e(i["id"])}">{e(i["id"])}</a>'
            f'<span class="badge">Level {i["level"]}</span>'
            f'<span class="badge {i["status"]}">{e(status_words)}</span>{part}</header>'
            + (f'<p class="plain">{e(i["plain"])}</p>' if i["plain"] else "")
            + f'<p class="official">{e(i["text"])}</p>'
            f"<dl><dt>What StackVet does</dt><dd>{e(meaning)}</dd>{checks}{asked}"
            f"<dt>When it applies</dt><dd>{e(i['applies'])}</dd>{level_note}{severity}{related}</dl></article>")


def framework_page(key, items, commit):
    _file, short, long, what, url = PAGES[key]
    words = SBD_COVERAGE if key == "sbd" else COVERAGE
    statuses = ["support", "owner"] if key == "sbd" else ["settle", "found", "support", "owner", "none"]
    other = "ASVS requirements that ask the same" if key == "sbd" else "Secure by Design controls that ask the same"
    intro = {
        "asvs": "ASVS sorts its requirements into three levels. Level 1 is the baseline for any app, and level 2 is "
                "for most apps that hold people's information; StackVet holds each app to one of those two, from "
                "what the app does and keeps. Level 3, for apps where a break-in would do the most harm, is listed "
                "here, though StackVet does not yet hold any app to it.",
        "aisvs": "AISVS uses the same three levels as ASVS, for an app's AI features. Its Appendix C is about code an "
                 "AI tool helps write; StackVet hands your AI tool its coding rules while it builds.",
        "sbd": "The checklist's controls are decisions made before the code, so no check can settle them: a person "
               "answers each one. StackVet builds them into planning, and shows beside each control what its checks "
               "found on the ASVS requirements that ask the same thing.",
    }[key]
    legend = "".join(f"<li><strong>{e(w)}</strong>: {e(m)}</li>" for w, m in (words[s] for s in statuses))
    body = [f"<h1>{e(short)}: every requirement, and what StackVet does about it</h1>",
            f'<p class="lede">{e(long)}. {e(what)} {e(intro)}</p>',
            counts_table(items, statuses, words),
            f'<div class="panel" style="margin-top:16px"><ul style="margin:0;padding-left:20px">{legend}</ul></div>',
            '<div class="filter" role="group" aria-label="Show a level">'
            '<button type="button" data-level="all" aria-pressed="true">Every level</button>'
            + "".join(f'<button type="button" data-level="{lv}" aria-pressed="false">Level {lv}</button>'
                      for lv in (1, 2, 3)) + "</div>"]
    family, section = None, None
    for i in items:
        if i["family"] != family:
            if family is not None:
                body.append("</section>")
            family, section = i["family"], None
            body.append(f'<section class="family" id="family-{e(family)}"><h2>{e(family)} {e(i["family_name"])}</h2>')
        if i["section"] != section and i["section"] != i["family"]:
            section = i["section"]
            body.append(f"<h3>{e(section)} {e(i['section_name'])}</h3>")
        body.append(entry(i, words, other))
    body.append("</section>")
    return page(f"{short} — StackVet", key, "\n".join(body), commit)


def index_page(pages, commit):
    cards = []
    for key, (f, short, long, what, _url) in PAGES.items():
        items = pages[key]
        by = Counter(i["status"] for i in items)
        if key == "sbd":
            line = f"{len(items)} controls, each answered by a person; {by['support']} with supporting evidence."
        else:
            line = (f"{len(items)} requirements: StackVet can confirm or fail {by['settle']}, "
                    f"and can only show {by['found']} missing.")
        cards.append(f'<a class="panel" href="{f}" style="text-decoration:none;color:inherit;display:block">'
                     f"<h2 style=\"border:0;margin:0 0 6px;padding:0\">{e(short)}</h2><p>{e(what)}</p>"
                     f'<p class="small" data-summary="{key}">{e(line)}</p></a>')
    body = ("<h1>What StackVet checks, requirement by requirement</h1>"
            '<p class="lede">OWASP is a nonprofit that writes the security checklists many companies build to. '
            "StackVet uses three of them. Each page lists every requirement, says what StackVet does about it, "
            "and says plainly what it does not.</p>" f'<div class="grid">{"".join(cards)}</div>')
    return page("What StackVet checks — StackVet", "index", body, commit)


# ---------------------------------------------------------------- writing
def safe_out(out):
    out = Path(out).expanduser()
    if out.is_symlink():
        sys.exit(f"{out} is a link; nothing is written through it.")
    if out.exists() and not (out / MARK).is_file() and any(out.iterdir()):
        sys.exit(f"{out} is already there and was not made by this tool, so it is left as it is. Choose --out.")
    resolved = out.resolve()
    if resolved.is_relative_to(ROOT) and not resolved.is_relative_to(ROOT / "exports"):
        sys.exit(f"{out} is inside the repository outside exports/; the pages are never committed. Choose --out.")
    return out


def build(out):
    out = safe_out(out)
    pages = entries()
    commit = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--short", "HEAD"], capture_output=True,
                            text=True, encoding="utf-8").stdout.strip() or "unknown"
    out.mkdir(parents=True, exist_ok=True)
    write_text(out / MARK, "Made by tools/framework_pages.py. Run it again to bring these pages up to date.\n")
    write_text(out / "index.html", index_page(pages, commit))
    for key, items in pages.items():
        write_text(out / PAGES[key][0], framework_page(key, items, commit))
    print(f"wrote {out}: index.html and " + ", ".join(f"{PAGES[k][0]} ({len(v)})" for k, v in pages.items()))


def self_test():
    assert e('<a href="x">') == "&lt;a href=&quot;x&quot;&gt;"
    assert code_words("run `sv <x>` now") == "run <code>sv &lt;x&gt;</code> now"
    assert scopes("V1.2.3") == ["V1.2.3", "V1.2", "V1"] and scopes("SBD-AS-01") == ["SBD-AS-01"]
    rules = [{"scope": "V6", "condition": "auth", "notApplicableReason": "No sign-in."},
             {"scope": "V6.2.1", "condition": "always"}]
    assert rules_for("V6.2.1", rules) == [rules[1]], "the most specific scope wins"
    assert rules_for("V6.3.1", rules) == [rules[0]]
    assert applies("V6.2.1", rules) == "To every app held to its level."
    assert applies("V6.3.1", rules).startswith("When the app has sign-in.")
    assert applies("V2.1.1", rules) == "To every app held to its level.", "no rule: it applies"
    assert applies("V1.1.1", [{"scope": "V1", "condition": "never"}]).startswith("StackVet does not")
    # The overlay replaces a scope's rules; it does not add to them.
    assert all(r.get("source") for r in applicability() if r["scope"] == "C10.2")
    print("framework_pages self-test: ok")


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
