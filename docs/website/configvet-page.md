# stackvet.dev: ConfigVet, StackVet's companion for deployment configuration (preview)

The words of the page about ConfigVet (`cv`) for ops and DevSecOps engineers (backlog 0695), as `home-page.md` keeps
the home page's: `tools/website.py` reads them, shows each section's headline in bold as its heading, and leaves out
each section's last list, the notes on it. The owner chose on 21 November 2026 one page, labeled preview, linked from
the home page, read by the owner before it merges. Every claim comes from ADR-157 (its decisions, the owner's answers,
"As built" parts 1 to 3, and its Later entries), the design entry
`docs/design/0396-configvet-a-companion-for-an-app-s-deployment-configuration.md`, `cv`'s usage text in
`crates/cv-cli/src/main.rs`, and each tool's `note` in `data/cv-adapters.json`; the notes say which. ADR-157's
decision 7 lets ConfigVet's pages use the field's own terms; this page still explains the few that are StackVet's
own. **Nothing here is live** until the owner publishes the site.

## The opening

**ConfigVet: a review of how your app is deployed. A preview.**

ConfigVet, `cv`, is StackVet's companion for ops and DevSecOps engineers. StackVet reads an app's code and tests the
running app. ConfigVet reads the layer between them: the files that say how the app is deployed. For each setting
that matters, it says whether the setting is safe, unsafe, or something it could not establish, and it never counts
what it could not establish as a pass.

**This is a preview.** ConfigVet reads files only, has four rules of its own, and writes its results as one
`report.json`, without the report pages StackVet has. Reading live hosting accounts comes later.

```
cv check <folder> [--out <folder>] [--fail-on attention|not-assessed] [--no-tools]
```

- ADR-157, decisions 1 and 7, and "As built" parts 1 to 3 (backlogs 0686, 0687, and 0693, 20 and 21 November 2026).
  The command line is the usage text in `crates/cv-cli/src/main.rs`. "Four rules of its own" are
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

## Files now, live accounts later

**Checking files and reading a live account are kept apart.**

- **`cv check` reads the folder you give it.** Reading a live account is never part of a check: it will be a command
  of its own, which you ask for by name.
- **It never writes over your files.** It writes one file, `report.json`, into `--out`, by default
  `<folder>/cv-report`, and refuses a folder that holds anything else.
- **Its exit codes are StackVet's,** so a pipeline can stop on them: 0 when the run finished; 1 only with
  `--fail-on attention` and a finding; 2 when a deployment file could not be read or the folder holds none, or with
  `--fail-on not-assessed` and anything not assessed; and 3 when `cv` itself failed.

**Live hosting accounts come later, behind rules of their own.** Reading a live account means holding a credential
to production, a first for this project, so the rules were written before any of it is built:

- **Off until you ask for it by name,** one provider at a time, with a command of its own. Never on by default, never
  part of a report run, and never from ConfigVet's MCP server.
- **The token comes from the environment or the provider's own credential helper,** never from an option, since
  options are kept in shell history. It is never written to any file, log, or report.
- **Read only.** A token the provider says can write is refused. Where a provider can't say, the report says so
  before any finding.
- **A fixed list of requests compiled into the program:** HTTPS to each provider's named hosts only, with
  certificate checks, `GET` or a GraphQL query from the list (never a mutation), and limits on how many, how large,
  and how long.
- **Settings, never data:** an environment variable's name, never its value; never logs, database rows, or stored
  files.
- **Every request is listed in the report:** its method, host, path, and response status.

A live reading counts as the provider's own account of its settings: it can show what a setting's value is, never
the behavior the setting is meant to cause. Where the files and the account disagree, that is drift, a finding of its
own: the files are what someone intended, and the account is what is running.

GitHub's repository and organization settings come first, then Cloudflare's: SSL mode, the lowest TLS version
accepted, HSTS, and redirecting to HTTPS, the settings that sit between the visitor and the app.

- ADR-157, decision 5 (the live fence), decision 6 and the owner's answers (credit), and the design entry's
  "Proposed order of the build", step 4 (GitHub first, its fine-grained tokens can be made read-only). Cloudflare
  second is the owner's answer of 20 November 2026; stackvet.dev itself is hosted there, so the first client can be
  tried on an account the project owns. Vercel comes later: its tokens can't be limited to reading, and its API can
  return environment variables' values.
- `cv live <provider>` is the planned command's name in ADR-157; it is not built, so the page does not print it as
  a command. "Before any finding" is decision 5's "where a provider cannot say".
- The page makes no claim that `cv` opens no network connection, though `cv check` opens none on 21 November 2026
  (ADR-157, "As built", part 2): live reading will, and the owner chose on 21 November 2026 not to print a claim that
  goes stale when it lands. "Never part of a check" is decision 5's "never from a report run".

## How it relates to StackVet

**The same engine and the same rules of evidence, in a program of its own.**

ConfigVet is built in StackVet's workspace, on StackVet's engine. The two share what a finding is, how much credit a
piece of evidence earns, how a key is shown (its first four characters and its length, never the key), and the
runner of outside tools. Every rule of evidence holds unchanged: something not assessed is never a pass and never a
failure.

- **One rule has one home.** A check StackVet already makes stays in StackVet. StackVet already reports a
  Dockerfile's final stage setting `NODE_ENV` to `development`; ConfigVet lists that value as set unsafe and leaves
  the finding to StackVet, naming its rule.
- **StackVet's own network fence doesn't change.** StackVet gains no connection from ConfigVet. When the live
  readers are built, they go in a part of the code StackVet's program doesn't include, and a test will fail if it
  ever does.
- **Two programs, installed separately.** `sv` and `cv` will come from one release, with one version number, as two
  packages. Installing StackVet never brings a program that can reach a live account.

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
- **No live accounts, drift, or joins between layers yet,** such as an app that trusts a proxy header with no proxy in
  front of it, or Cloudflare's SSL mode leaving the hop to the origin unencrypted while the app's HSTS header promises
  otherwise. Those need a second source to compare, so they come after the first live provider.
- **Not yet run through the outside tools:** kube-linter, and Terraform plans tried through Checkov and Trivy.

Until these are built, ConfigVet stays a preview, and this page says what each part really does as it lands.

- ADR-157, "As built", part 3, "Not done"; part 2's owner's choice on the report; the design entry's "One app across
  its layers" (the two examples) and "Proposed order of the build", step 5.

## Trying it

**Built from source, for now.**

ConfigVet is not packaged yet. To try it today, build it from StackVet's repository with Rust's `cargo`:

```
cargo build --release -p cv-cli
```

The program is `cv`, in `target/release/`. A Homebrew formula and a Scoop manifest for `cv` are coming with StackVet
v0.3.0, beside StackVet's own. Install hadolint, Checkov, and Trivy yourself if you want ConfigVet to run them;
without them, it says they did not run.

If something is wrong or confusing, tell us in a GitHub issue.

- The owner's answer on the release (ADR-157): a `cv` formula in `homebrew-stackvet` and a manifest in
  `scoop-stack-vet`, no new repositories. Being prepared in backlog 0694 to ship with v0.3.0, which is not tagged on
  21 November 2026; no install command is printed until it is. Contact is GitHub issues only, as on the home page.
