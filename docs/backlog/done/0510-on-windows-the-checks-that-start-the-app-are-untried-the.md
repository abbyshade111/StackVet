# On Windows the checks that start the app are untried: the guide, the setup page, and sv doctor say so, and what that leaves unchecked

**Status:** done, 14 October 2026

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

**Done, 14 October 2026.** `sv doctor` built for Windows adds a line after its Docker line, *can't tell*, from
`doctor::WINDOWS_RUN_UNTRIED`: the checks that start the app have not been tried on a Windows computer, even with
Docker Desktop running; without them the report says which requirements only they reach; a Mac or Linux computer, or a
cloud runner, is the known way. It counts as nothing missing, and `stackvet_status` says the same. The setup page's
"On Windows" section opens with that sentence in place of "needed later only"; the Windows prompt has a step 11 in
which the AI tool tells the person and does not promise the full report; prompt 6 says it first on Windows. The guide's
step 3 and section 6 say it in fewer words (the guide stays at 4,199 of its 4,200), and its install step no longer
tells a Windows user to install Docker Desktop for a route that does not work there. ADR-108 has a Later entry. Tests:
`doctor/windows_tests.rs` and `tests/windows_run_untried.rs`, broken on purpose four ways (the prompt's promise
softened, the page's sentence turned into a claim it works, the guide's sentence dropped, the doctor line switched
off), each turning the one expected test red.

Not done: nothing was tried on a Windows computer; the sentences change when somebody does. Backlog 0375 (StackVet in
Docker on Windows) stays separate.
