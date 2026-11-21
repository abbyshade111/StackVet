# ConfigVet, a companion for an app's deployment configuration (20 November 2026)

A first design for ConfigVet, a companion to `sv` that reviews how an app is configured where it runs, and flags
settings that are wrong and settings that are missing. Written at the owner's asking on 20 November 2026; nothing here
is built. The decision it rests on is ADR-157 (proposed). The owner's three answers so far:

- **One engine, its own command.** ConfigVet lives in this workspace, on `sv`'s engine, and ships as a binary of its
  own, `cv`.
- **Its people are ops and DevSecOps engineers**, not the non-programmers `sv` is written for.
- **It reads live hosting accounts through their APIs**, not only files.

## What `sv` already reads, so ConfigVet does not read it twice

`sv` is not blind to configuration. On 20 November 2026, `crates/sv-check/src/config.rs` and its siblings hold
about forty `config.*` checks, and `docs/COVERAGE.md` counts 12 of ASVS V13's 21 requirements as ones a check can
settle. Among them:

- **The repository:** a secrets file committed, now or in history; `.gitignore` covering `.env`; pinned versions; a
  security contact.
- **GitHub Actions:** token permissions, unpinned actions, fork code run with secrets, untrusted text in `run:`, the
  checkout keeping its token.
- **Platforms read as files:** Firebase rules left open, Supabase tables without row-level security, MCP servers
  unpinned or unencrypted, a TURN relay with no limits.
- **Framework settings in code:** debug mode on (`ast.debug-mode-on`), a development server started, certificate
  checks turned off.
- **The running app** (V13.4): source control, directory listings, TRACE, documentation, and version details exposed,
  each asked by `sv run` as an anonymous visitor.

So ConfigVet's ground is the layer between the code and the running app: **the files that say how the app is
deployed, and the accounts it is deployed into.** One rule has one home. A check already in `sv` stays in `sv`, and
ConfigVet runs it through the shared engine rather than writing a second one.

## What ConfigVet adds

### 1. A setting that is missing, not only one that is wrong

Most configuration linters flag a value that is present and bad: `privileged: true`, `DEBUG = True`. Fewer can say
that nothing sets the session cookie's `Secure` flag, and that this framework's default leaves it off. That needs two
things `sv` does not have yet:

- **An effective value for each setting, with where it came from:** set in a named file and line, set in a live
  account, a framework's or platform's default, or unknown.
- **A table of defaults as data** (proposed: `data/config-defaults.json`). Each entry names the framework or platform,
  the version range it holds for, the default, and the page of the vendor's own documentation it was read from. A
  default is a claim, held as a citation is: written after reading the source, and checked again when a new major
  version comes out.

Each setting a requirement depends on then lands in one of five outcomes:

| Outcome | Meaning | In the report |
|---|---|---|
| Set, safe | Set explicitly, to a safe value | Credit, at the tier of the source that set it |
| Set, unsafe | Set explicitly, to an unsafe value | A finding |
| Missing, default safe | Not set; the default for this version is safe | Credit in part, with a note that an upgrade could change it |
| Missing, default unsafe | Not set; the default for this version is unsafe | A finding: the main new kind |
| Unknown | The version is unknown, the default is not in the table, or the layer was not read | Not assessed, with what would settle it |

The last row is `sv`'s first rule carried over unchanged: a setting ConfigVet could not establish is never a pass.

### 2. One app across its layers

An ops engineer sees one layer at a time: the Dockerfile in one tool, Terraform in another, the CDN's dashboard in a
third. ConfigVet's view is one app across all of them, and its most useful findings sit at the joins between layers.
Examples it should catch:

- The app trusts `X-Forwarded-For` (`trust proxy`), but nothing in front of it is a proxy that sets the header, so any
  client can choose its own address.
- The code reads `SESSION_SECRET`, and neither the Kubernetes secret, the compose file, nor the hosting account
  defines it, so the app falls back to a default or crashes.
- Cloudflare's SSL mode is *Flexible*, so traffic from the CDN to the origin travels unencrypted, while the app's HSTS
  header promises the browser it is encrypted end to end.
- The Dockerfile drops root, but the Kubernetes pod spec sets `runAsUser: 0` and overrides it.
- A setting in the repository's files differs from the live account. That is **drift**, a finding of its own: the
  files are what someone intended, and the account is what is running.

### 3. Live accounts, read and never written

Live accounts are where an ops engineer's real configuration is, and also the riskiest thing ConfigVet can touch. It
holds the keys to production. ADR-157 holds the rules; in short:

- **Off until asked for by name** (`cv live <provider>`), with the token taken from an environment variable or the
  provider's own credential helper. Never from an option, because options land in shell history, and never written
  to any file ConfigVet makes.
- **Read only.** Where the provider can tell ConfigVet a token's scopes, a token that can write is refused. Where it
  cannot, the report says so in its first lines, and the guide says how to make a read-only member or role.
- **Requests from a fixed list compiled into the binary:** HTTPS to each provider's named hosts only, `GET` (or a
  GraphQL query from the list, never a mutation), with limits on count, size, and time.
- **Settings, never data.** ConfigVet asks for an environment variable's name, never its value; never logs,
  database rows, or storage contents.
- **Every request listed in the report:** method, host, path, and status, so a person can see exactly what was asked.
- **`sv` cannot do any of this.** The provider clients sit in a crate `sv-cli` does not depend on, and a test fails
  if `sv`'s dependency tree ever includes it. `sv`'s fence, four exceptions and no more, is unchanged.

## How it is built on the shared engine

