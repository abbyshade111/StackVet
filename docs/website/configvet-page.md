# stackvet.dev: ConfigVet, StackVet's companion for deployment configuration (preview)

The words of the page about ConfigVet (`cv`) for ops and DevSecOps engineers (backlog 0695), as `home-page.md` keeps
the home page's: `tools/website.py` reads them, shows each section's headline in bold as its heading, and leaves out
each section's last list, the notes on it. The owner chose on 21 November 2026 one page, labeled preview, linked from
the home page, read by the owner before it merges. Every claim comes from ADR-157 (its decisions, the owner's answers,
"As built" parts 1 to 4, and its Later entries), the design entry
`docs/design/0396-configvet-a-companion-for-an-app-s-deployment-configuration.md`, `cv`'s usage text in
`crates/cv-cli/src/main.rs` and `crates/cv-cli/src/live.rs`, each tool's `note` in `data/cv-adapters.json`, and
`docs/INSTALL.md`'s sections on `cv live github` and `cv live cloudflare`; the notes say which. ADR-157's
decision 7 lets ConfigVet's pages use the field's own terms; this page still explains the few that are StackVet's
own. **Nothing here is live** until the owner publishes the site.

## The opening

**ConfigVet: a review of how your app is deployed. A preview.**

ConfigVet, `cv`, is StackVet's companion for ops and DevSecOps engineers. StackVet reads an app's code and tests the
running app. ConfigVet reads the layer between them: the files that say how the app is deployed. For each setting
that matters, it says whether the setting is safe, unsafe, or something it could not establish, and it never counts
what it could not establish as a pass.

**This is a preview.** ConfigVet has four rules of its own for deployment files, reads one GitHub repository's or
one Cloudflare zone's settings only when you ask it to, and writes its results as one `report.json`, without the
report pages StackVet has.

```
cv check <folder> [--out <folder>] [--fail-on attention|not-assessed] [--no-tools]
cv live github <owner>/<repo> [--out <folder>] [--fail-on attention|not-assessed]
cv live cloudflare <zone> [--out <folder>] [--fail-on attention|not-assessed]
```

- ADR-157, decisions 1 and 7, and "As built" parts 1 to 4 (backlogs 0686, 0687, 0693, 0717, and 0732, 20 to 22
  November 2026). The command lines are the usage text in `crates/cv-cli/src/main.rs`. "Four rules of its own" are
  `cv.express-node-env`, `cv.django-session-cookie-secure`, `cv.django-csrf-cookie-secure`, and `cv.container-root`.
  "Report pages, SARIF, the seal, and the OSCAL export" wait for a later part (part 2's owner's choice).

## What it reads

**Deployment files, read the way they are deployed.**

`cv check` reads the folder's Dockerfiles, compose files, Kubernetes manifests, and Terraform plans (as JSON), with
the app's `package.json` and Django settings. It turns each layer into the units that are deployed:

- **Dockerfiles:** each stage's `ENV` and `USER`, following the chain of `FROM` that a stage's name makes.
- **Compose files:** each service's `image`, `build`, `environment` (in both forms), `env_file`, and `user`.
- **Kubernetes manifests:** each workload's containers, their `env` and `envFrom`, and `runAsUser` and
  `runAsNonRoot`, a container's own setting outranking its pod's.
- **Terraform plans:** ECS container definitions' `environment`, `secrets`, `environmentFiles`, and `user`. ConfigVet
  reads a plan, not the `.tf` files, because a plan holds the values after variables and modules are worked out.
- **The app itself:** `package.json`, for which major version of Express the defaults are for, and a Django app's
  settings module, since that is where Django's settings are.

Its first rules: Express running in development mode, with stack traces in its error answers; Django's session and
CSRF cookies without the `Secure` flag; and a container that runs as root.

- ADR-157, "As built", part 2: the four readers, and why a plan is read. "For which major version of Express" is
  `crates/cv-rules/src/rules.rs`, which reads every `package.json` that depends on Express and leaves the setting not
  assessed when they do not agree on one major version.
