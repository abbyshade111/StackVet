# Check any public code repository from its GitHub or GitLab address, for someone deciding whether to use it

**Status:** done, 25 October 2026

**Asked for by the owner, 25 October 2026:** "an option to use StackVet to evaluate any public code repository - for
example, supply a GitHub or GitLab link to the tool and have it run all the tests so a user can decide if they want
to accept the risk of using the code themselves."

**Order,** after 0375.

**What it is.** Today `sv` checks a folder on the person's own computer, an app they built. This is for a different
person: someone who found a project on GitHub or GitLab and wants to know, before using it, what `sv` can say about
it. They give `sv` the project's public address; `sv` fetches the code into a folder of its own and checks it as it
checks any app, and the report is written for someone deciding whether to take the risk, not for the person who
wrote the code.

**To decide before it is built,** each with its own record and the owner's word:

- **The fence.** `sv` opens no network connection of its own beyond three named exceptions (`docs/ARCHITECTURE.md`,
  "The fence"). Fetching a repository is a fourth: which hosts (GitHub and GitLab only, by their public addresses,
  or any address the person types), how (a shallow `git clone` over HTTPS, or the host's download of one commit as
  an archive), whether submodules and Git LFS files are fetched, and a size limit. The fetch must run none of the
  project's code: no hooks, no `git` filters, no install scripts.
- **Running it.** "Run all the tests" means three different things here. The static checks and the outside tools
  read the code and run none of it (ADR-032), so they are safe on a stranger's project. Running the project itself,
  as `[stack.run]` does for an owner's app, means building and starting code nobody has vetted, in the fenced
  container; whether to do that at all, only when the project carries its own `stackvet.toml`, or only when the
  person asks with a flag, is a decision. A project with no `stackvet.toml` has no way to be started or signed in to,
  so the running checks are said not assessed, with why.
- **The questions only an owner can answer.** Much of a report rests on the owner's answers in `stackvet.toml`
  (whether the app takes payments, signs people in, handles health data). A person checking someone else's project
  cannot answer them for it, so they stay unanswered, and the report must say plainly which requirements were left
  open because of that, never read silence as "no" (ADR-015).
- **What the report says, and to whom.** A finding in a stranger's project is not a task for the reader to fix but a
  risk for them to weigh, so the wording, the summary, and the "what to do next" change; and a project's own tests
  passing, or `sv` finding nothing, must never read as the project being safe (the honest-evidence rule).
- **Where it is kept.** The fetched copy lives in a folder `sv` makes and says where, so the person can delete it,
  and the report names the commit it checked, since the project may change the next day.

**Done when.** `sv report <address>` (or another form the decisions settle on) checks a public repository end to end,
with a test that fetches nothing from the network (a local repository served as the address), a test that a hook in
the fetched repository does not run, and the report saying which checks ran, which could not, and why.

**The owner's choices, 25 October 2026,** asked with a recommendation for each (ADR-154, proposed with this claim):
hosts, "GitHub and GitLab only"; how, "Download one snapshot"; starting it, "Only when asked, and only after running
all static checks first"; the report, "A reader weighing a risk".

In three parts, each a pull request:

1. **The fetch:** `sv report` given a GitHub or GitLab project's address downloads the host's archive of one commit
   through `curl` (HTTPS, those hosts, limits on redirects, size, and time), and unpacks it into a folder it names,
   refusing links and paths outside it and stopping at a limit; the commit is read from the archive. The fence's
   picture and table in `docs/ARCHITECTURE.md`, and CLAUDE.md's sentence, say four exceptions. Tests fetch nothing from
   the network: a local server stands in for the host.
   **Part status:** done, 25 October 2026 (ADR-154, Later). A stand-in for the host, not a local server: a server
   would need plain HTTP or a certificate of its own, and the fetch takes neither; the module's tests answer for the
   host, and a stand-in `curl` answers for it through the `sv` binary. A project with no `stackvet.toml` gets one of
   comments only, so its questions stay unanswered. Tried against both hosts the same day, at the owner's word: a
   GitHub and a GitLab project each fetched with its commit named, and reported on.
2. **The report for a reader weighing a risk:** the project, its address, and its commit named; the owner's questions
   left unanswered, and the requirements that leaves open said; findings worded as risks; never "nothing found" read as
   safe. A project that carries its own `stackvet.toml` holds its author's answers, not the reader's: the report
   says whose they are.
   **Part status:** done, 25 October 2026 (ADR-154, Later). Every page names the project, its address, and its commit
   at its top and says how to read it; the short version counts risks to weigh and never reads as safe; the next steps
   are the reader's; an author's `stackvet.toml`, and one changed since the fetch, are said to be. Each finding's own
   words and the short pages' titles are as they were, read under that banner.
3. **Started only when asked:** with `--run`, after every static check has finished, a project with its own
   `stackvet.toml` is started in the fence as any app is, and the report says at its top that unvetted code was run.
   **Part status:** done, 25 October 2026 (ADR-154, Later, and accepted). Started only with `--run`, after every check
   that reads its code, and only when it carries its own `stackvet.toml`; its packages downloaded only with the reader's
   `--install` (the owner's choice, "Only with --install (Recommended)"); every page's first line then says code nobody
   vetted was run.