| Part | Reused from `sv` | New for ConfigVet |
|---|---|---|
| The standards and what applies | `sv-frameworks`, `data/frameworks/`, the applicability rules | A few conditions (runs in a container, behind a CDN, on Kubernetes) |
| Reading the folder | `sv-scan`'s one walk (`files::Listing`) | Parsers for Dockerfile, compose, Kubernetes YAML, Terraform plan JSON, nginx, Caddy, and the hosting platforms' files |
| Checks | `Finding`, `Verified`, `Tier`, `Secret::redact`; `sv`'s `config.*` checks run as they are | `cv-rules`: missing settings, the joins between layers, drift |
| Outside tools | `data/adapters.json` and its container runner (ADR-032) | Adapters for Checkov, Trivy, hadolint, and kube-linter, each mapped to requirements after reading them |
| Live accounts | None (that is the point) | `cv-live`: one client per provider, behind ADR-157's rules |
| Reports | `sv-report`'s `status_of`, the nine statuses, SARIF, `report.json`, the seal, the OSCAL export (ADR-156) | ConfigVet's own pages, and the list of requests made |
| The command and MCP | `sv-cli`'s exit codes and the MCP server's rules | `cv-cli`, a binary of its own; its MCP server never reads a live account, as `sv`'s never starts the app |

One thing to settle before the build: `Finding` and its kin live in `sv-check`, which is 93,500 lines. ConfigVet
depending on all of it for a handful of types is the wrong way round. The build's first step should move the shared
types into a small crate (`sv-core`) that both depend on. That is the layering fix backlog item 4 of 8 October already
asked for between `sv-run` and `sv-check`.

## Which requirements it speaks to

The home is ASVS 5.0's **V13 Configuration**. Its 21 requirements were read on 20 November 2026 from
`data/frameworks/asvs-5.0.0.json`. The ones `sv` cannot settle today and ConfigVet plausibly could:

- **V13.2.1, V13.2.2** (service-to-service authentication, least privilege): Kubernetes service accounts and RBAC, and
  cloud IAM roles read from the account.
- **V13.2.5** (the server's allowlist of where it may send requests): Kubernetes network policies, security groups,
  egress rules.
- **V13.3.1, V13.3.2** (a secrets manager, and least-privilege access to it): whether secrets come from a vault or a
  plain environment variable, and who can read them.
- **V13.3.4** (secrets expire and rotate): rotation settings read from a vault or a cloud account.
- **V13.4.2** (debug off in production): the deployed environment's value, not the code's.

Each line above is a candidate, not a citation. A citation is a claim: each will be read against the rule that would
make it before it is written into code. Outside V13, likely candidates are V12 (TLS between the layers) and V3.4 (the
headers set by a proxy or CDN rather than by the app). Through ADR-156's crosswalk the same results reach NIST SP
800-53 (CM-6 and CM-7 among them) with no new work, which matters to a DevSecOps reader with an authorization to keep.

**CIS Benchmarks** are the other standard an ops reader will ask for. Their licence limits reuse, as Semgrep's rules
licence does (ADR-111), so the licence should be read before any is cited or shipped.

## What changes for an ops reader

`sv` writes for people who are not programmers, and the project's plain-language rule says so. ConfigVet's readers
are ops engineers, and writing "the system that decides which programs may talk to each other" for "network policy"
would slow them down. Proposed: ConfigVet's pages use the field's own terms and link each one to the glossary the
first time it appears. Everything else in the working style holds: say what was verified and what was not, and report
results as they are.

## Where ConfigVet should not compete

AWS, Azure, and Google Cloud accounts are already well served by Prowler, ScoutSuite, and Steampipe, and IaC files by
Checkov, Trivy, and KICS. Writing another AWS scanner would be a long road to a worse copy. ConfigVet should run those
tools as outside tools, map what they find to requirements, and spend its own rules where nobody else does: missing
settings, the joins between layers, drift, honest gaps, and a report that ties all of it to one app. Its own live
clients start with providers whose APIs are small and whose tooling is thin.

## Proposed order of the build

1. **`sv-core`**: the shared types moved out of `sv-check`, with no change in behavior. `sv`'s tests are the proof.
2. **Files only**: `cv check <folder>` for Dockerfile, compose, Kubernetes manifests, and Terraform plan JSON, plus the
   defaults table for two frameworks (Express and Django). No network.
3. **Outside tools**: the Checkov, Trivy, and hadolint adapters.
4. **One live provider**, behind every rule in ADR-157. GitHub's repository and organization settings are the
   strongest first candidate: fine-grained tokens can be made read-only and ConfigVet can check that, and the project
   already knows the API.
5. **Drift and the joins between layers**, once there are two sources to compare.

## The owner's answers (20 November 2026)

Four questions were put to the owner. The owner answered two, asked for a recommendation on the other two, and accepted both the same day.
ADR-157 records all four:

1. **The binary is `cv`.**
2. **The first live provider after GitHub: Cloudflare,** recommended and accepted. Its settings sit
   between the visitor and the app, which is where the joins between layers are found (the *Flexible* SSL example
   above), and its tokens are built from named permissions, so one can be made read-only. And the project's own
   website, stackvet.dev, is hosted there, so the first client can be tried on an account the project owns. Vercel comes later: its
   tokens cannot be limited to reading, and its API can hand back environment variables' values.
3. **A live reading is *checked* for a setting's value only,** never for the behavior the setting causes. That
   stays `sv probe`'s and `sv run`'s to ask. Agreed as recommended.
4. **The release: one release, two packages,** recommended and accepted. Both binaries would come from
   one commit, with one version and one signed release, and each would have its own formula in `homebrew-stackvet`
   and its own manifest in `scoop-stack-vet`. Installing `sv` would never bring a binary that can reach a live
   account.
