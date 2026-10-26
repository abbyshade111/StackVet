# stackvet.dev: what the home page says

The words of the home page, settled by the owner section by section on 12 October 2026 (backlog 0224). They come
before any design: the mock-ups use them as they stand. **Nothing here is live**, and the site does not go live while
any line below marked *not yet true* is still not true of StackVet (the items are listed at the end). stackvet.dev is
the site; stackvet.app redirects to it.

Every fact here is checked against `main` and the guide. Where a number comes from a generated file, the site reads
it from that file when it is built rather than keeping its own copy.

## The look: direction C, in the logo's colors

Chosen by the owner on 12 October 2026 from three directions (A, the report up front, dark; B, the plain guide,
light; C, the conversation) and C in five palettes side by side (the logo's own, an orchard palette after Applejack,
harbor teal, plum, and blueprint navy). **Direction C, in the logo's own colors:**

- **The opening is a short conversation** beside the headline: a person describes their app to their AI tool, the
  tool says what StackVet decided before any code (the ASVS level, from what the app keeps), asks a question only the
  person can answer, and later reports a finding it fixed and checked again. It is labeled "An example, written to show
  how a build goes", and every step in it is something StackVet really does.
- **Type:** Atkinson Hyperlegible for everything a person reads (made for readers with low vision); JetBrains Mono,
  the logo's face, only for the wordmark and small labels.
- **Colors, kept with what each is for and its contrast in `docs/brand/README.md`:** page `#EEF0EA`, panels
  `#FFFFFF`, text `#14201C` (softer `#3B4842`, captions `#5A6660`), lines `#E1E3DB`; accent and buttons `#C2410C`
  with white text, accent text `#9A3412`; the question bubble `#FBEFE7`. Dark bands (the report, the footer)
  `#14201C` with `#F3F4EF` text, panels `#182A23`, "needs attention" `#FDBA74`, "checked" `#A7D7BF`.
- Every grid stacks to one column at phone width; every button is at least 44 px high.

## 1. The opening

**Vibe-code your app with security in mind from the first prompt.**

Connect StackVet to your AI coding tool before you start. As you describe your app, it tells your tool which OWASP
security rules apply, checks the code after each feature, and has your tool ask you the questions only you can
answer. Once your app works, StackVet writes a report: the most important things to fix first, then what was
checked, and what nobody has looked at yet.

Buttons: *Get started*, *See how a build goes* (the click-through of one real build, `build-walkthrough-page.md`, made
22 October 2026, backlog 0373).

*From a real build in Cursor, 14 October 2026, in the tool's and the person's own words, shortened, with one line
moved to the end and a few words of ours in square brackets:*

> **You:** I want to build: an application for my iPhone that I can share with my family (everyone has their own
> login) where we can post any games we've vibecoded and play each others' games.
> **Your AI tool:** No application code is written yet. Three lines that decide where the app runs, whether anyone
> signs in, and how it is reached are still open, so I need your answers before those go in the file.
> **Your AI tool asks:** Anyone who can open the site can create an account, and a new account can play every game.
> There is no invite code. Say if you want one.
> **You:** I want to do invite codes because I only want my family to have access
> **Your AI tool, later:** Creating an account now requires one shared invite code. … I checked a wrong code in the
> browser, and the page said it was not right.
> **Your AI tool:** [On two findings about passwords in its tests:] As I read them, these are passphrases I
> invented for the tests, not a password for a real account. I did not change the tests to hide them. If you agree
> they are false alarms, you can record that yourself with `sv review`.

- The conversation is the Cursor build of the setup prompt trial (`docs/research/SETUP-PROMPT-TRIALS-2026-10-14.md`),
  chosen by the owner on 22 October 2026 over a written example. Each line is quoted from the chat as it is kept on
  the owner's drive, with cuts marked "…". The person's two lines are the brief pasted into all three tools and an
  answer typed in reply. The last line was said in the same message as the question about invite codes, before the
  person answered; it is moved to the end, as the label says. Its words in square brackets are not the tool's:
  they stand in for the lines where the tool printed the test passwords, which are left out, as is every folder path. No line
  names a person.
- "Security in mind" says how the app is built, not what it ends up being: StackVet never says an app is secure.
- The order of the last sentence, the most important things to fix first, is true of the report since 13 October
  2026 (item 2 below, done).

## 2. Who it's for

**You don't need to be a programmer.**

StackVet is for people building an app by talking to an AI coding tool: Claude, Cursor, or the AI in VS Code. You
describe what you want and the tool writes the code. StackVet checks that code after each feature, and when a
question can only be answered by you, such as who should be allowed to see what, your AI tool asks you.

