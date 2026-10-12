# prompt_trial.py has the one shell=True in tools/

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** The string is fixed, nothing from the build is interpolated, so there is no injection; it is the one `shell=True` in `tools/`, and CodeQL's Python leg scans `tools/`.

**Where.** `tools/prompt_trial.py:65-66`: `subprocess.run('git init -q && git add -A && git … commit -qm build --allow-empty', shell=True, cwd=dst, check=True)`.

**Evidence.** The call above.

**How it bites.** Tidiness and CodeQL noise; a later edit that interpolates a path would make it real.

**Fix.** Three list-form calls, as `image_smoke.py:83-85` does.

**Record.** No record governs `prompt_trial.py`; say so.
