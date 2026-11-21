#!/usr/bin/env python3
"""Compares each repository's rules on GitHub with `.github/rulesets.json`, and fails when they differ (backlog 0690).

`tools/adr_check.py` reads a pull request's diff, so a rule changed in a repository's settings page changes no file
and owes no record. `.github/rulesets.json` is that state written down: the rulesets of this repository, of the Scoop
bucket, and of the Homebrew tap, whether each allows auto-merge, and which merge methods it offers. ADR-051 and ADR-108 govern the file, so changing
a repository's rules means changing it in a pull request, where the "Decision records" check asks for the record's
line. This script keeps the file true: the weekly workflow `.github/workflows/rulesets.yml` runs it.

What can be read depends on who asks. Without an administrator's sign-in GitHub shows a ruleset's rules, its target,
its branches, and whether it is enforced, but not who may bypass it, and not a repository's own settings. Those
are then listed as not read, never as matching: nothing here passes what it did not see. The owner's own run,
`--gh`, reads them through the signed-in `gh` and so compares everything.

It opens connections to api.github.com only, read-only. It is a tool for this repository's upkeep, not part of `sv`,
whose fence (docs/ARCHITECTURE.md) is unchanged. A token, when one is given in GITHUB_TOKEN, is sent to GitHub and
never printed.

    python3 -I tools/rulesets_check.py          # as the weekly workflow runs it
    python3 -I tools/rulesets_check.py --gh     # the owner's run, reading everything through `gh`
    python3 -I tools/rulesets_check.py --self-test
"""

import json
import os
import subprocess
import sys
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXPECTED = ROOT / ".github" / "rulesets.json"
API = "https://api.github.com/"
# What a ruleset is compared on. `bypass_actors` is shown only to an administrator; the rest to anyone.
FIELDS = ["target", "enforcement", "conditions", "rules"]
ADMIN_ONLY = ["bypass_actors"]
# A repository's own settings compared, each shown to an administrator only. The merge methods are kept to merge
# commits (backlog 0709): `main`'s history is cited by commit hash, so it is never squashed or rebased.
SETTINGS = {
    "allow_auto_merge": "auto-merge",
    "allow_merge_commit": "merging with a merge commit",
    "allow_squash_merge": "squash merging",
    "allow_rebase_merge": "rebase merging",
}


def canonical(value):
    """The value with every list in an order of its own, so two listings of the same rules compare equal."""
    if isinstance(value, dict):
        return {k: canonical(v) for k, v in sorted(value.items())}
    if isinstance(value, list):
        items = [canonical(v) for v in value]
        return sorted(items, key=lambda v: json.dumps(v, sort_keys=True))
    return value


def compare(repo, expected, live):
    """(differences, not read) for one repository: lines a person reads."""
    differences, unread = [], []
    for setting, said in SETTINGS.items():
        if live.get(setting) is None:
            unread.append(f"{repo}: {said} (shown to an administrator only)")
        elif live[setting] != expected[setting]:
            differences.append(
                f"{repo}: {said} is {'on' if live[setting] else 'off'}, the file says "
                f"{'on' if expected[setting] else 'off'}"
            )
    want = {r["name"]: r for r in expected["rulesets"]}
    have = {r["name"]: r for r in live["rulesets"]}
    for name in sorted(want.keys() - have.keys()):
        differences.append(f"{repo}: the ruleset {name!r} is in the file but not on GitHub")
    for name in sorted(have.keys() - want.keys()):
        differences.append(f"{repo}: the ruleset {name!r} is on GitHub but not in the file")
    for name in sorted(want.keys() & have.keys()):
        for field in FIELDS + ADMIN_ONLY:
            if field not in have[name] or have[name][field] is None:
                if field in ADMIN_ONLY:
                    unread.append(f"{repo}: {name!r} {field} (shown to an administrator only)")
                else:
                    differences.append(f"{repo}: {name!r} has no {field} on GitHub")
                continue
            if field == "rules":
                differences += rule_differences(f"{repo}: {name!r}", want[name]["rules"], have[name]["rules"])
            elif canonical(have[name][field]) != canonical(want[name].get(field)):
                differences.append(
                    f"{repo}: {name!r} {field} differs\n    file:   {json.dumps(canonical(want[name].get(field)))}"
                    f"\n    GitHub: {json.dumps(canonical(have[name][field]))}"
                )
    return differences, unread


def rule_differences(where, want, have):
    """Each rule, by its type, that is missing, added, or set otherwise: only that rule is shown."""
    want = {r["type"]: canonical(r.get("parameters")) for r in want}
    have = {r["type"]: canonical(r.get("parameters")) for r in have}
    lines = [f"{where}: the {t} rule is in the file but not on GitHub" for t in sorted(want.keys() - have.keys())]
    lines += [f"{where}: the {t} rule is on GitHub but not in the file" for t in sorted(have.keys() - want.keys())]
    for t in sorted(want.keys() & have.keys()):
        if want[t] != have[t]:
            if isinstance(want[t], dict) and isinstance(have[t], dict):
                keys = sorted(k for k in want[t].keys() | have[t].keys() if want[t].get(k) != have[t].get(k))
            else:
                keys = []
            for k in keys or [None]:
                file_side = want[t].get(k) if k else want[t]
                github_side = have[t].get(k) if k else have[t]
                lines.append(
                    f"{where}: the {t} rule's {k or 'settings'} differ\n    file:   {json.dumps(file_side)}"
                    f"\n    GitHub: {json.dumps(github_side)}"
                )
    return lines


