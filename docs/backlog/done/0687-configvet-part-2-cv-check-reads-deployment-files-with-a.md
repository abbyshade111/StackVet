# ConfigVet part 2: cv check reads deployment files, with a defaults table for Express and Django (ADR-157)

**Status:** done, 20 November 2026

Asked for by the owner on 20 November 2026. The second step of the build order in the design entry "ConfigVet, a
companion for an app's deployment configuration (20 November 2026)", under ADR-157 (accepted). Files only: no network
connection of any kind.

- **The owner's choices, 20 November 2026,** asked before any code was written:
  - **Crates:** `crates/cv-rules` (the parsers, the five outcomes, the defaults table, and the rules, over `sv-core`,
    `sv-scan`, and `sv-frameworks`) and `crates/cv-cli` (the `cv` binary alone). The live clients come later in a
    crate of their own.
  - **YAML:** `sv-check`'s reader (`crates/sv-check/src/yaml.rs`) moves word for word into a small crate of its own,
    `crates/sv-yaml`, re-exported by `sv-check` under its old path, so both binaries read YAML one way, no outside
    dependency is added, and `sv-core` stays free of tree-sitter.
  - **First rules:** Express's `NODE_ENV`, Django's `SESSION_COOKIE_SECURE` and `CSRF_COOKIE_SECURE`, and a container
    running as root.
  - **Output:** a listing on the terminal and a `report.json` of findings and credits. Report pages, SARIF, the seal,
    and the OSCAL export wait for a later part.
- **Parsers:** Dockerfile (the final stage's `ENV` and `USER`), compose (`environment`, `user`), Kubernetes manifests
  (container `env`, `securityContext`), and Terraform plan JSON (`planned_values`, a container definition's
  `environment`). Each value keeps its file and line.
- **The five outcomes** for a setting: set safe, set unsafe, missing with a safe default, missing with an unsafe
  default, and unknown, which is *not assessed*, never a pass.
- **The defaults table** as data, `data/config-defaults.json`, read through `sv_frameworks::data`: each default names
  its framework or platform, the version range it holds for, and the vendor's own page it was read from. A default
  the vendor does not document is left out, and a setting resting on it is not assessed (Kubernetes' `runAsUser`, whose
  default the reference could not be read for, is one).
- **One rule, one home:** `sv`'s launch check already flags `NODE_ENV=development` written in a Dockerfile; that stays
  in `sv`. `cv` reads the layers `sv` does not.
- **Every citation after reading its text** in `data/frameworks/asvs-5.0.0.json`; each new check has a witness that
  goes red when the check is disabled.
- **Done when:** `cv check <folder>` runs on the examples, `cargo fmt --all --check`, `cargo clippy --all-targets --
  -D warnings`, and `cargo test --workspace` pass, ADR-157's "As built" gains part 2, and the build pull request is
  open for the owner to see.

**Done, 20 November 2026.** `cv check <folder>` reads Dockerfiles, compose files, Kubernetes manifests, and Terraform
plans as JSON, settles each setting as one of the five outcomes, prints the list, and writes `report.json`; no network
connection is opened. Built as the owner chose: `crates/cv-rules`, `crates/cv-cli`, and `sv-check`'s YAML reader moved
into `crates/sv-yaml`. Four rules: `cv.express-node-env` (V13.4.2, V16.5.1), `cv.django-session-cookie-secure` and
`cv.django-csrf-cookie-secure` (V3.3.1), and `cv.container-root` (no requirement). `data/config-defaults.json` holds six
defaults, each with its vendor's page; Kubernetes' `runAsUser` default is not among them, since its reference could not
be read for it, so such a pod is not assessed. Each rule turned off sent its witnesses red (six, three, two, and three
tests). ADR-157, "As built", part 2 says the rest.