You don't need to know about security either. The report explains each result in plain words.

- Claude, Cursor, and VS Code are the three tools `sv connect` writes settings for.

## 3. How it works

**Three steps, set up once.**

1. **Connect StackVet to your AI tool.** On a Mac or Linux it runs in Docker, a free program that keeps it in a box of
   its own (Docker Desktop or Colima, either works); on Windows it is installed with Scoop. Setup takes about fifteen
   minutes, once.
2. **Describe the app you want.** Paste the starting prompt from the guide into your AI tool, with a few sentences
   at the top about your app: who uses it, what they do, and what it keeps about them. The tool writes the code.
   After each feature it checks its work with StackVet, and it asks you the questions only you can answer.
3. **Read the report.** One page in your browser. It starts with the most important things to fix, then what was
   checked, and what nobody has looked at yet. Your AI tool can read it through with you.

- Step 2 is for someone who has used an AI chat but never built an app with one.
- Step 3's order is true of the report since 13 October 2026 (item 2, done).

## 4. The report

**A report that puts what matters first and hides nothing.**

Most tools give you a green checkmark or a score. StackVet starts with the most important things to fix, states
every requirement in plain language, and highlights what it wasn't able to check or verify, so you know what to
follow up on.

Your AI tool can help with that part, too. It can read the report through with you, explain why it matters, fix
what needs attention, and help you answer the questions only you can.

*Example lines, not from a real app:*

| | |
|---|---|
| needs attention | Passwords are stored with a method that is too quick to guess against. |
| checked | No passwords or keys are written into the code. |
| checked in part | Pages tell the browser not to let other sites frame them, but only the pages it could reach were looked at. |
| not assessed | Whether signing out really ends the session: the app was not started, so this could not be tried. |
| not verified | Whether backups are kept safe: nothing in StackVet can look at this. You can answer it yourself. |

None of these words means "passed." Even *checked* means one check looked and found nothing wrong, not that the
whole requirement is met.

- "Starts with the most important things to fix" is true since 13 October 2026 (item 2, done). "Every requirement
  in plain language" is *not yet true* (item 3): today each requirement is shown in OWASP's formal wording, with
  checks named by internal ids.
- The example lines are invented and say so; a real report from one of the example apps replaces them before launch.
- The reports page (`reports-page.md`, backlog 0413) says what each report file is for and who reads it, with a real
  report of one example app; it is linked from this section, and the example lines above can be taken from it.
- The reading prompt (`sv prompts report`) has not yet been tried in an AI tool, hence "can".

## 5. What it checks

**Against the standards security teams use.**

OWASP is a nonprofit that writes the security checklists many companies build to. StackVet uses three of them:

| Standard | What it covers | Requirements | StackVet can confirm or fail |
|---|---|---|---|
| ASVS 5.0, levels 1 and 2 | Web apps and the services behind them | 253 | 122 |
| ASVS 5.0, level 3 | The same, for apps where a break-in would do the most harm | 92 | 10 |
| AISVS 1.0, levels 1 and 2 | Apps that use AI | 146 | 26 |
| AISVS 1.0, level 3 | The same, at the highest level | 45 | 1 |
| Secure by Design 0.5 | Decisions made before the code | 36 | built into planning |

StackVet has a check for 61 more of the ASVS requirements and 19 more of the AISVS ones, but those checks can only
show the protection missing: when they find nothing wrong, that doesn't show the protection is there.

Five of the ASVS ones counted above only CodeQL can check, and it runs only when you turn it on, because its license
allows it only for an open-source app, academic research, or a paid GitHub license. Without it, those five stay not
verified.

ASVS and AISVS sort their requirements into three levels. Level 1 is the baseline for any app, and level 2 is for
most apps that hold people's information; StackVet holds each app to one of those two, from what you tell it the app
does and keeps. Level 3, for apps where a break-in would do the most harm, such as banking or medical systems, is
there when you ask for it: StackVet holds your app to level 3 if you set it.

The Secure by Design requirements are decisions, not code, so no check can settle them. Instead, StackVet builds them
into the design process: before your AI tool writes a feature, it gets the decisions to make and the prompts that
help you both make them, and you answer each one yourself.

Everything StackVet can't check is still in your report, marked as not verified, so you can see it and decide what
to do.

It reads code in multiple programming languages: Python, JavaScript, TypeScript, Go, Ruby, PHP, Java, C#, Kotlin,
Rust, C, C++, Dart, Swift, and shell scripts. If your app uses a language it can't read, the report says so, rather
than guessing.

