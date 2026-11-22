# cv check reads nginx, Caddy, and the hosting platforms' own files

**Status:** open

Filed 22 November 2026, at the owner's word. The design entry (`docs/design/0396-configvet-a-companion-for-an-app-s-deployment-configuration.md`, the table of what is shared and what is ConfigVet's)
lists parsers for nginx, Caddy, and the hosting platforms' files beside Dockerfile, compose, Kubernetes, and Terraform
plans; `crates/cv-rules` reads only the last four today.

- **Before any code:** which files first, asked of the owner with a recommendation (nginx and Caddy are where a proxy's
  TLS, headers, and forwarded-address settings live; a platform file such as `vercel.json`, `netlify.toml`,
  `fly.toml`, or `render.yaml` may already be read by one of `sv`'s `config.*` checks, which stays `sv`'s:
  one rule has one home), and each setting's default only from the vendor's own documentation (ADR-157, point 3).
- **Check:** each new reader has its parser tests, and a setting left out is a finding only where the default is known
  and unsafe; otherwise not assessed.
- **Records:** ADR-157, a Later entry, and `data/config-defaults.json` with each default's source.
