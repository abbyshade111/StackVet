# ConfigVet part 5: drift and the joins between layers, once the files and a live account can both be read

**Status:** open

Filed 22 November 2026, at the owner's word ("file the rest of the items"). Part 5 of ADR-157's build order (the design
entry `docs/design/0396-configvet-a-companion-for-an-app-s-deployment-configuration.md`, "Proposed order of the build"), after part 4's two live providers (0717, GitHub; 0732, Cloudflare).

- **Drift:** a setting in the repository's files that differs from the live account is a finding of its own (ADR-157,
  point 6): the files are what someone intended, and the account is what is running. Neither side is taken as right.
- **The joins between layers** the design entry names ("What ConfigVet looks for"), each a finding only where both
  layers were read: the app trusts `X-Forwarded-For` with nothing in front that sets it; a secret the code reads that
  no layer defines; Cloudflare's SSL mode Flexible while the app's HSTS promises encryption end to end; a Dockerfile
  that drops root under a pod spec that sets `runAsUser: 0` (the last may already be part 2's rule; check first).
- **Before any code:** which pairs of a file setting and a live setting name the same thing, each from both sides'
  documentation, recorded on ADR-157; a pair whose two readings cannot be matched with certainty is not assessed.
- **Check:** each join and each drift pair has a test with both sides given, and one with either side missing, which
  says not assessed, never a pass.
- **Order,** after 0732.
