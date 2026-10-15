# `ast.open-redirect` fires on `RedirectResponse(url="/")`: the literal-argument exemption does not see a keyword argument

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the clean Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: low (thirteen false findings in one
report). Verified: `app/main.py:165` and `:218` are `RedirectResponse(url="/", status_code=...)`.

**Fix.** The exemption in `data/ast-rules.json` matches the literal whether positional or `url=`; the tests cover only
the positional form today; add the keyword case for Python and the equivalent for the other languages.

**Record.** None.
