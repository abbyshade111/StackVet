# Notes with users — what the checks found

Produced by `sv` 0.2.0 (commit 7031bd26f038) on 2026-11-21 15:48:12 UTC, run 15a3dae229ac.

**This file:** What the checks found, each in full (where, the code, how sure, why it matters, what to do, and how to set it aside), then what was not examined. **For:** Whoever fixes the code, you or your AI coding tool. The end of `report.html` lists every file in this folder and who each is for.

Nothing shows that sv was used while this app was built: no call from an AI coding tool to sv's MCP server is recorded for it. That is not a finding. The app may have been checked at a terminal, or built before sv kept this record.

report.pdf beside this report is the same report as one file to read or print, on US Letter paper (`report-paper = "a4"` under `[app]` in stackvet.toml makes it A4).

## 2 things to fix

Worst first.

### [medium] A password is stored, or a key made from it, with too few rounds of work

**Where:** `app.py` line 49

*How sure: confirmed.*

*Fingerprint: `v2-9d2dd4b16e6a08fd`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A password is hashed for storage, or a key derived from it, with PBKDF2, and the number of rounds written into the code is below what OWASP recommends: 600,000 for SHA-256 and 210,000 for SHA-512 (SHA-1 needs 1,300,000). A count below 210,000 is reported whatever the hash; one below 600,000 where the call itself names SHA-256, and one below 1,300,000 where it names SHA-1.

**The code.**

```text
    48  def hash_password(password, salt):
>   49      return hashlib.pbkdf2_hmac("sha256", password.encode(), salt.encode(), 200_000).hex()
    50  
```

**Why it matters.** Anyone who gets hold of what the key protects, such as an encrypted backup, an export, or a stored hash, can try guesses at the password quickly on ordinary hardware, and most people's passwords fall to a fast guesser.

**What to do.** Raise the number of rounds to at least 600,000 for PBKDF2 with SHA-256 (210,000 with SHA-512, and 1,300,000 with SHA-1, which is better replaced), or use a password hashing function made for this, such as Argon2id or scrypt. Keep the count where it can be raised later without breaking what was already stored.

Evidence about: V11.4.2, V11.4.4

Known as: CWE-916

### 1 low-severity finding, by rule

Each is in full, with where and how to fix it, in `low-findings.md`.

- `config.security-contact` — 1 time, in `SECURITY.md`

## Worth knowing: 1 note

Not things to fix. Each says something about how the app is built that is worth knowing, and asks for nothing unless you decide it matters; if you do, its "What to do" says how.

### [info] 138 commit(s) on the default branch were made by an AI coding tool and not merged in

**Where:** `.git` line 1

*How sure: confirmed.*

*Fingerprint: `4213cb48a86e8e56`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

These commits name an AI coding tool and reached the branch directly: `1fe46225` Run the app's container read-only, without capabilities (#496) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `93bb7454` Claim running the app's container read-only (#493) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `53154bca` Backlog: say when a manifest and its lockfile disagree (#484) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `67a497af` Example app: PyJWT 2.15.1, and a lock that matches its manifest (#482) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `aeb056e1` Backlog: a prompt library from the CSA guide and lessons learned (#476) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `e91c7173` Backlog: record what the sessions agreed about the promotion (#221) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `8ced5fb1` Backlog: v1-paper is tagged and released (#216) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `76156b3c` Backlog: promote sv to the top of the repository, keep v1 for the paper (#210) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `0acfe495` Backlog: what the owner's first build from scratch found, and a zip (#204) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `ce032691` Backlog: packaging sv, a container now and a download later (#198) (Claude Opus 5.5 &lt;noreply@anthropic.com>); and 128 more. This check reads only what a commit says about itself, in its `Assisted-by:` or `Co-Authored-By:` line, so a commit that names no tool is not seen. Whether the hosting service's protection rules held is in its settings, which `sv` does not read.

**Why it matters.** A change that reached the branch without a merge reached it without another person's review.

**What to do.** Ask the AI coding tool to work on a branch and open a pull request, and merge it after a look. Turn on the pull-request rule for the default branch, so a direct push is refused.

## What was not examined

- **the running app** — `sv report` does not start the app unless you pass --run. Without it, nothing here has asked the app anything — what it sends to a browser, what it says when something goes wrong, which sites it accepts.
- **the security tool this language already has** — `sv report` ran none of the outside tools, because --no-tools was given; without it they run. Bandit, gosec, Brakeman, and Semgrep each know their languages far better than the handful of rules built in here.
- **the check `config.gitignore-covers-env`** — This app is a folder inside a larger git repository and has no .gitignore of its own. A .gitignore in a folder above it may leave out .env, but `sv` reads only the app's folder, so this check did not run.
- **the check `config.security-contact-placeholder`** — there is no SECURITY.md or security.txt to read, which config.security-contact reports
- **the check `config.versions-pinned`** — No package manifest was found, so there is nothing whose versions could be pinned. If this app installs dependencies some other way, that is not something `sv` can see.
- **the check `config.development-server-started`** — There is no Dockerfile or Procfile (other than ones named for development), so `sv` cannot see what command starts the app where it is deployed.
- **42 requirements that are design review, not scanning** — these ask how the system was designed and how it is run — whether trust zones are enforced, whether an incident response plan is rehearsed, whether data has named owners. No check here reaches them and none ever will, so they are counted as applicable and unverified, and a person has to answer them.
- **known vulnerabilities in the packages this app ships** — `sv report` compares against known vulnerabilities only when you pass --advisories DIR. `sv` does not fetch anything, because the list of packages an app depends on is yours: download an OSV export for this app's ecosystems, unpack it, and pass its folder with --advisories.
- **18 requirements that ask for a written decision** — No tool can answer these: they ask what your rules are, who may do what, and how long things are kept. Run `sv notes` to write security-notes.md, answer the questions in it, and they become documented. Your AI coding tool can ask you them: `sv questions` prints them for its chat.
- **14 questions about how this app is built** — No tool can settle these — whether input is validated on the server, whether the app's own services authenticate to each other. Answer them in the `[design]` section of stackvet.toml: V2.2.2, V7.2.1, V8.3.1, V15.3.1, V1.1.1, V4.1.3, V4.2.1, V8.2.3, V11.2.2, V14.2.2, V15.3.4, V16.2.2, V16.4.3, V2.3.3. Your AI coding tool can ask you them: `sv questions` prints them for its chat.

