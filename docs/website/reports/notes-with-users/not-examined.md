# What was not examined: Notes with users

Produced by `sv` 0.3.0 (commit 7cb7878b2ec7) on 2026-11-22 03:24:15 UTC, run 90bae36f9345.

**This file:** What this report could not look at, and why, with the outside tools' runs, from compliance.md. **For:** You, before you read anything as safe. The end of `report.html` lists every file in this folder and who each is for.

This page is one part of the report, cut whole from it: it credits nothing on its own, and the counts stay in `compliance.md`. Like every file in this folder it is sealed with the report, so `sv` can tell whether it is the page it wrote.

## What was not examined

Each of these is a limit on what the rest of this report can mean.

| not examined | why |
|---|---|
| the running app | `sv report` does not start the app unless you pass --run. Without it, nothing here has asked the app anything — what it sends to a browser, what it says when something goes wrong, which sites it accepts. |
| the security tool this language already has | `sv report` ran none of the outside tools, because --no-tools was given; without it they run. Bandit, gosec, Brakeman, and Semgrep each know their languages far better than the handful of rules built in here. |
| the check `config.gitignore-covers-env` | This app is a folder inside a larger git repository and has no .gitignore of its own. A .gitignore in a folder above it may leave out .env, but `sv` reads only the app's folder, so this check did not run. |
| the check `config.security-contact-placeholder` | there is no SECURITY.md or security.txt to read, which config.security-contact reports |
| the check `config.versions-pinned` | No package manifest was found, so there is nothing whose versions could be pinned. If this app installs dependencies some other way, that is not something `sv` can see. |
| the check `config.development-server-started` | There is no Dockerfile or Procfile (other than ones named for development), so `sv` cannot see what command starts the app where it is deployed. |
| 42 requirements that are design review, not scanning | these ask how the system was designed and how it is run — whether trust zones are enforced, whether an incident response plan is rehearsed, whether data has named owners. No check here reaches them and none ever will, so they are counted as applicable and unverified, and a person has to answer them. |
| known vulnerabilities in the packages this app ships | `sv report` compares against known vulnerabilities only when you pass --advisories DIR. `sv` does not fetch anything, because the list of packages an app depends on is yours: download an OSV export for this app's ecosystems, unpack it, and pass its folder with --advisories. |
| 18 requirements that ask for a written decision | No tool can answer these: they ask what your rules are, who may do what, and how long things are kept. Run `sv notes` to write security-notes.md, answer the questions in it, and they become documented. Your AI coding tool can ask you them: `sv questions` prints them for its chat. |
| 14 questions about how this app is built | No tool can settle these — whether input is validated on the server, whether the app's own services authenticate to each other. Answer them in the `[design]` section of stackvet.toml: V2.2.2, V7.2.1, V8.3.1, V15.3.1, V1.1.1, V4.1.3, V4.2.1, V8.2.3, V11.2.2, V14.2.2, V15.3.4, V16.2.2, V16.4.3, V2.3.3. Your AI coding tool can ask you them: `sv questions` prints them for its chat. |

## Outside tools

None ran this time: Bandit, gosec, Brakeman, Semgrep. They run with every `sv report` at a terminal unless `--no-tools` is given; the MCP server and the GitHub Action never run them.