def fetch_http(path):
    request = urllib.request.Request(API + path, headers={"Accept": "application/vnd.github+json"})
    token = os.environ.get("GITHUB_TOKEN")
    if token:
        request.add_header("Authorization", "Bearer " + token)
    with urllib.request.urlopen(request, timeout=30) as answer:
        return json.load(answer)


def fetch_gh(path):
    return json.loads(subprocess.check_output(["gh", "api", path]))


def read_live(repo, fetch):
    """The repository's rulesets and its own settings as GitHub shows them to this reader."""
    rulesets = [fetch(f"repos/{repo}/rulesets/{r['id']}") for r in fetch(f"repos/{repo}/rulesets")]
    settings = fetch(f"repos/{repo}")
    return dict({setting: settings.get(setting) for setting in SETTINGS}, rulesets=rulesets)


def run(fetch):
    expected = json.loads(EXPECTED.read_text(encoding="utf-8"))["repositories"]
    differences, unread, failed = [], [], []
    for repo, want in expected.items():
        try:
            live = read_live(repo, fetch)
        except (urllib.error.URLError, subprocess.CalledProcessError, OSError, ValueError) as error:
            failed.append(f"{repo}: could not be read ({error.__class__.__name__}: {error})")
            continue
        found, missed = compare(repo, want, live)
        differences += found
        unread += missed
    lines = ["# Repository rules against `.github/rulesets.json`", ""]
    if not (differences or failed):
        lines.append(f"Every rule read matches the file ({len(expected)} repositories).")
    for title, items in [("Differ", differences), ("Could not be read", failed), ("Not read", unread)]:
        if items:
            lines += ["", f"## {title}", ""] + [f"- {item}" for item in items]
    text = "\n".join(lines) + "\n"
    print(text, end="")
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a", encoding="utf-8") as out:
            out.write(text)
    # A repository that could not be read is not a match, so it fails the run as a difference does.
    return 1 if differences or failed else 0


def self_test():
    rule = {"type": "required_status_checks",
            "parameters": {"required_status_checks": [{"context": "a"}, {"context": "b"}]}}
    merge_only = {"allow_merge_commit": True, "allow_squash_merge": False, "allow_rebase_merge": False}
    want = {"allow_auto_merge": True, **merge_only, "rulesets": [
        {"name": "Protect main", "target": "branch", "enforcement": "active", "conditions": {}, "bypass_actors": [],
         "rules": [{"type": "deletion"}, rule]}]}

    def live(**change):
        ruleset = dict(want["rulesets"][0], **change)
        return {"allow_auto_merge": True, **merge_only, "rulesets": [ruleset]}

    # The same rules listed in another order match.
    shuffled = {"type": "required_status_checks",
                "parameters": {"required_status_checks": [{"context": "b"}, {"context": "a"}]}}
    assert compare("r", want, live(rules=[shuffled, {"type": "deletion"}])) == ([], [])
    # A required check dropped, enforcement turned off, or a ruleset gone is a difference.
    dropped = {"type": "required_status_checks", "parameters": {"required_status_checks": [{"context": "a"}]}}
    found = compare("r", want, live(rules=[{"type": "deletion"}, dropped]))[0]
    assert len(found) == 1 and "required_status_checks rule's required_status_checks differ" in found[0], found
    # A rule removed altogether is named by its type.
    assert compare("r", want, live(rules=[rule]))[0] == ["r: 'Protect main': the deletion rule is in the file but not on GitHub"]
    assert compare("r", want, live(enforcement="disabled"))[0]
    assert compare("r", want, {"allow_auto_merge": True, **merge_only, "rulesets": []})[0]
    # One added in the settings page, not in the file, is too.
    extra = dict(want["rulesets"][0], name="Another")
    assert compare("r", want, {"allow_auto_merge": True, **merge_only, "rulesets": live()["rulesets"] + [extra]})[0]
    # Someone added to the bypass list is a difference when it can be read.
    assert compare("r", want, live(bypass_actors=[{"actor_id": 5}]))[0]
    # What GitHub does not show this reader is not read, and never a match or a difference.
    hidden = live()
    del hidden["rulesets"][0]["bypass_actors"]
    for setting in SETTINGS:
        hidden[setting] = None
    found, unread = compare("r", want, hidden)
    assert found == [] and len(unread) == 1 + len(SETTINGS), (found, unread)
    # Auto-merge turned off is a difference.
    assert compare("r", want, dict(live(), allow_auto_merge=False))[0]
    # Squash or rebase merging turned back on is a difference, each named.
    for setting in ("allow_squash_merge", "allow_rebase_merge"):
        found = compare("r", want, dict(live(), **{setting: True}))[0]
        assert len(found) == 1 and SETTINGS[setting] in found[0], found
    # The committed file is read, names the three repositories, and keeps each to merge commits, in its settings and
    # in every ruleset that names merge methods.
    repositories = json.loads(EXPECTED.read_text(encoding="utf-8"))["repositories"]
    names = set(repositories)
    for repo, written in repositories.items():
        assert {k: written.get(k) for k in merge_only} == merge_only, repo
        for ruleset in written["rulesets"]:
            for r in ruleset["rules"]:
                methods = (r.get("parameters") or {}).get("allowed_merge_methods")
                assert methods in (None, ["merge"]), (repo, ruleset["name"], methods)
    assert names == {"abbyshade111/StackVet", "abbyshade111/homebrew-stackvet", "abbyshade111/scoop-stack-vet"}, names
    print("rulesets_check self-test: ok")


def main():
    if "--self-test" in sys.argv[1:]:
        self_test()
        return 0
    return run(fetch_gh if "--gh" in sys.argv[1:] else fetch_http)


if __name__ == "__main__":
    sys.exit(main())
