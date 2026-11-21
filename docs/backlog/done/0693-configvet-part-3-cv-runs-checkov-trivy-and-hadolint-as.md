# ConfigVet part 3: cv runs Checkov, Trivy, and hadolint as outside tools

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word ("let's get started on ConfigVet part 3"). The third step of ADR-157's build
order and of the design entry's "Proposed order of the build": "Outside tools: the Checkov, Trivy, and hadolint
adapters". Parts 1 (`sv-core`, backlog 0686) and 2 (`cv check`, files only, backlog 0687) are done.

**The owner's choices, 21 November 2026, asked with a recommendation for each** (recorded in ADR-157's Later entry of
the same day):

- **Trivy's rules are the ones built into its program, offline,** never downloaded on a run, so `cv`'s fence stays as
  it is and a report names the exact Trivy version whose rules ran.
- **The tools are tried for real, installed with Homebrew** as someone using `cv` would (hadolint 2.15.1 and Checkov
  3.4.6 on the owner's Mac, beside Trivy 0.74.0), the way every `sv` adapter was tried (ADR-018).
- **A tool's finding that one of `cv`'s own rules also made is one finding,** `cv`'s, naming the tools that agree, as
  `sv` merges findings at one place.

**What the build holds `cv` to, as `sv`'s adapters are held** (`data/adapters.json`, ADR-018, ADR-111):

- **SARIF only,** one parser; a tool not installed is reported as not run with how to install it, never a clean pass.
- **The app cannot switch a tool's rules off unseen:** each tool's settings file in the app (`.checkov.yaml`,
  `.hadolint.yaml`, `trivy.yaml`, `.trivyignore`) and its comments that skip a rule are tried, and either kept out of
  the run or reported as what they hide, as `sv` does for Bandit's `# nosec` and Brakeman's settings file.
- **No network:** each tool is run with what it fetches turned off, and that is measured, not assumed (Checkov's
  version check and platform features, Trivy's check bundle).
- **Rule ids mapped to requirements one at a time, after reading the requirement;** a rule not mapped is a finding
  carrying no requirement rather than a wrong one. Not every rule of Checkov's is mapped in this item.
- **They run with every `cv check` unless `--no-tools`,** as `sv`'s do with every report (ADR-111).
- **Not in this item:** kube-linter (the design entry's table names it; its build order does not), and the report
  pages, which wait as part 2 said.

**Done, 21 November 2026** (ADR-157, "As built", part 3; ADR-018, Later). `cv check` runs hadolint 2.15.1, Checkov
3.4.6, and Trivy 0.74.0 through `sv`'s own runner (`cv-cli` now depends on `sv-check`, the owner's fourth choice),
described in `data/cv-adapters.json`. Each was measured in the sandbox and over a planted app: run as they come, Trivy
connected to `check.trivy.dev` and Checkov to `api0.prismacloud.io`, and the app's `trivy.yaml`, `.trivyignore`,
`.hadolint.yaml`, `# hadolint ignore=` comments, and `.checkov.yaml` each hid a rule (Trivy's `trivy.yaml`, every
rule). Each is now run offline and with `cv`'s own settings file, and Checkov, which reads the app's file whatever it is
told, is "ran in part" while the file is there. 23 rules are mapped after reading ASVS 5.0; only a secret in a
Dockerfile's `ARG` or `ENV` speaks to a requirement (against V13.3.1, never crediting it). A root container the tools
report is folded into `cv.container-root`'s finding. Witnessed in `crates/cv-cli/tests/tools.rs`: five protections,
each removed in turn, turned a test red. Not done: kube-linter, the report pages, and Terraform plans tried through the
tools.
