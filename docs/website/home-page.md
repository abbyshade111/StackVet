# stackvet.dev: what the home page says

The words of the home page, settled by the owner section by section on 12 October 2026 (backlog 0224). They come
before any design: the mock-ups use them as they stand. **Nothing here is live**, and the site does not go live while
any line below marked *not yet true* is still not true of StackVet (the items are listed at the end). stackvet.dev is
the site; stackvet.app redirects to it.

Every fact here is checked against `main` and the guide. Where a number comes from a generated file, the site reads
it from that file when it is built rather than keeping its own copy.

## 1. The opening

**Vibe-code your app with security in mind from the first prompt.**

Connect StackVet to your AI coding tool before you start. As you describe your app, it tells your tool which OWASP
security rules apply, checks the code after each feature, and has your tool ask you the questions only you can
answer. Once your app works, StackVet writes a report: the most important things to fix first, then what was
checked, and what nobody has looked at yet.

Buttons: *Get started*, *See how a build goes* (a walk-through of one build from the first prompt; *not yet made*).

- "Security in mind" says how the app is built, not what it ends up being: StackVet never says an app is secure.
- The order of the last sentence, the most important things to fix first, is *not yet true* of the report (item 2
  below).

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
- Step 3's order is *not yet true* of the report (item 2).

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

- "Starts with the most important things to fix" and "every requirement in plain language" are *not yet true*
  (items 2 and 3). Today each requirement is shown in OWASP's formal wording, with checks named by internal ids.
- The example lines are invented and say so; a real report from one of the example apps replaces them before launch.
- The reading prompt (`sv prompts report`) has not yet been tried in an AI tool, hence "can".

## 5. What it checks

**Against the standards security teams use.**

OWASP is a nonprofit that writes the security checklists many companies build to. StackVet uses three of them:

| Standard | What it covers | Requirements | StackVet can check |
|---|---|---|---|
| ASVS 5.0, levels 1 and 2 | Web apps and the services behind them | 253 | 153 |
| AISVS 1.0 | Apps that use AI | 191 | 46 |
| Secure by Design 0.5 | Decisions made before the code | 36 | built into planning |

ASVS sorts its requirements into three levels. Level 1 is the baseline for any app, and level 2 is for most apps
that hold people's information; StackVet holds each app to one of those two, from what you tell it the app does and
keeps. Level 3, for apps where a break-in would do the most harm, such as banking or medical systems, has been left
out so far, so it isn't counted here.

The Secure by Design requirements are decisions, not code, so no check can settle them. Instead, StackVet builds them
into the design process: before your AI tool writes a feature, it gets the decisions to make and the prompts that
help you both make them, and you answer each one yourself.

Everything StackVet can't check is still in your report, marked as not verified, so you can see it and decide what
to do.

It reads code in multiple programming languages: Python, JavaScript, TypeScript, Go, Ruby, PHP, Java, C#, Kotlin,
Rust, C, C++, Dart, Swift, and shell scripts. If your app uses a language it can't read, the report says so, rather
than guessing.

- The counts are `docs/COVERAGE.md`'s "can settle" on 12 October 2026 (ASVS level 1: 58 of 70; level 2: 95 of 183),
  read from that file when the site is built.
- "Left out so far" is `stackvet.toml`'s own wording: level 3 is one `sv` "does not yet hold any app to". The level
  3 examples are to be checked against ASVS's own wording before launch.
- Secure by Design in planning: `stackvet_plan`, `stackvet_before`, and each library prompt's list of the controls it
  helps answer. None counts as checked in the report.

## 6. What it does with your code

**Your code stays on your computer.**

StackVet runs on your own machine. It doesn't upload your code, and it doesn't send the names of the packages your
app uses anywhere. When it starts your app to test it, the app runs with no internet connection. Your AI coding tool
is different: it can read every file in your app's folder, including any data or keys kept there, and it sends what
it reads to the company that runs it.

There are two exceptions, and you turn each one on yourself: a check of your live website, which makes at most a few
requests to the address you type; and, if you ask for it, downloading your app's packages so it can start.

- The rule is CLAUDE.md's, ADR-027 (`sv probe`), and ADR-052 (`install = true`).

## 7. Get started

**Two ways to get started.**

**New to building with AI?**
Start with the setup prompts. You copy each one into your AI coding tool, and the tool does the setup with you: it
checks what your computer already has, tells you what to install and where to get it, and asks before it installs
or changes anything. If a step doesn't work, it stops and tells you what happened in plain words.

You'll need an AI coding tool that can run commands on your computer, not only a chat window. The setup prompts have
been tried in *[the tools they have really been tried in, filled in from the owner's test]*.

Want to see it first? Watch a two-minute walk-through of a setup from start to finish.

Buttons: *Open the setup prompts*, *Watch the walk-through*.

**Used a terminal or Docker before?**
Install Docker Desktop or Colima, fetch StackVet with one command, and connect it to your AI tool; on Windows,
install StackVet with Scoop instead. The step-by-step guide covers every command.

For the checks that start your app, install StackVet on your computer as well:
- **Windows:** install with Scoop, a free installer. Nothing to build.
- **Mac or Linux:** install with Homebrew. It builds StackVet on your computer the first time, which takes a while.

Button: *Read the guide*.

- The setup prompt library and the walk-through are *not yet made* (items 1 and 4). The site names only AI tools
  the prompts have really been tried in.

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

*Outside tools, used only when you turn them on:*
- **Semgrep**, and **Opengrep** in its place when Semgrep isn't installed, running the Semgrep Registry rules. The
  rules are under the Semgrep Rules License, which allows use but not resale; StackVet is not sold.
- **CodeQL**, GitHub, for JavaScript and TypeScript, Python, and Ruby.
- **Bandit**, for Python. **Brakeman**, for Ruby on Rails. **gosec**, for Go.

*Data:*
- **OSV**, the open vulnerability database, which you download yourself.
- **MITRE ATLAS**, for the threats to apps that use AI, in the report's threat list.
- **FIRST's CVSS 4.0 calculator**, whose tables StackVet uses to score vulnerabilities, with its copyright notice
  and license kept.

StackVet is not made by or endorsed by OWASP, GitHub, Semgrep, MITRE, FIRST, or any of these tools' makers.

- garak is left out: the owner decided on 26 September 2026 not to take it up (backlog 0140).
- CC BY-SA 4.0 asks for credit, a link to the license, and a note of changes; the credits page says StackVet holds
  the standards as data and words some of them plainly.
- Each license is confirmed from its source before launch (item 5); CodeQL's and Brakeman's first.

## Not yet true, and filed

The site does not go live while any of these is open:

1. A setup prompt library for people new to building with AI, tried by the owner, with the tools it works in named.
2. The report, and the prompt that reads it, lead with the most important things to fix.
3. Each requirement in the report stated in plain language, beside OWASP's wording, with checks named in words.
4. A short walk-through of a setup, as a video or a click-through slide show.
5. Every credited tool's and data source's license confirmed from its source.
