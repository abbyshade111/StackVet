# On Windows the checks that start the app are untried: the guide, the setup page, and sv doctor say so, and what that leaves unchecked

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Filed 14 October 2026 from the owner's question whether Windows users should be warned about running without the
Docker fence. What is true today: on Windows, StackVet is installed with Scoop and runs natively (ADR-108); the checks
that read the code and the outside tools need no Docker; the checks that start the app (`sv report --run`, the fence of
ADR-019) need Docker Desktop in Linux-container mode and have never been tried by hand on a Windows computer (ADR-108:
the release is smoke-tested, not suite-tested; the nightly Windows run leaves out the container tests). The report is
already honest after the fact: a run without `--run` says the running-app checks did not run and how many of the
requirements that apply only they can reach (`bluf.rs::not_run_line`), and a run whose fence cannot start takes the
honest-absence path. What is missing is the warning before the fact, where a Windows user decides what to expect:

- `docs/GETTING-STARTED.md` (the Windows passages at lines 83 to 85 and 324 to 327) and `docs/prompts/setup.md`'s
  "On Windows" section say Docker Desktop is "needed later only for the checks that start your app", which reads as
  if that path works. Say instead: those checks have not been tried on Windows; the report says which requirements
  they would reach; a Mac or Linux computer, or a cloud runner, is the known way to run them.
- `sv doctor` on a Windows build: one line saying the same, beside its Docker line.
- The setup page's Windows prompt: the same sentence, so the AI tool does not promise the person the full report.

Not a code change to the fence. Once somebody tries `sv report --run` on Windows with Docker Desktop and it works (or
does not), the sentences change to what was seen; 0375 (StackVet itself in Docker on Windows) stays separate.
**Record:** ADR-108 governs the Windows route: a Later line.
