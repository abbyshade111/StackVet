# `ast.open-redirect` fires on `RedirectResponse(url="/")`: the literal-argument exemption does not see a keyword argument

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the clean Cursor + Flash run of 15 October 2026 (backlog 0577). Severity: low (thirteen false findings in one
report). Verified: `app/main.py:165` and `:218` are `RedirectResponse(url="/", status_code=...)`.

**Fix.** The exemption in `data/ast-rules.json` matches the literal whether positional or `url=`; the tests cover only
the positional form today; add the keyword case for Python and the equivalent for the other languages.

**Record.** None.

**Done, 15 October 2026** (documentation). `ast.rs` reads a destination given by name: for Python, a keyword
argument named `url`, `location`, `redirect_to`, or `to` is judged by its value (`named_destination`), wherever it
stands among the keyword arguments, so `RedirectResponse(url="/", status_code=303)` and
`RedirectResponse(status_code=303, url="/login")` are a fixed address; in C# and Kotlin, where the query takes an
argument's first part, the label of `url: "/"` or `url = "/"` is skipped (`is_argument_label`) and the value is
judged by its own match. Only `ast.open-redirect` is affected (the one rule with `saysWhenChecked`). A destination
built from the request and given by name is still reported. Tests: `ast/keyword_destination_tests.rs` (2), written
first and seen failing on the trial's own line; with either half switched off, its test fails.
