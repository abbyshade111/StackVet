# A page on stackvet.dev for checking somebody else's project

**Status:** claimed by claude/0671, 26 October 2026

**Asked for by the owner, 26 October 2026:** "let's pull together some info on this new feature to add to
stackvet.dev", the feature backlog 0670 built (ADR-154). Asked where it goes, with a recommendation, the owner chose
"A page of its own (Recommended)": a new page in the site's top menu, "Check a project", and one short line on the
home page pointing to it.

**What it is.** A page in plain words for the person who found a project on GitHub or GitLab and wants to know what
StackVet can say about it before using it: what the one command does, that one version is downloaded and nothing in it
runs, how the report is written for someone weighing a risk, that a project is started only when asked (and its
packages downloaded only with `--install`), and what a check cannot tell them. Every claim is checked against what
0670 built; what was not tried is said in the page's notes, not on the page.

**Done when.** `docs/website/check-a-project-page.md` holds the page's words; `tools/website.py` builds
`check-a-project.html` from them and lists it in every page's menu; the home page has one line pointing to it; the
site's self-test and the Site check pass; and ADR-132 has a Later entry with the owner's choice.
