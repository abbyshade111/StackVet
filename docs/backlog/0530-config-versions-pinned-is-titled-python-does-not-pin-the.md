# `config.versions-pinned` is titled "Python does not pin the versions it installs" when every version is pinned

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), all three apps. Severity: low (wording).

**What is wrong.** Each app's `requirements.txt` pins every line with `==`; the finding's text says the problem is the
missing lockfile with hashes. The title contradicts the text, and a person reading only titles is misled.

**Fix.** Retitle to "No lockfile with hashes beside requirements.txt" (and the matching title for each ecosystem), keep
the text.

**Record.** None.