- The rules' citations: `cv.express-node-env` V13.4.2 and V16.5.1; the two Django rules V3.3.1, as a credit in part,
  since they do not read the cookie-name prefix the requirement also asks for; `cv.container-root` cites no
  requirement, since none in ASVS 5.0 asks it of a container (the owner's choice). Each written after reading the
  requirement's text.

## The five outcomes

**A setting left out is a finding only when its default is known, and unsafe.**

Most configuration linters flag a value that is present and bad. ConfigVet also says when nothing sets a setting and
the default leaves it unsafe. It knows a default only from a table of defaults, each entry read from the vendor's own
documentation for a named range of versions. Every setting lands in one of five outcomes:

| Outcome | What it means | What the report says |
|---|---|---|
| Set, safe | Set explicitly, to a safe value | Checked in part: one file saying so is not the running system |
| Set, unsafe | Set explicitly, to an unsafe value | A finding |
| Missing, default safe | Not set, and the default for this version is safe | Checked in part |
| Missing, default unsafe | Not set, and the default for this version is unsafe | A finding, at medium confidence, since the deployment may set it somewhere the folder doesn't hold |
| Unknown | The version is unknown, the default is not in the table, or the value comes from somewhere `cv` can't read | Not assessed, saying why |

**Not assessed is never a pass.** A value that comes from a variable, a `valueFrom`, an `env_file`, an `envFrom`, or
an image ConfigVet can't read is unknown, and so is a version outside what the table was read for. A hardening
guide's recommendation is not enough on its own: unless the default is unsafe, reporting it would pass a preference
off as a weakness, and a report that cries wolf teaches its reader to ignore it.

The table holds six defaults today, each read from the vendor's page on 20 November 2026: Express's `env` and its
error handler's stack trace (Express 4 and 5), Django's `SESSION_COOKIE_SECURE` and `CSRF_COOKIE_SECURE` (4.2 to
5.2), the official `node` image's `NODE_ENV` and `USER` (images 22 and 24), and Docker's default user, root.

- ADR-157, decision 3, and "As built", part 2 (`outcome.rs`, `data/config-defaults.json`); the design entry's table
  of five outcomes, as built: "set, safe" is a credit in part, always. "Cries wolf" is ADR-157's rejected alternative,
  "Treat a missing setting as a finding whenever a hardening guide recommends setting it".
- Kubernetes' default for `runAsUser` is not in the table, since its API reference could not be read for it, so a pod
  spec that sets neither field is not assessed. A range open at either end also leaves a setting not assessed.

## The outside tools

**hadolint, Checkov, and Trivy, run offline, and kept from reading the app's own settings.**

`cv check` runs hadolint, Checkov, and Trivy over the same files, where they are installed, unless you add
`--no-tools`. A tool that isn't installed is reported as not run, with how to install it, and never counted as a
pass. ConfigVet was tried with hadolint 2.15.1, Checkov 3.4.6, and Trivy 0.74.0, each installed with Homebrew.

- **Offline, measured, not assumed.** Each tool was run in a sandbox that reports every connection a program tries.
  Run as they come, Trivy connected to check.trivy.dev and Checkov to api0.prismacloud.io. ConfigVet runs them with
  those switched off, and each gave the same results. Trivy uses the configuration rules built into its program, so
  a report names the version whose rules ran. hadolint connected nowhere.
- **The app can't switch a rule off unseen.** An app's `.hadolint.yaml`, `# hadolint ignore=` comments, `trivy.yaml`,
  and `.trivyignore` each hid a rule when tried; Trivy's `trivy.yaml` hid every one. ConfigVet gives hadolint and
  Trivy its own settings files in their place, and every rule came back. Checkov reads an app's `.checkov.yaml`
  whatever it is told, so while that file is there, Checkov's run counts as ran in part, and finding nothing is not
  credited.
- **Handed only what ConfigVet read.** hadolint and Checkov are given ConfigVet's Dockerfiles, Kubernetes manifests,
  and Terraform plans by name, never a file it could not read, since Checkov given a plan it can't parse says nothing
  and exits 0. Trivy takes one target, so it is given the folder; it did not follow a link out of it. Neither Checkov
  nor Trivy reads compose files.
- **One finding where they agree.** When a tool reports a root container that ConfigVet's own rule also found, in the
  same file, the report shows one finding and names the tools that agree.

Each tool's rules are mapped to ASVS 5.0 one at a time, after reading the requirement. Of the 23 mapped so far, only
a secret written into a Dockerfile's `ARG` or `ENV` speaks to a requirement: it is evidence against V13.3.1 and never
credits it, since finding none doesn't show that a vault is used. A rule that isn't mapped is still a finding, with no
requirement attached.

- ADR-157, "As built", part 3, and its Later entry of 21 November 2026 (the owner's choices: built-in rules offline,
  the tools tried for real with Homebrew, one finding where they agree). What each tool needed is its `note` in
  `data/cv-adapters.json`: Trivy's `--skip-check-update`, `--skip-version-check`, and `--disable-telemetry`;
  Checkov's `--skip-download`; hadolint's `-c` and `--disable-ignore-pragma`; Trivy's `--config` and `--ignorefile`;
  Checkov's `switched_off_by`.
- The agreeing rules folded into `cv.container-root`: hadolint DL3002, Checkov CKV_DOCKER_8 and CKV_K8S_23, Trivy
  DS-0002 and KSV-0012 (`also_reported_by`). The secret rules: hadolint DL3064 and Trivy DS-0031. With
  `--fail-on not-assessed`, a tool that did not run, or ran in part, exits 2.

## Files, and live accounts: GitHub and Cloudflare

**Checking files and reading a live account are kept apart.**

- **`cv check` reads the folder you give it.** Reading a live account is never part of a check: it is
  `cv live github` or `cv live cloudflare`, a command of its own, which you ask for by name.
- **It never writes over your files.** It writes one file, `report.json`, into `--out`, by default
  `<folder>/cv-report`, and refuses a folder that holds anything else.
- **Its exit codes are StackVet's,** so a pipeline can stop on them: 0 when the run finished; 1 only with
  `--fail-on attention` and a finding; 2 when a deployment file could not be read or the folder holds none, or with
  `--fail-on not-assessed` and anything not assessed; and 3 when `cv` itself failed.

**Reading a GitHub repository's settings, behind rules of their own.** `cv live github <owner>/<repo>` reads one
repository's settings: the rules on its default branch, whether workflows may approve pull requests, what a
workflow's token may do by default, which actions may run and whether they must be pinned, each environment's
reviewers and deployment branches, and secret scanning. Reading a live account means holding a credential to
production, a first for this project, so the rules were written before any of it was built:

- **Off until you ask for it by name,** one provider at a time. Never on by default, and never part of `cv check`.
- **The token comes from `GH_TOKEN` or `GITHUB_TOKEN`,** never from an option, since options are kept in shell
  history. It goes to `curl` on its input, never on a command line, and is never written to any file, log, or report,
  which show only its first characters and its length.
- **Read only.** A classic token that can write is refused, and nothing more is read. GitHub doesn't let a
  fine-grained token say what it can do, so the report says, before any finding, that ConfigVet could not confirm it
  is read-only. Make it with read-only access to the repository's Administration and Actions, and nothing else.
- **A fixed list of requests compiled into the program:** every one a `GET` to `api.github.com`, over HTTPS with
  certificate checks, a redirect followed only to that host, and at most 40 requests of 1 MB and 20 seconds each.
- **Settings, never data:** no secret's value, no logs, and nothing from the repository's files, which `cv check`
  and StackVet read.
- **Every request is listed in the report:** its method, host, path, and response status.

A live reading counts as GitHub's own account of its settings: a safe setting is *checked in part*, since it shows a
setting's value, never the behavior the setting is meant to cause. What GitHub does not show the token is *not
assessed*, never a pass. Where the files and the account disagree, that will be drift, a finding of its own: the files
are what someone intended, and the account is what is running.

**Reading a Cloudflare zone's settings, behind the same rules.** `cv live cloudflare <zone>` reads the settings that
sit between the visitor and the app: the SSL/TLS mode (the connection from Cloudflare to the app's server), the lowest
TLS version accepted, TLS 1.3, Always Use HTTPS, HSTS, Automatic HTTPS Rewrites, and the cache rules, with ten
requests, every one a `GET` to `api.cloudflare.com`. What differs from GitHub:

- **The token comes from `CLOUDFLARE_API_TOKEN` alone.** A Global API Key and the token `wrangler login` keeps can
  change the whole account, so either is refused before anything is sent, and the token must be active.
- **Read only, as far as Cloudflare lets it be told.** Cloudflare shows a token its own permissions only if it may
  read every API token on the account, which ConfigVet doesn't ask for, so the report says, before any finding, that it
  could not confirm the token is read-only. Make it with Zone Read, Zone Settings Read, and Cache Rules Read, for the
  one zone, and nothing else.
- **A setting's value, said with what it depends on.** The SSL/TLS mode guards only a connection to a server behind
  Cloudflare, which ConfigVet can't see without the DNS records, so its finding says so; Cloudflare's own HSTS being
  off is *not assessed*, since the app may send the header itself. The cache rules are listed, never credited.

It was tried first on stackvet.dev, this site's own zone, with a read-only token: two of its findings were real (TLS
1.0 accepted, and plain HTTP not redirected), both were changed in Cloudflare, and the next reading credited both.
Drift, where the files and the account disagree, comes next.

- ADR-157, decision 5 (the live fence), decision 6 and the owner's answers (credit), and the design entry's
  "Proposed order of the build", step 4 (GitHub first, its fine-grained tokens can be made read-only). Cloudflare
  second is the owner's answer of 20 November 2026; stackvet.dev itself is hosted there, so the first client can be
  tried on an account the project owns. Vercel comes later: its tokens can't be limited to reading, and its API can
  return environment variables' values.
- `cv live cloudflare` is ADR-157's "As built, part 4, Cloudflare" (backlog 0732); its three questions, the owner's
  choice not to ask for API Tokens Read, and what each setting counts as are its Later entry of 22 November 2026. The
  try on stackvet.dev, the two readings it corrected, and the two settings the owner then changed are in "As built";
  the permissions are `docs/INSTALL.md`'s. Reading the DNS records is backlog 0739, the owner's choice first.
- `cv live github` is ADR-157's "As built, part 4" (backlog 0717), and what it reads, what each setting counts as, and
  why a fine-grained token cannot be confirmed read-only are its Later entry of 21 November 2026. "Administration and
  Actions, read-only" is what each endpoint was found to need with a read-only token on this repository the same day;
  `docs/INSTALL.md` says how to make one. "Before any finding" is decision 5's "where a provider cannot say".
- The page makes no claim that `cv` opens no network connection: `cv check` opens none (ADR-157, "As built", part 2),
  and `cv live github` and `cv live cloudflare` open connections to `api.github.com` and `api.cloudflare.com` only
  (part 4), as `docs/ARCHITECTURE.md`, "The fence", says. "Never part of a check" is decision 5's "never from a report run".

## How it relates to StackVet

**The same engine and the same rules of evidence, in a program of its own.**

ConfigVet is built in StackVet's workspace, on StackVet's engine. The two share what a finding is, how much credit a
piece of evidence earns, how a key is shown (its first four characters and its length, never the key), and the
runner of outside tools. Every rule of evidence holds unchanged: something not assessed is never a pass and never a
failure.

- **One rule has one home.** A check StackVet already makes stays in StackVet. StackVet already reports a
  Dockerfile's final stage setting `NODE_ENV` to `development`; ConfigVet lists that value as set unsafe and leaves
  the finding to StackVet, naming its rule.
- **StackVet's own network fence doesn't change.** StackVet gains no connection from ConfigVet. The live readers
  are in a part of the code StackVet's program doesn't include, and a test fails if it ever does.
- **Two programs, installed separately.** `sv` and `cv` come from one release, with one version number, as two
  packages, since v0.3.0. Installing StackVet never brings a program that can reach a live account.

StackVet is written for people who are not programmers. ConfigVet is written for ops and DevSecOps engineers, in the
field's own terms.

- ADR-157, decisions 1 to 4 and 7, "As built", parts 1 and 2 ("One rule, one home": `sv`'s
  `config.development-server-started`), and the owner's answer on the release. The shared crate is `sv-core`
  (`Finding`, `Verified`, `Tier`, `Secret`); the runner is `sv_check::adapters`. `cv` runs none of `sv`'s checks yet.
- `sv`'s counts in `docs/COVERAGE.md` are unchanged by ConfigVet; its own counts are for the part that gives it
  report pages.

## Not there yet

**What the preview doesn't do.**

- **No report pages yet:** the results are on the terminal and in `report.json`. The pages, SARIF, the seal, and the
  OSCAL export come in a later part.
- **No drift or joins between layers yet,** such as an app that trusts a proxy header with no proxy in front of it,
  or Cloudflare's SSL mode leaving the hop to the origin unencrypted while the app's HSTS header promises otherwise.
  With two live providers built, they come next.
- **Not yet tried with the outside tools:** kube-linter isn't run, and no Terraform plan has been tried through
  Checkov or Trivy yet, though Checkov is handed the plans `cv` read, and Trivy the whole folder.

Until these are built, ConfigVet stays a preview, and this page says what each part really does as it lands.

- ADR-157, "As built", part 3, "Not done"; part 2's owner's choice on the report; the design entry's "One app across
  its layers" (the two examples) and "Proposed order of the build", step 5.

## Trying it

**Released with StackVet, as a package of its own.**

`cv` comes from the same release as StackVet, with the same version number, and installs separately. On a Mac or
Linux, Homebrew builds it on your computer:

```
brew install abbyshade111/stackvet/cv
```

On Windows, Scoop installs the signed download from the release:

```
scoop bucket add stackvet https://github.com/abbyshade111/scoop-stack-vet
scoop install stackvet/cv
```

Or build it from StackVet's repository with Rust's `cargo`, and the program is `cv`, in `target/release/`:

```
cargo build --release --locked -p cv-cli
```

Install hadolint, Checkov, and Trivy yourself if you want ConfigVet to run them; without them, it says they did not
run.

If something is wrong or confusing, tell us in a [GitHub issue](https://github.com/abbyshade111/StackVet/issues).

- The owner's answer on the release (ADR-157): a `cv` formula in `homebrew-stackvet` and a manifest in
  `scoop-stack-vet`, no new repositories (backlog 0694). StackVet v0.3.0, published 21 November 2026, is the first
  release with `cv`'s own Windows zip (#2462), signed beside `sv`'s. The bucket's `cv.json` (scoop-stack-vet#3) and
  the tap's release formula (homebrew-stackvet#5) are the packages these lines install; the pull request that wrote
  them (backlog 0696) merges only after both have merged with their install checks passing. `scoop bucket add` is only needed once, and is the same line as for `sv`; PowerShell
  has no command named `cv`, which the release workflow checks. `--locked` is the formula's own. Contact is GitHub
  issues only, as on the home page.