- The counts are `docs/COVERAGE.md`'s "Can be credited", read from that file when the site is built
  (`tools/website.py`): the levels 1 and 2 rows add its "ASVS 5.0 by level" and "AISVS 1.0 by level" tables' L1 and
  L2 rows, and the level 3 rows are their L3 rows. The "more" sentence is "Can settle" less "Can be credited" on
  every row of the table. The figures written here are those of 22 October 2026; the site uses the file's. The owner
  chose this column on 13 October 2026 (backlog 0433), and the level 3 rows on 22 October 2026, when level 3 had
  become something an app can be held to (ADR-130).
- "Confirm or fail": a requirement counted here can come back *checked* (or *checked in part*) as well as
  *needs attention*. Many can only ever be *checked in part*, since the check tries part of what the requirement
  asks; `docs/COVERAGE.md` gives that count under the level table.
- "If you set it" is `[app] level = 3` in `stackvet.toml` (ADR-130), which only ever raises the level. The level 3
  examples are to be checked against ASVS's own wording before launch.
- A picture for this section: `docs/coverage-by-level.svg` draws the ASVS level table as bars, and
  `docs/coverage-by-chapter.svg` the chapter table, each part labeled with its count. `tools/coverage.py` draws both
  from the same counts as `docs/COVERAGE.md`, so the site can show one beside the table (backlog 0483).
- Secure by Design in planning: `stackvet_plan`, `stackvet_before`, and each library prompt's list of the controls it
  helps answer. None counts as checked in the report.

## 6. What it does with your code

**Your code stays on your computer.**

StackVet runs on your own machine. It doesn't upload your code, and it doesn't send the names of the packages your
app uses anywhere. When it starts your app to test it, the app runs with no internet connection. Your AI coding tool
is different: it can read every file in your app's folder, including any data or keys kept there, and it sends what
it reads to the company that runs it.

There are four exceptions. Three you turn on yourself: a check of your live website, which makes at most a few
requests to the address you type; if you ask for it, downloading your app's packages so it can start; and checking
somebody else's public project on GitHub or GitLab, which downloads its code once, as files, and runs none of it
unless you ask. The fourth is on unless you turn it off: StackVet runs the security tools each language already has, when they are installed on
your computer, and one of them, Semgrep (or Opengrep in its place), downloads its rules from semgrep.dev each time it
checks your app. It checks your code on your computer, with its usage reporting turned off. To check without any of
these tools, add `--no-tools`.

Thinking of using somebody else's code? [Check the project first](check-a-project.html).

- The rule is drawn and tabled in `docs/ARCHITECTURE.md`, "The fence", and is CLAUDE.md's, ADR-027 (`sv probe`), ADR-052 (`install = true`), ADR-154 (a public project's address), and ADR-111 (the outside tools run by
  default since 13 October 2026, the owner's decision, Semgrep's download included). "Usage reporting turned off" is
  the `--metrics=off` `sv` passes Semgrep (`data/adapters.json`); Opengrep has no such option and sends no metrics.

## 7. Get started

**Two ways to get started.**

**New to building with AI?**
Start with the setup prompts. You copy each one into your AI coding tool, and the tool does the setup with you: it
checks what your computer already has, tells you what to install and where to get it, and asks before it installs
or changes anything. If a step doesn't work, it stops and tells you what happened in plain words.

