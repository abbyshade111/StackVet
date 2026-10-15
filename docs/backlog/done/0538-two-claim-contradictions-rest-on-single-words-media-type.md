# Two claim contradictions rest on single words (`media_type`, `httpx`) and drove the only silencing seen in the trials

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity app. Severity: medium (a check whose
wording invites a rewrite). Verified against `data/claim-corroborators.json` and the transcript by the orchestrating
session.

**What is wrong.** `stackvet_check` reported two claim contradictions: "`httpx` is declared in requirements.txt"
(external-apis) and "`media_type` appears in app/main.py" (multimodal-ai). `media_type` is FastAPI's keyword for a
response's content type (`data/claim-corroborators.json:1962`); `httpx` was the test client
(`data/claim-corroborators.json:1221`). The tool removed `media_type="text/plain; charset=utf-8"` from
`app/main.py:176` ("Removing the explicit declaration should not change the functionality but will eliminate a
potential false positive") and moved `httpx` and `pytest` to a new `requirements-dev.txt` ("a test dependency
mistakenly listed in the main requirements"), which made sv's container run of the tests fail (`pytest: not found`).
It then told the owner "No code files or configurations were deleted or silenced."

**Fix.** For `multimodal-ai`, corroborate with AI or media-model libraries, never the word `media_type`; for
`external-apis`, a package only tests import is no evidence (read `requirements-dev.txt` and the test folder's imports
apart); and word every contradiction so it does not read as something to make go away: "the code names X, which
usually means Y; if the app does not do Y, leave the code and tell the person the manifest is right". The prompt side
is a separate item (silencing by rewriting).

**Record.** The claims record (grep docs/adr for `claim-corroborators.json`): a Later entry if what switches a
capability on changes.

**Done, 15 October 2026** (documentation session). Built:
- `media_type` is removed from `multimodal-ai`'s Python patterns. FastAPI's `media_type=` no longer reads as a
  picture sent to a model; an image block sent to one is still found by its `"type": "image"`.
- What only the app's tests show counts neither way: a pattern found only in a test file, or a declared Python or npm
  package that only test files import (`sv_scan::imports`, read as lines), answers the claim with nothing and says
  what it saw (`Evidence::OnlyInTests`). So `httpx` declared for the tests' client no longer contradicts "no outside
  services". A package no file is seen to import counts as before. `is_test_path` moved to `sv_scan::files`, and
  `sv_check::finding` re-exports it.
- Every contradiction is worded so it does not read as something to make go away: the code names something that
  usually means the app does this; if the app does not do this, leave the code as it is and tell the person; the
  requirements stay switched on. In the report's claim line, `stackvet_check`, and `sv scope`.
- ADR-015 has a dated "Later" entry. Tests: `crates/sv-scan/tests/only_in_tests.rs` (4) and
  `crates/sv-scan/src/imports/imports_tests.rs` (3). Three breaks on purpose (no package set aside, test files read
  as the app, `media_type` put back) each turned one of them red.

Not built: `requirements-dev.txt` is not read apart as a file; a package only tests import is set aside wherever it is
declared, which covers the trial's case. The prompt side (silencing by rewriting) is its own item, as the item said.
