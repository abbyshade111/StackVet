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
