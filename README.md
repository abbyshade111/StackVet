<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/brand/stackvet-logo-dark.svg">
  <img alt="StackVet" src="docs/brand/stackvet-logo.svg" width="320">
</picture>

# StackVet (`sv`)

StackVet checks an app against the OWASP standards and writes reports that say plainly what was verified and
what was not. You write the app in whatever AI coding tool you like, in any language; you have the back-and-forth
with your own AI tool until the app is what you wanted, and then `sv` picks up the code and grades it.

![Three lanes, you, your AI coding tool, and sv, across five steps: plan, build, check, questions, and report. You describe the app, and the tool writes stackvet.toml with you; sv works out which requirements apply. The tool writes the code a feature at a time and calls stackvet_check after each one; sv says first what it did not examine, and the tool fixes what it finds. The tool asks you the questions only you can answer, and you record your answers with sv review. Then you run sv report --run at a terminal; sv starts the app behind its fence and writes the report folder.](docs/website/diagrams/how-it-works.svg)

**If you are not a programmer, start with [`docs/GETTING-STARTED.md`](docs/GETTING-STARTED.md).** It walks through
installing Docker, connecting StackVet to your AI tool, the prompt to start with, and reading the report, one step
at a time. The rest of this page is written for programmers, and its commands build StackVet from its source code.

## What `sv` checks, and what it writes

The compliance engine, the scanners, the container runner, the checks and the reports all run. What is not
built is listed in `docs/backlog/` (one file per item; `docs/BACKLOG.md` is its guide and roadmap), and the reports say plainly which parts of an app nothing has examined. Which
requirements of ASVS, AISVS, and the Secure by Design checklist any check can speak to at all, and what
each check needs to run, is counted in `docs/COVERAGE.md`, and drawn as one bar per ASVS chapter in
[`docs/coverage-by-chapter.svg`](docs/coverage-by-chapter.svg) (by level: [`docs/coverage-by-level.svg`](docs/coverage-by-level.svg)); AISVS by level is
[`docs/coverage-aisvs-by-level.svg`](docs/coverage-aisvs-by-level.svg), and the Secure by Design checklist by domain
[`docs/coverage-sbd-by-domain.svg`](docs/coverage-sbd-by-domain.svg). `docs/REQUIREMENTS.md` lists every ASVS and AISVS requirement by level and
family, with the checks that speak to each.

### The commands

```bash
cargo run -p sv-cli -- init              # the stackvet.toml spec to hand to your AI tool
cargo run -p sv-cli -- scope ./my-app    # which requirements apply to this app, and why
cargo run -p sv-cli -- run ./my-app      # start it behind the network fence and ask it questions
cargo run -p sv-cli -- check ./my-app    # credentials, configuration, and rules that read the code
cargo run -p sv-cli -- sbom ./my-app     # what the app ships, as CycloneDX JSON
cargo run -p sv-cli -- audit ./my-app --advisories ./osv   # against known vulnerabilities
cargo run -p sv-cli -- report ./my-app   # the whole thing, written out to read and to keep
cargo run -p sv-cli -- report ./my-app --advisories ./osv  # …with known vulnerabilities in it too
cargo run -p sv-cli -- before-live ./my-app  # the checklist before going live, from the last report
cargo run -p sv-cli -- bundle ./my-app   # the app, its report and a SHA-256 for every file, in one zip beside the app
cargo run -p sv-cli -- dashboard ./app-one ./app-two --out ~/sv-dashboard.html  # one page for several apps, from their reports
cargo run -p sv-cli -- mcp --root ~/code  # serve the checks to your AI coding tool (see below)
```

### The languages it reads

| How `sv` reads it | What |
|---|---|
| As code | Python, JavaScript, TypeScript, Go, Ruby, PHP, Java, C#, Kotlin, Rust, C, C++, Dart, Swift, and shell scripts |
| Its code taken out, then read as code | A script written into a web page (a `<script>` block, an event handler, or a `javascript:` link) as JavaScript, reported against the page and the line it is really on; Jupyter notebooks as Python, cell by cell; Astro and EJS pages |
| As pages | Templates that cannot run code: Handlebars, Mustache, Jinja, Liquid, Twig, and Nunjucks |
| Not read, and no code rule claims anything while one is present | `.pug`, `.erb`, `.jsp`, `.cshtml`, and `.razor` files (the report names them, ADR-054), a page whose code could not be taken out, and any language not listed above |

A language outside that list is not guessed at: while a file `sv` cannot parse is present, no code rule claims
anything about the app at all, and the report says which language stopped it. The same holds one rule at a time: a
rule that has not been taught a language in your app claims nothing, and the report names the rule and the language.

### Exit codes

