#!/usr/bin/env python3
"""Keeps the NIST SP 800-53 control text in the ASVS crosswalk true to NIST's own OSCAL catalog.

    python3 tools/nist_controls.py                    # re-read the pinned release, rewrite the control text
    python3 tools/nist_controls.py --release v1.7.0   # move to a newer oscal-content release, if nothing broke
    python3 tools/nist_controls.py --catalog <file>   # read a catalog already downloaded, instead of fetching it

`data/asvs-800-53-crosswalk.json` pairs each ASVS 5.0 requirement with the 800-53 Rev. 5 controls and enhancements
that ask the same thing (ADR-156). The pairs are chosen by reading both texts; the controls' labels, titles, and
statements are never written by hand: this script reads them from the catalog NIST publishes in OSCAL and writes them
into `controls`, for every control the crosswalk cites and no other. Moving to a newer release names every cited
control whose title or statement changed, and refuses when one is withdrawn or gone, naming where NIST moved it, so a
pair is never kept to something 800-53 no longer asks.

This is maintenance of the repository's own data, run by whoever runs it, outside the sandbox: `sv` itself fetches
nothing, and reads the file as it was committed. Only NIST's public catalog file is fetched.

NIST SP 800-53 Rev. 5 is a work of the United States government, from https://github.com/usnistgov/oscal-content.
"""

import argparse
import json
import re
import sys
import urllib.request
from pathlib import Path
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _shared import write_text  # noqa: E402  (tools/_shared.py)


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
CROSSWALK = ROOT / "data" / "asvs-800-53-crosswalk.json"
RELEASE_URL = (
    "https://raw.githubusercontent.com/usnistgov/oscal-content/{}/nist.gov/SP800-53/rev5/json/"
    "NIST_SP-800-53_rev5_catalog.json"
)


def fetch(release: str) -> dict:
    assert re.fullmatch(r"v\d+\.\d+\.\d+", release), release
    request = urllib.request.Request(
        RELEASE_URL.format(release),
        headers={"User-Agent": "StackVet-sv-repository-maintenance"},
    )
    with urllib.request.urlopen(request, timeout=120) as response:
        return json.loads(response.read().decode("utf-8"))


def prose(parts: list) -> list:
    out = []
    for part in parts or []:
        if part.get("prose"):
            out.append(part["prose"])
        out.extend(prose(part.get("parts")))
    return out


def params(control: dict, inherited: dict) -> dict:
    """Each parameter as the catalog shows it to a reader: `[Assignment: ...]` or `[Selection: ...]`."""
    seen = dict(inherited)
    for p in control.get("params", []):
        if "label" in p:
            seen[p["id"]] = f"[Assignment: {p['label']}]"
        elif "select" in p:
            seen[p["id"]] = "[Selection: " + "; ".join(p["select"].get("choice", [])) + "]"
        else:
            seen[p["id"]] = "[Assignment]"
    return seen


def fill(text: str, seen: dict) -> str:
    # A selection can hold an assignment, so a few passes.
    for _ in range(3):
        text = re.sub(
            r"\{\{\s*insert:\s*param,\s*([\w.\-]+)\s*\}\}", lambda m: seen.get(m.group(1), m.group(1)), text
        )
    return re.sub(r"\s+", " ", text).strip()


def read_catalog(catalog: dict) -> tuple:
    """Every control and enhancement by OSCAL id: the live ones' text, and where the withdrawn ones went."""
    live, withdrawn = {}, {}

    def walk(control, inherited):
        props = control.get("props", [])
        seen = params(control, inherited)
        if any(p["name"] == "status" and p.get("value") == "withdrawn" for p in props):
            withdrawn[control["id"]] = [
                link["href"].lstrip("#")
                for link in control.get("links", [])
                if link.get("rel") in ("incorporated-into", "moved-to")
            ]
        else:
            label = next(p["value"] for p in props if p["name"] == "label" and p.get("class") != "zero-padded")
            statement = [part for part in control.get("parts", []) if part["name"] == "statement"]
            live[control["id"]] = {
                "label": label,
                "title": control["title"],
                "statement": fill(" ".join(prose(statement)), seen),
            }
        for enhancement in control.get("controls", []):
            walk(enhancement, seen)

    for group in catalog["catalog"]["groups"]:
        for control in group["controls"]:
            walk(control, {})
    if len(live) < 900:
        sys.exit(f"read only {len(live)} controls: the catalog's layout is not the one expected")
    return live, withdrawn


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--release", help="an oscal-content release, such as v1.6.0")
    parser.add_argument("--catalog", type=Path, help="a catalog file already downloaded from that release")
    args = parser.parse_args()
    crosswalk = json.loads(CROSSWALK.read_text(encoding="utf-8"))
    release = args.release or crosswalk["catalog"]["release"]
    catalog = json.loads(args.catalog.read_text(encoding="utf-8")) if args.catalog else fetch(release)
    version = catalog["catalog"]["metadata"]["version"]
    live, withdrawn = read_catalog(catalog)

    cited = sorted({c for pairs in crosswalk["requirements"].values() for c in pairs}, key=sort_key)
    gone = [c for c in cited if c not in live]
    if gone:
        lines = [
            f"  {c}: "
            + (f"withdrawn, now in {', '.join(withdrawn[c]) or 'nothing named'}" if c in withdrawn else "not in it")
            for c in gone
        ]
        sys.exit(
            f"The 800-53 catalog in {release} no longer lists these as live controls. Nothing was written: read "
            "where each went and choose what those requirements should pair with now.\n" + "\n".join(lines)
        )
    for c in cited:
        was = crosswalk.get("controls", {}).get(c)
        if was and was["title"] != live[c]["title"]:
            print(f"{live[c]['label']}: renamed from {was['title']!r} to {live[c]['title']!r}")
        elif was and was["statement"] != live[c]["statement"]:
            print(f"{live[c]['label']}: its statement changed; read it again against the requirements it pairs with")
    crosswalk["catalog"] = {
        "name": "NIST SP 800-53 Rev. 5",
        "version": version,
        "release": release,
        "source": RELEASE_URL.format(release),
    }
    crosswalk["controls"] = {c: live[c] for c in cited}
    write_text(CROSSWALK, json.dumps(crosswalk, indent=2, ensure_ascii=False) + "\n")
    print(f"{len(cited)} controls read from 800-53 {version} ({release}); wrote {CROSSWALK.relative_to(ROOT)}")


def sort_key(control: str) -> tuple:
    """`ac-2` before `ac-2.1` before `ac-10`, as NIST orders them."""
    family, _, rest = control.partition("-")
    return (family, *(int(n) for n in rest.split(".")))


if __name__ == "__main__":
    main()
