# py7zr is pinned by version only, so its eight dependencies float in the job that holds the Docker Hub token

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** ADR-089 pins the one package; pip resolves pycryptodomex, pyzstd, pyppmd, pybcj, brotli, inflate64, multivolumefile, and texttable to whatever is newest on PyPI at run time, and a pip install runs each package's setup code, before the tests, with the Docker Hub token on disk (the item above).

**Where.** `.github/workflows/rust.yml:166` and `.github/workflows/windows.yml:62`: `python3 -m pip install py7zr==1.1.3`.

**Evidence.** `pip install py7zr==1.1.3` with no requirements file and no `--require-hashes`.

**How it bites.** One of those eight publishes a compromised release; the next CI run installs it.

**Fix.** A `tools/py7zr-requirements.txt` with every transitive package pinned and `--hash` lines, installed with `pip install --require-hashes -r`; Dependabot's pip ecosystem then moves it.

**Record.** ADR-089 governs the py7zr pin: a Later line; ADR-051 governs `rust.yml`.