`sv check`, `sv report`, `sv audit`, `sv run`, and `sv probe` end with a status a CI job can act on: 0 finished, 1
something needs a person (`sv audit`'s known vulnerability, or a finding when you ask with `--fail-on`), 2 not
assessed (part of what was asked could not be checked), and 3 `sv` itself failed. Findings alone never fail a run
unless you ask; [the table in `docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#the-exit-codes) says exactly what each
number means for each command, with `--fail-on` and `--baseline`.

Packages in folders `stackvet.toml` says are not the app, such as example apps and test fixtures, are listed apart
and still count: that file is written by your AI coding tool, and naming a folder there must never hide a
vulnerability. `sv` holds itself to this every week, auditing the Rust files it is built from
(`.github/workflows/audit.yml`).

### Running the app

Running the app needs a container backend (Docker or Colima). Without one, everything that needs the app
running reports *not assessed* — never a pass, and never a failure.

The report's run record (`report.json`, `inputs`) names the helper images `sv` runs by digest either way. Its
`app_image_digest`, the digest of your app's own image, is read from the local Docker, so it is left out when Docker
is not running or the app was not run. Nothing else in the record changes.

`sv run` gives the app no network, so it cannot install its own packages, and an app that needs them never starts.
Put `install = true` under `[stack.run]` and `sv` installs them first, in a container of its own that sees only the
package list, never the app's code or its `.env`: from `requirements.txt` with every line pinned (`name==1.2.3`), or
from `package.json` with its `package-lock.json`. It runs none of the packages' own install code, so a Python package
with no ready-made download cannot be installed this way (ADR-052).

### The network

`sv` opens no network connection of its own, with four exceptions, drawn and listed in [the
fence](docs/ARCHITECTURE.md#the-fence-what-can-reach-the-internet-and-what-turns-it-on). Three you ask for by name: `sv
probe <address>` (a few read-only requests to the address you type), `install = true` (the install step above), and
`sv report` given a public project's GitHub or GitLab address in place of a folder (one download of its code, as an
archive, run only when you add `--run`, and its packages downloaded only when you add `--install`; ADR-154). The fourth is on unless you pass `--no-tools`: the outside tools, which `sv report` and `sv bundle` run (ADR-111), are
other people's programs, and semgrep fetches its rule packs (`p/security-audit`, `p/default`, and
`p/ai-best-practices` for an app that may call a model) from semgrep.dev every time it runs. Advisory data is
something you download and point it at; the list of packages your app depends on is yours, and a check that quietly
phones out is one you did not agree to. Semgrep's rules are under the Semgrep Rules License, which allows them for
internal use and not for resale; `sv` uses them only while nothing built on them is sold. gosec is started so that it
cannot download your app's modules or run a C compiler, and a tool that would read through a link in your app is not
run while the link is there. `sv run` starts containers, and Docker downloads any image it does not have yet.

### What a run asks the app

`sv run` also asks the running app questions as somebody who has not signed in: among them, what headers
and cookies it sends, what it says when asked for a page that is not there, whether it accepts a site it has
never heard of, whether it echoes requests back, and whether it leaves source control, documentation, or a
development console open to anyone. What those questions cannot reach — anything behind a login — is
printed as *not assessed* before any finding, because a suite that only tries the front door and says
nothing reads exactly like one that found nothing wrong.

### The app's own tests

If your app has its own tests, they can count too — but only for requirements they name. They run only when
`sv` starts the app (`sv run`, or `sv report --run`), with the command given as `test` under `[stack.run]`.
Write the id into the test, in its name or in a comment on the line above it:

```python
def test_V1_2_4_search_uses_bound_parameters():   # or: # covers V1.2.4
```

When the suite passes, `sv` reports those requirements as *tested by the app's own tests* and says which
file and line to go and look at. That is a status of its own, below *checked*: your AI coding tool wrote
the tests, and `sv` does not read whether each one asks what its requirement asks (ADR-050). Only code is
read, so an id in a Markdown note under `tests/` counts for nothing. If your runner can write a report of its own (JUnit XML, TAP, `go test -json`, or
jest's or Vitest's JSON), point `test-report` at it and
the tests that passed still count even when others in the suite failed — without one, `sv` sees a single
exit code and one broken test costs the credit of every other test. There is no clever matching behind this, on purpose: guessing that
`test_login` is about a particular authentication requirement would credit it on the strength of a name
somebody chose for other reasons. A test that names nothing is not evidence about anything in particular,
which is a perfectly fair thing for a test to be — most tests are.

### What a report holds

`sv report` writes the whole thing out, into a folder named `stackvet-report` inside the app's folder unless you
give `--out`. Add `--run` to start the app behind the fence and include what it answers. The outside security tools
that are installed (Bandit, gosec, Brakeman, and Semgrep) run unless you add `--no-tools`, which is the quick run.
CodeQL runs only when you turn it on with `[tools] codeql = true` in `stackvet.toml`: its license allows it only on an
app released under an open-source license, for academic research, or with a paid GitHub Advanced Security license,
and StackVet cannot tell which applies to yours. Off, the report does not name it. It writes fourteen files:

| File | What it holds | Who it is for |
|---|---|---|
| `report.html` | The whole report on one page: what to fix first, what was not examined, and every requirement that applies | You, in a browser: double-click it |
| `compliance.md` | Every requirement that applies, chapter by chapter, with what is known of each; the threats; what only you can check; and the tests worth writing | Anyone who needs the whole picture against the standards, you or someone you show it to |
| `security.md` | What the checks found, each in full (where, the code, how sure, why it matters, what to do, and how to set it aside), then what was not examined | Whoever fixes the code, you or your AI coding tool |
| `low-findings.md` | Every low-severity finding in full, where the other reports group them by rule | You or your AI coding tool, when you want the detail |
| `before-going-live.md` | What to do or confirm before the app is on the internet, in the order you would do it, with what could not be verified at its end. It credits nothing | You, or whoever puts the app on the internet |
| `fix-now.md` | What needs attention, worst first, each with where it is and what to do: the findings of security.md that are things to fix | You or your AI coding tool, first |
| `your-decisions.md` | The questions only you can answer and the checks only you can make, from compliance.md | You |
| `not-examined.md` | What this report could not look at, and why, with the outside tools' runs, from compliance.md | You, before you read anything as safe |
| `information.md` | What asks nothing of you: findings in test code, checks that found nothing, your AI coding tool's files, and the text of each requirement, from compliance.md and security.md | You, when you want the rest |
| `stackvet.sarif` | The findings as SARIF, a standard format for findings (named findings.sarif before 14 October 2026) | Code editors and GitHub's code scanning, which show a finding beside its line |
| `report.json` | The report's data as JSON | Programs, among them sv review, sv compare, and sv dashboard |
| `seen.json` | What sv saw of the running app, with every key and sign-in in it cut short | You, to see what a finding rests on |
| `sbom.cdx.json` | The app's bill of materials, every package it uses with its version, as CycloneDX JSON | Tools that read a bill of materials |
| `report.pdf` | The whole report as one file to read, print, or send: compliance.md's order, with each finding in full. US Letter paper unless stackvet.toml's report-paper setting asks for A4; its fonts show Western European letters only, so report.html is the complete version | You, or whoever you hand it to |

`compliance.md` and `security.md` open with the same lines on a small app, but they are written for different
readers. [The reports page](docs/website/reports-page.md) says when to open each, with an example of each.

The reports lead with what was **not** examined, say what each check covered when it found nothing wrong, and
nothing in them says a requirement passed — `sv` is not able to establish that, so it does not claim it.

Each requirement gets one of nine statuses, strongest evidence first, from *needs attention* and *checked* down to
*stated by the AI coding tool* and *not verified*. Only a check of `sv`'s own can make one *checked*; the app's tests,
your own word through `sv review`, and the tool's word each rank lower. [The ladder in
`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#the-nine-statuses-strongest-evidence-first) draws all nine with who can
grant each.

They also list the tests worth writing: every requirement that applies and has no evidence yet, and no
test in the app naming it, lowest level first. A passing test that names the requirement's id is the one way
for the app itself to add evidence about the many requirements no check here reaches. It counts as *tested by
the app's own tests*, below *checked*, and never for a requirement about documents or how the app is run.

The reports also have a threat model: what could go wrong with an app like this one, by the part of it
each threat concerns (sign-in, stored data, the AI model, uploads, payments, and so on), with what the
checks showed about each: found, checked in part, not verified, or not known to apply until a question
in stackvet.toml is answered. (For a threat, *checked in part* means some of its requirements were checked; the
app's own tests and the owner's word never move one.) It is made from rules, not by asking an AI, and it never calls a threat
handled, because a threat is only as settled as the requirements that answer it. How it came to be made this way is a
dated reading, `docs/research/THREAT-MODELING.md`, kept as history.

### Your AI coding tool's own files

The reports also read your AI coding tool's own files in the app's folder, in a section of their own, apart from the app's
grade: the commands its settings run, the permissions that let it act without asking, the MCP servers it starts, and
any characters hidden in its instruction files (`AGENTS.md`, `CLAUDE.md`, and the like) that a person reading them
cannot see (ADR-049).

### What the AI coding tool did, from a hook you install

`sv connect claude --hook` prints a Claude Code hook for you to add to `.claude/settings.local.json`; `sv` never writes
it. After each command Claude Code runs and each file it writes, the hook adds a line to a record kept outside the
app's folder, in `~/.local/state/stackvet/hook-records/`: the command, with any key cut short, or the file's path, and
never a file's contents, a command's output, your prompts, or the tool's words. `sv report --hook-record` reads it for
that report alone: a section of its own says how many commands and file writes it saw, which ran the tests and whether
the last of them failed, each time the tool ended a turn saying the tests pass when its own record shows no test
ran, the last one failed, or files were written since (the hook keeps that the tool said so, never its words), and
whether the record was changed since the last report. It credits nothing, and the tool,
which runs as you, could have changed the record before then (ADR-122).

### The questions no tool can answer

Nineteen of the requirements at level 1 and 2 ask for a written decision and nothing else: what counts
as valid input, who may do what, how long somebody stays signed in, how soon a library with a known
vulnerability gets updated. Nothing can read those out of the code, because what they ask for is a
decision somebody made.

`sv notes` writes `security-notes.md` into your app's folder: one question per requirement, in plain words,
with what `sv` already found underneath it — the outside services by the package that showed each one,
the kinds of data you said the app holds. You write the answer. Running it again keeps everything you
have written.

An answer makes that requirement **documented by the owner** in the report. That is its own line in
the table and never *checked*: nothing reads whether your answer is right, or whether the app does
what it says. A check that found a problem always wins over what the notes say, and an answer cannot
settle a threat in the threat model — otherwise an app could talk its way out of one by describing
itself.

Some requirements are about where the app is *served from* rather than what is in it, and no amount
of reading the code settles them. `sv probe https://your-app.example.com` asks your own live site the
questions that matter most: is the certificate one browsers trust, is plain HTTP still served, does
it tell browsers to stick to HTTPS, do its cookies carry the `__Host-` prefix, does it still accept
the old TLS 1.0 or 1.1, and does it staple its certificate's revocation status.

It is deliberately narrow about what it will do. The address has to be typed at the terminal, never
read from a file. It fetches headers only, sends no cookies and no credentials, makes at most four
requests, and will not follow a redirect to any host but the one you named. It cannot sign in and
cannot change anything. `--api /path` names an address of your app's API on the same site, and adds one
request: that address over plain HTTP, asked the way a program asks, since an API should refuse plain
HTTP rather than redirect it (ASVS V4.1.2).

About fifty of the requirements that apply to a typical app at level 2 cannot be settled by any tool at all
(49 and 50 for the examples `notes-with-users` and `flask-booking`, counted on 8 October 2026), and
the report has a section for them: **what only you can check**, with a line each saying what
doing something about it involves — write it down in the notes, answer it in `[design]`, or go and
look at the live site and here is what at. Nothing on that list is counted as met. Doing the thing is
what would change that, not reading about it.

One thing you can state as a number, and `sv` will hold your app to it: how many wrong passwords in
a row it should allow before pushing back. Put `failed-sign-ins = 5` under `[policy]` in
`stackvet.toml` and the checks make seven wrong attempts, two more than you allowed, and watch what the app
does (at most 26, so a number above 24 is reported as not assessed). That settles
V6.3.1, one of the Level 1 requirements, and it is a real check rather than your word: an app that
only gives way after twenty attempts, when you said five, is reported. Say nothing and nothing is
claimed either way.

Others ask how the app is built rather than what is in it: is input checked on the server as
well as in the browser, do the app's own parts prove who they are to each other. You answer those in
the `[design]` section of `stackvet.toml` with yes, no, or not sure, and where in the code it is
done.

Answering yes makes the requirement **attested by the owner** — the weakest thing the report says of
your own word (only the AI coding tool's answer counts for less), and deliberately so. It is your word about the app, not a check of it, so the requirement stays on
the list of tests to write, and it cannot settle a threat. Answering **no** is the more useful
answer: the report says plainly that the control is missing, on your own say-so. If `where` names a
file that is not there any more, the report says that too, rather than keeping a pointer that leads
nowhere.

For an app that calls an AI model, semgrep's rules about such apps are read against AISVS too: user
input placed in the system instructions, no limit on how long an answer may be, a model called in a
loop with no way out, an MCP tool that hands the model a password. Each of those, when found, marks
the AISVS requirement it breaks as needing attention. Finding none marks nothing as checked, because
the absence of one way to get it wrong is not the control AISVS asks for.

The OWASP Secure by Design checklist is read too, alongside ASVS and AISVS. Its thirty-six controls are
design review rather than scanning — whether trust zones are enforced, whether an incident response plan
is rehearsed, whether your data has named owners — so nothing here can check a single one of them, and
the reports say exactly that rather than counting them as things that were looked at. Two of them can be
written down: SBD-MT-06 (what you do if something goes wrong) and SBD-AC-06 (the rules that might apply to
the app), as sections of a `design-decisions.md` in the app's folder, which the design-time prompts write.
A section marked as yours and recorded with `sv review` counts as *documented by the owner*; nothing in that
file ever counts as checked. Its ids are written
`SBD-AC-01` to keep them apart from AISVS Appendix C, which numbers its own requirements `AC.1.1`.
The checklist has no levels; each control takes the level of the ASVS requirement that asks the same
thing (`data/sbd-asvs-crosswalk.json`), or is shown at every level when nothing in ASVS does. How `sv` decides which requirements
apply, from `stackvet.toml` and the code, and the two answers that set the level, are drawn in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#how-sv-decides-what-applies).

## Setting a finding aside, confirming an answer, or giving your own: `sv review`

A finding set aside as a false alarm or a risk you accept, an answer of your AI coding tool's that you have checked and
confirm, and your own answers (`by = "owner"` in `stackvet.toml`, `Written by: owner` in `security-notes.md`) count as
yours only once **you** record them, in your own terminal:

```bash
sv review ./my-app
```

Until then they count as the tool's word, a step lower, and the report says so: the tool rewrites code until a warning
stops, and writing `owner` into a file is the easiest way to stop one. `sv review` needs a terminal someone is typing
in, which an AI coding tool does not have. It shows each entry, asks your name or `owner`, and *signs* it with a key of
its own in `~/.config/stackvet/`, outside the app, protected by a passphrase if you choose one; reports made anywhere
else count the entry only if they are given the key's public half, one line `sv review` shows you. It also asks you to
confirm the two answers that set the app's level.

With only Docker, the same from the container (make `~/.config/stackvet` first; the full steps are linked below):
`docker run --rm -it --network none -v "$PWD":"$PWD" -w "$PWD" -v "$HOME/.config/stackvet":/sv-config/stackvet -e XDG_CONFIG_HOME=/sv-config ghcr.io/abbyshade111/stackvet-sv review .`

[`docs/REVIEW.md`](docs/REVIEW.md) is the whole of it, with a picture: what each kind of entry is, the key and its
passphrase, the list of trusted keys and how a signature is checked, what a seal cannot show, and the setup for the
container and for CI (`SV_TRUSTED_SEALS`, and where GitHub keeps it).

## Signing in

`sv run` asks the running app questions as somebody who has not signed in — and, when
`stackvet.toml` says how, as signed-in users too. Under `[stack.run.users]` you say how accounts are
made (a `seed` command run inside the app's container, or the app's own `signup`), how to sign in and
out, which pages are private or admin-only, and how one user creates something another must not read.
`sv` makes two ordinary accounts and, if you list admin pages, an admin, each with a password made for
that run, and then asks, among other things:

- can somebody who has not signed in open a private page? (V8.2.1)
- can an ordinary user open an admin page? (V8.2.1)
- can one user find, read, change or delete what another created? (V8.2.2; finding it on the private pages and
  at `list`, and changing and deleting it with `update` and `delete`, all under `owned`. When only reading was
  tried, it is *checked in part*, never *checked*. A change or deletion that was refused counts only when the
  owner's own request does change or delete their record, so a request that never reached a route is not taken
  for a refusal.)
- does a private page let another website read it, with the user signed in? (V3.4.2; only ever a finding)
- is a request from another website accepted with the user's cookies? (V3.5.1)
- does signing in issue a new session, and does signing out end it? (V7.2.4, V7.4.1)
- is the session cookie out of reach of scripts and other sites? (V3.3.4, V3.3.2)
- is the session id long enough to guess, and different each time? (V7.2.3; only ever a finding)
- does a known default account, such as `admin` / `admin`, sign in? (V6.3.2; only ever a finding)
- is a password accepted in the address rather than the body? (V14.2.1; only ever a finding)
- is the password field on the sign-in and sign-up pages masked, and can a password be pasted into it?
  (V6.2.6; V6.2.7, only ever a finding)
- does visiting the sign-out address, rather than submitting its form, sign the user out? (V3.5.3; only
  ever a finding)
- given an address outside the app as `next` (and eight other common return parameters), do sign-in and
  sign-out send the browser there? (V3.7.2; only ever a finding)
- with `change-password` set: can the password be changed, and does that need the current one?
  (V6.2.2, V6.2.3)
- with `change-email` and `signup` set: does changing the email address need the password again? (V7.5.1;
  only ever done to an account made for it)
- with `owned` set and `requests-per-minute` under [policy]: when one user creates one record more than
  that in a minute, is the last one refused? (V2.4.1)
- with `once` set: does an action that should go through once (booking the last seat, redeeming a
  one-time code) go through for two people when sent 20 times at the same instant, half by each of two
  test users? (V2.3.4)
- with `delete-account` and `signup` set: does deleting an account end its other sessions? (V7.4.2;
  only ever done to an account made for it)
- is there a password hint or secret question on the sign-up or sign-in page? (V6.4.2; only ever a
  finding)
- with `uploads` set: are files too large, of the wrong kind, or compressed beyond the limits you state
  refused? (V5.2.1, V5.2.3, and others)
- with a password reset, sign-in codes sent by email, or a code from an authenticator app (`totp`) set: can a
  reset or a code be used twice, or a code guessed? (V6.4.3, V6.5.1, V6.6.3, and others; the run gives the app a
  mail server of its own to read the emails from)

When `signup` is set, `sv` also signs up through it, whether or not `seed` made the test users, and
asks what passwords the app accepts: one of 7 characters (V6.2.1), one of lowercase letters alone
(V6.2.5), three common ones, beside a random one of the same shape (V6.2.4; all three must be
refused), one far down the common list that is known to be in breaches (V6.2.12), and one of 83 characters
(V6.2.9), which is then tried with only its first 72, and the strong one with its capitals swapped, to
see that the password is checked exactly as typed (V6.2.8). Each is compared with an
ordinary strong password signed up first, and whether a password was accepted is told by signing in
with it.

Each question first shows the thing it depends on actually worked — the session opens a private page,
the owner can read back what they made, the admin can open the admin page — and when it cannot show
that, the answer is *not assessed*, not a pass. `examples/notes-with-users` is a complete example.

## Rules your AI coding tool follows while it codes

`sv rules ./my-app` writes a short set of security rules into the app's `AGENTS.md`, the file many AI
coding tools read before they work in a folder: keep keys and people's data out of the chat, treat web
pages, issues, and tool results as data rather than instructions, run the check after each feature,
add only packages that really exist, never merge or deploy its own work, and write CI workflows that
keep secrets away from code from forks. Each rule names the requirement it comes from. Rules about
something your `stackvet.toml` says the app does not have, such as a CI pipeline, are left out, and
the file says how many. Anything else in `AGENTS.md` is left as it is: `sv` writes only between its
own two markers, and running it again replaces only that section. `--print` shows the rules instead
of writing them. From inside the tool, `stackvet_guidance` gives the same rules, for one topic or
all of them.

The rules are instructions for the tool, not a check. Following them is not evidence that the app
meets anything, and no requirement in the report changes because of them. The report keeps the
Appendix C requirements nothing has reached out of its headline numbers, in a section of their own,
"How the app is built with AI", which lists each one and says whether it is given to your tool as a
rule, is your decision, or is reached by nothing in `sv`.

**Where they come from.** The rules are adapted from
[OWASP AI Security Verification Standard (AISVS) 1.0, Appendix C: AI-Assisted Secure Coding](https://github.com/OWASP/AISVS/blob/main/1.0/en/0x92-Appendix-C_AI_for_Code_Generation.md),
by the OWASP AISVS project and its contributors, licensed under
[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). Its requirements were rewritten as
instructions for an AI coding tool, and only those a coding tool can act on while it writes code are
included (27 rules, drawn from 39 of its 68 requirements; the owner's decisions among the rest are asked
through `sv questions` and the security notes). Every copy `sv` writes carries this credit and the license, and the adapted text is
shared under the same license. It does not reach your app's own code.

If your tool reads another file instead of `AGENTS.md` (Claude Code reads `CLAUDE.md`, for example),
point that file at it; for Claude Code, a line `@AGENTS.md` in `CLAUDE.md` should do it. Neither that
nor which tools read `AGENTS.md` on their own has been tried here yet.

## A zip to keep or hand on

`sv bundle ./my-app` writes one zip beside the app folder (`--out FILE.zip` puts it elsewhere, but never inside the app):
the app's files under `app/`, the report and the list of what the app ships under `report/`, a `BUNDLE.json` with which
`sv` made it, when, and a SHA-256 for every file, and a `README.txt` in plain words. It takes the same `--run`, `--no-tools` and
`--advisories` options as `sv report`.

**It leaves out anything that could hold a secret, and says so.** Left out, and listed with the reason in the zip and on
the screen: any file the credential scan flagged; environment files (`.env` and `.env.*`, but not `.env.example`); files named
like key stores or credential files (`.pem`, `.key`, `id_rsa`, `.npmrc`…); database files (`.sqlite`, `.db`…); links, which can
lead outside the folder; editor folders, which can hold a token; and any file the credential scan could not read, other than
plain images and fonts. What it cannot do: tell which files hold data about the app's people. It leaves out the database files
it recognizes by name and nothing else, so look through the zip before you hand it on. The zip is written stored (not
compressed) by `sv` itself, so it adds no dependency.

## From inside your AI coding tool

`sv mcp` offers the same checks over the Model Context Protocol, so the tool you build with can run them
mid-conversation and work through the findings with you. Install it once (`sh tools/install.sh`, which
builds it and puts it with its data in `~/.local/share/stackvet`, linked from `~/.local/bin/sv`), then
register it — for Claude Code, which needs its `claude` command installed (the Claude desktop app does not install it; there, use the `.mcp.json` in the Claude section of `docs/GETTING-STARTED.md`):

```bash
claude mcp add stackvet -- ~/.local/bin/sv mcp --root ~/code
```

or, for a tool configured with JSON:

```json
{ "mcpServers": { "stackvet": { "command": "/path/to/sv", "args": ["mcp", "--root", "/home/you/code"] } } }
```

It offers twelve tools. Three write a file, and only the check, the report, and an answer recorded as the tool's
own can count toward a requirement; everything else is advice for the tool, written nowhere and crediting nothing.

| Tool | What it gives | Writes | Counts toward a requirement | At a terminal |
|---|---|---|---|---|
| `stackvet_status` | Whether everything is ready, one line each | nothing | no | `sv doctor` |
| `stackvet_spec` | The `stackvet.toml` to write, for the tool to write with you | nothing itself | no | `sv init` |
| `stackvet_plan` | The plan from `stackvet.toml`, before any code (below) | nothing | no | `sv plan` |
| `stackvet_before` | One feature's brief before it is built (below) | nothing | no | `sv brief --feature` |
| `stackvet_preflight` | What `sv run` will need, read from the code without running it (below) | nothing | no | `sv preflight` |
| `stackvet_guidance` | The rules to follow while coding ("Rules your AI coding tool follows while it codes", above) | nothing | no | `sv rules --print` |
| `stackvet_prompts` | Prompts from [the prompt library](docs/PROMPTS.md), each saying whether it has been shown to work | nothing | no | `sv prompts` |
| `stackvet_check` | What applies, what was found, and first of all what was not examined; its section `questions` gives the questions only you can answer, for the tool to ask you one at a time | nothing | yes: the checks' own results | `sv check`, `sv questions` |
| `stackvet_explain` | A requirement in its framework's own words | nothing | no | `sv explain ID`, which adds the checks that speak to it and, with `--app DIR`, what that app's last report said |
| `stackvet_write_report` | The full reports | the report folder inside the app | yes: the report | `sv report` |
| `stackvet_record_answer` | An answer under one question in `security-notes.md`, the file your written decisions go in, made if it is not there | `security-notes.md` | only as the tool's own word, the lowest rank, until you change its `Written by:` line to `owner` and record it with `sv review` | `sv notes` makes the file |
| `stackvet_bundle` | One zip of the app and its report, to keep or hand on ("A zip to keep or hand on", above); a tool offers it once the report is written | a zip beside the app, never inside it | no | `sv bundle` |

The ten coding prompts shown to work are also given in full at the end of `sv init` and in `stackvet_spec`'s first
answer, since a prompt given where the tool starts did better than the same prompt fetched mid-build. The server's
opening instructions say where they are rather than carry them, since an AI tool pays for its instructions again on
every turn.

The loop the server is built for, and the two things it never does:

```text
 you ──prompt──► AI coding tool ──► stackvet_spec ──► stackvet.toml, written with you
                      │
                      ▼
               writes a feature ──► stackvet_check ──► what was not examined, then the findings
                      ▲                                        │
                      └──────────── fixes ◄────────────────────┘
                      │
                      ▼
       stackvet_check's questions ──► asked to you, one at a time
                      │
                      ▼
       stackvet_record_answer (the tool's word) ──► stackvet_write_report ──► the report folder

    ✗ never starts the app: that is `sv report --run`, at a terminal
    ✗ never runs the outside tools (Semgrep and the rest): that is `sv report`, at a terminal
```

| Limit | What happens past it |
|---|---|
| A check runs 50 seconds | The tool is told it did not finish and that nothing was assessed, rather than left waiting. Checking this whole repository takes about six seconds; `sv mcp --time-limit SECONDS` changes the limit, and `sv report` at a terminal has none. A tool that asks to hear how a check is going is told each stage as it starts. |
| An answer is about 40,000 characters | A longer plan or check comes in parts, since Claude Code saves anything longer to a file instead of reading it. The first part starts with what to act on (for a plan, what to decide and what `sv run` needs; for a check, what was not examined and then the findings) and ends with every section and how to ask for it (`section`, and `page` for a long one). Nothing is left out, and `"section": "all"` still gives the whole answer at once. |
| A request is 1 MiB | A longer request line gets an error in reply; a tool call is a few hundred bytes. |

It also offers [the design-time prompts](docs/prompts/design-time.md) as MCP prompts, for you to choose from
your tool (where it shows them, for example as slash commands): what to decide with the tool before any code is
written, each saying whether it has been shown to work. Its instructions ask the tool, for an app with no code yet, to
write `stackvet.toml` with you first, for the app as it will be, and to go through the prompt for each feature
before writing it.

**A plan before any code** (`sv plan`, `stackvet_plan`) turns `stackvet.toml` into the requirements that will
apply, the decisions and questions to settle before each feature, the tests worth writing (named so they count once
they pass), what the app must give `sv run` to be tested running (test accounts, the sign-in and sign-out forms, the
private pages, and so on), and the threats your answers raise. It is what the app will be held to, not evidence that
anything was built, and it is built from the same parts as the report, so the two agree about what applies.

**Before each feature** (`sv brief --feature uploads`, `stackvet_before`; `sv brief` alone lists the twelve features,
from sign-in and uploads to payments, an AI feature, and several customer organizations in one app) gives the
requirements the feature brings, including those that will apply once `stackvet.toml` says the app has it; the
design-time prompts for the decisions to make first, in full; the coding rules, the coding prompts shown to work, and the tests for it; and the settings
`sv run` needs to test it. It answers before `stackvet.toml` exists too, saying that which requirements apply cannot
be known yet.

**Before `--run`** (`sv preflight`, `stackvet_preflight`) looks in the app's files for what `sv run` will need from
`[stack.run]` (a server listening on every address at `$PORT`, a seed that makes the test accounts, the sign-in form
the settings name, and so on) and marks each *looks right*, *look at this*, or *could not tell*. It reads text and
runs nothing, so "looks right" means the thing was found, not that it works; the point is that the tool can fix a
wrong setting in the same conversation, before a run is spent on it.

A `.mcp.json` in the app's folder, with the JSON above, works in tools that have no `claude` command,
such as the Claude desktop app.

### Without installing Rust: the container

`sv` is published as an image, so the only thing to install is Docker (on a Mac, Docker Desktop or
Colima):

```bash
docker pull ghcr.io/abbyshade111/stackvet-sv
```

It is built from the `Dockerfile` by CI on every change to `main`, once the image has passed its
test, and tagged `latest` and with the commit it came from. Each one is signed with a statement of what built it:
this repository, its workflow, and that commit. To check the one you pulled came from there, with the
[GitHub CLI](https://cli.github.com/):

```bash
gh attestation verify oci://ghcr.io/abbyshade111/stackvet-sv:latest --owner abbyshade111
```

To build it yourself instead, from the repository root: `docker build -t ghcr.io/abbyshade111/stackvet-sv .`

Then the tool starts it in its `.mcp.json`. Use your own folder in all three places, and the full path
to `docker`, since an app started from the Dock often cannot see `/opt/homebrew/bin`:

```json
{ "mcpServers": { "stackvet": {
  "command": "/opt/homebrew/bin/docker",
  "args": ["run", "-i", "--rm", "--network", "none",
           "-v", "/Users/you/code:/Users/you/code",
           "ghcr.io/abbyshade111/stackvet-sv", "mcp", "--root", "/Users/you/code"] } } }
```

- The folder is mounted at the same path inside, so the paths in findings are your own.
- `--network none` means the container has no network at all, so the promise that `sv` opens no
  connection is enforced rather than only kept.
- On Linux, add `"--user", "1000:1000"` (your own `id -u` and `id -g`) before the image name, so the
  files it writes are yours. Without it the image runs as a user of its own, never root, and that user
  cannot write into your folder. On a Mac, Docker Desktop makes what it writes yours either way.
- Docker (or Colima) has to be running when the tool starts, or the stackvet tools are simply absent.
- The container never runs `sv report --run`: starting the app means starting containers, which from
  inside a container would mean handing it control of Docker on your machine. Run that step with a
  native `sv` at a terminal.

`tools/image_smoke.py` drives the image as an AI tool would, and CI runs it on every change.

Two limits are deliberate. It only reads apps under the folder given to `--root`; a path outside it is
refused, `..` and symbolic links included. And it never starts your app or runs other people's security
tools — each of those runs code, and that stays your decision at a terminal (`sv report --run --tools`).
The results say both were not done, the same way the written report does.

## On each pull request: the GitHub Action

If your app's code is on GitHub, `sv` can check every pull request. Add this file to the app's repository as
`.github/workflows/stackvet.yml`:

```yaml
name: StackVet
on: [pull_request]
permissions:
  contents: read
  security-events: write   # to show the findings in the Security tab
jobs:
  sv:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: abbyshade111/StackVet@main
        with:
          path: .           # the folder holding the app's stackvet.toml
```

Each run checks the image it pulls against its signature, then runs `sv report` with no network and
without write access to your code. What it found appears in three places: GitHub's code scanning (the
Security tab, and notes on the pull request's changed lines), a summary on the run's page, and the whole
report folder kept with the run as an artifact.

- **It fails the check only when a check could not run.** To fail it on findings too, add
  `fail-on: attention` (or `attention:high` for high and critical only).
- **Code scanning is free on a public repository.** On a private one it needs GitHub's paid code security,
  so the Action skips the upload there unless you set `upload-sarif: 'true'`; the summary and the report
  are kept either way. A pull request from a fork cannot upload either, since its token cannot write.
- **It never starts your app and never runs other people's tools** (it leaves out `--run`, and passes
  `--no-tools`): on a pull request, starting the app would run code you have not read yet, and the image carries
  none of the tools. Run those at a terminal.
- **For a compliance pipeline, `oscal: 'true'`** also writes the results as OSCAL assessment results, NIST's format,
  kept with the run as a second artifact. Only *checked* is concluded as satisfied and only *needs attention* as not
  satisfied; NIST SP 800-53 controls are named as related, never as assessed (ADR-156).
- **The app needs its `stackvet.toml` committed.** Without one, the Action stops and says how to make it.
- **It sees only the history the checkout fetched.** `actions/checkout` fetches one commit unless told
  otherwise (`fetch-depth: 0` for all of it), and the check for secrets files committed in the past reads
  that history.

### For a compliance pipeline: OSCAL

If an organization asks for the results in NIST's format for security assessments (OSCAL), for an authorization to
operate or a compliance tool: `sv report --oscal FILE` writes OSCAL 1.2.3 assessment results in the same run as the
report, `sv export --oscal [FOLDER] [--out FILE]` writes them later from a report `sv` wrote on this computer, and the
Action does it with `oscal: 'true'`. The file says no more than the report does. Only *checked* becomes "satisfied"
and only *needs attention* "not satisfied", each for one ASVS requirement. Every other status is passed on as what it
is, with whose word it is and how it was found out (a test, a document read, or someone's answer), and *not verified*
is neither. The NIST SP 800-53 controls each requirement relates to are named, never marked as met: a control asks
more than any one requirement does ([ADR-156](docs/adr/ADR-156.md)). The file goes outside the report folder, which
holds only `sv`'s own files.

## Building

Rust 1.95 or newer.

```bash
cargo test
```

The OWASP data files (`data/frameworks`, `data/knowledge`) and `sv`'s own data files live together in `data/`.
`sv` looks for that folder in `SV_DATA_DIR`, then beside itself (`data`, or `../share/stackvet/data`), then
in the repository it was built from; `sv --version` names the one it uses.

To contribute, read [CONTRIBUTING.md](CONTRIBUTING.md): the checks a change must pass and the rules every change
keeps. Taking part means agreeing to the [code of conduct](CODE_OF_CONDUCT.md); a security problem in `sv` itself
goes through [SECURITY.md](SECURITY.md), not a public issue.

## Reading order

Where to start depends on who you are. Nobody reads the design record end to end; it is the dated history.

| You are | Start with |
|---|---|
| Not a programmer, building an app with an AI tool | [`docs/GETTING-STARTED.md`](docs/GETTING-STARTED.md), the guide, step by step; its section 6 says what the report covers and what it leaves *not assessed*. [`docs/GLOSSARY.md`](docs/GLOSSARY.md) explains the words StackVet uses. |
| A programmer using `sv` | This page: [What `sv` checks, and what it writes](#what-sv-checks-and-what-it-writes) (the commands, the languages, the exit codes, and the report files), [From inside your AI coding tool](#from-inside-your-ai-coding-tool) (the MCP server), and [the GitHub Action](#on-each-pull-request-the-github-action). |
| Contributing to StackVet | [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md), the ten-minute map, first; then [`CONTRIBUTING.md`](CONTRIBUTING.md), [`docs/adr/README.md`](docs/adr/README.md) (why things are as they are), and [`docs/BACKLOG.md`](docs/BACKLOG.md) (what is left to do). |
| Researching how it was built | [`docs/paper/`](docs/paper/), starting with [`HOW-SV-WORKS.md`](docs/paper/HOW-SV-WORKS.md) and [`METHODOLOGY.md`](docs/paper/METHODOLOGY.md); then the design record, one dated entry per file in [`docs/design/`](docs/design/) ([`docs/DESIGN.md`](docs/DESIGN.md) says how it is kept), which explains the two changes from v1, why deleting the wizard was the hard part, and what the first run against real data turned up. v1 itself is archived: see [the last section](#if-you-used-stackvet-before-9-october-2026). |

## If you used StackVet before 9 October 2026

Each note here is for someone who set StackVet up before a change; a first-time reader can skip all of them.

- **The name.** Until 9 October 2026 the product was called SecureVibe and the image was
  `ghcr.io/abbyshade111/securevibe-sv`. That name is left as it is and gets no further pushes; the files `sv` wrote
  under the old names (`securevibe.toml`, `securevibe-report`, `~/.config/securevibe`) are still read, and the report
  says once what to rename (`docs/adr/ADR-062.md`).
- **Unsigned images.** Images published before 9 October 2026 carry no signature (`docs/adr/ADR-080.md`).
- **Two MCP tools folded into others.** Until 9 October 2026 there were thirteen tools: `stackvet_questions` is now
  `stackvet_check`'s section `questions`, and `stackvet_notes_file` is `stackvet_record_answer` called with no question
  and no answer. An AI coding tool that still calls either old name gets the same answer, with a line naming the tool
  to call instead; neither is listed.
- **Answers recorded with `sv review` before 6 October 2026** were sealed another way, with
  `~/.config/securevibe/review-key`, and count only on the computer that holds that key. The next time you run
  `sv review` there, it offers to sign them all again at one yes, without asking each question again.
- **An installed copy from before 5 October 2026.** The guide had you use the program in the build folder then; if
  your `PATH` or your AI tool's settings name `…/stackvet/target/release/sv`, change them to `~/.local/bin/sv`
  ([`docs/INSTALL.md`](docs/INSTALL.md#if-you-set-stackvet-up-before-5-october-2026)).
- **SecureVibe v1**, the earlier version that asked you to fill in a form and then wrote a Node app for you, is
  archived, not deleted: it lives on the `v1` branch (see `ARCHIVED.md` there) and at the tags `v1-paper` and
  `v1-final`. Its Zenodo version DOI, the one to cite, is 10.5281/zenodo.22984709. Until 26 September 2026 this
  repository held v1 at its top and `sv` under `agnostic/`; no history was rewritten, so every commit hash cited
  elsewhere still resolves.