You'll need an AI coding tool that can run commands on your computer, not only a chat window. The setup prompts have
been tried in *[the tools they have really been tried in, filled in from the owner's test]*.

Want to see it first? Follow one real build, screen by screen, from the first decisions to the report.

Buttons: *Open the setup prompts*, *See how a build goes*.

**Used a terminal or Docker before?**
Install Docker Desktop or Colima, fetch StackVet with one command, and connect it to your AI tool; on Windows,
install StackVet with Scoop instead. The step-by-step guide covers every command.

For the checks that start your app, install StackVet on your computer as well:
- **Windows:** install with Scoop, a free installer. Nothing to build.
- **Mac or Linux:** install with Homebrew. It builds StackVet on your computer the first time, which takes a while.

Button: *Read the guide*.

- The walk-through is the build's, *How a build goes* (`build-walkthrough-page.md`, item 4): the owner chose on 22
  October 2026 to start with it, and leave a walk-through of a setup for later. The setup prompt library is item 1,
  done 14 October 2026. The site names only AI tools the prompts have really been tried in.

## 8. Free, open, and new

**Free, open source, and honest about being new.**

StackVet is free and its code is public on GitHub under the MIT license. That means anyone can use it, change it,
and share it, even in their own work, as long as they keep the license with it, and it comes with no guarantee. It's
new: very few people have used it yet, and what is known not to work is listed openly. If something confuses you,
tell us in a GitHub issue.

- Contact is GitHub issues only, for now.

## 9. The research

*Left blank until the paper is finished and published. The owner supplies its title, summary, and citation then.*

## Credits (the footer, and a page of its own)

**The standards and tools StackVet builds on.**

- **OWASP Application Security Verification Standard (ASVS)**, version 5.0.0. OWASP Foundation. CC BY-SA 4.0.
  github.com/OWASP/ASVS
- **OWASP Artificial Intelligence Security Verification Standard (AISVS)**, version 1.0, including Appendix C,
  AI-Assisted Secure Coding. OWASP Foundation. CC BY-SA 4.0. github.com/OWASP/AISVS
- **OWASP Secure by Design Framework**, version 0.5.0 (draft, August 2025). OWASP Foundation. CC BY-SA 4.0.
  github.com/OWASP/www-project-secure-by-design-framework

*Outside tools, run with every report when they are installed (add `--no-tools` to leave them out):*
- **Semgrep**, and **Opengrep** in its place when Semgrep isn't installed, running the Semgrep Registry rules. The
  rules are under the Semgrep Rules License, which allows use but not resale; StackVet is not sold. Semgrep and
  Opengrep themselves are under the GNU LGPL 2.1.
- **CodeQL**, GitHub, for JavaScript and TypeScript, Python, and Ruby, only when you turn it on (`[tools] codeql = true`
  in `stackvet.toml`). The CodeQL command-line tool is under the GitHub CodeQL Terms and Conditions, which allow it
  on an app released under an open-source license, for academic research, or with a paid GitHub Advanced Security
  license; its queries are under the MIT License.
- **Bandit**, for Python, and **gosec**, for Go, both under the Apache License 2.0. **Brakeman**, for Ruby on
  Rails, under the Brakeman Public Use License, which allows use to analyze your own software and not commercial
  use.

*Data:*
- **OSV**, the open vulnerability database, which you download yourself. Each of its sources keeps its own
  license, most of them CC BY 4.0, CC0 1.0, MIT, or Apache 2.0 (the list is on osv.dev).
- **MITRE ATLAS**, for the threats to apps that use AI, in the report's threat list. Apache License 2.0;
  StackVet keeps MITRE's copyright line with the technique names it holds.
- **FIRST's CVSS 4.0 calculator**, whose tables StackVet uses to score vulnerabilities, with its copyright notice
  and license kept.

StackVet is not made by or endorsed by OWASP, GitHub, Semgrep, MITRE, FIRST, or any of these tools' makers.

- garak is left out: the owner decided on 26 September 2026 not to take it up (backlog 0140).
- CC BY-SA 4.0 asks for credit, a link to the license, and a note of changes; the credits page says StackVet holds
  the standards as data and words some of them plainly.
- Each license was read from its source on 14 October 2026 (backlog 0374): the three OWASP standards (CC BY-SA 4.0,
  each repository's own `LICENSE.md`), Semgrep and Opengrep (LGPL 2.1), CodeQL (its command-line tool's
  `LICENSE.md` in github/codeql-cli-binaries, and the MIT License of github/codeql), Bandit and gosec (Apache 2.0),
  Brakeman (`LICENSE.md` in presidentbeef/brakeman), OSV (osv.dev's data page), MITRE ATLAS (Apache 2.0, with no
  NOTICE file), and FIRST's calculator (BSD 2-Clause, as ADR-033 records). StackVet's image installs none of the
  outside tools: the person installs each, and its license governs their own use. The Semgrep Rules License's own
  page could not be read from the session that checked (its network refused semgrep.dev); the owner's reading of
  26 September (ADR-018) stands. Three questions for the owner are backlog 0513.

## Not yet true, and filed

The site does not go live while any of these is open:

1. ~~A setup prompt library for people new to building with AI, tried by the owner, with the tools it works in named.~~
   Done 14 October 2026 (backlog 0370).
2. ~~The report, and the prompt that reads it, lead with the most important things to fix.~~ Done 13 October 2026
   (backlog 0371).
3. ~~Each requirement in the report stated in plain language, beside OWASP's wording, with checks named in words.~~
   Done 14 October 2026 (backlog 0372).
4. ~~A short walk-through of a setup, as a video or a click-through slide show.~~ Done 22 October 2026 as the
   walk-through of a build, at the owner's word, the setup's left for later (backlog 0373).
5. ~~Every credited tool's and data source's license confirmed from its source.~~ Done 14 October 2026 (backlog 0374).
