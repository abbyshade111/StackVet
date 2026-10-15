# A page that answers 200 with a Content-Length and an empty body, with the server's own error in the app log, is counted as answered

**Status:** claimed by honesty, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the clean Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: high (the sign-in page was blank in a
browser and the report said the app "answered 68 of 68"). Verified by the orchestrating session in the rerun's
seen.json: `/login` 200, Content-Length 5415, body 0 bytes; `/signup` 200, Content-Length 5620, body 0 bytes; the app log
holds uvicorn's `RuntimeError: Response content longer than Content-Length` four times (the handlers render twice
and replace the body with a longer rendering).

**What is wrong.** The run reads a status and headers as an answer. A 200 whose body is shorter than its
Content-Length, or empty where the content type promises a page, is a broken answer, and the app log said why; sv
turned the consequences into unrelated gaps (a wrong "no sign-out link" finding on the account page, a record not
read back, V6.2.6 and V2.2.2 stopped) instead of one line: "five pages answered with no body; the server's log says
the response was longer than its Content-Length".

**Fix.** The run treats a body shorter than Content-Length, or an empty body with a text content type, as "not an
answer" and says so once, with the app-log line beside it; the checks that read those pages report not assessed with
that reason, never a finding about what the page lacks. A fake-app case that truncates its body.

**Record.** ADR-021 Later (what counts as the app answering).
