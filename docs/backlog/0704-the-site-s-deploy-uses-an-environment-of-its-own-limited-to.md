# The site's deploy uses an Environment of its own, limited to main, with the owner as reviewer

**Status:** claimed by Claude Code session (site review), 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A3). The site's deploy
job read the Cloudflare keys from the repository's secrets, which every workflow can reach, and named no Environment.

- **Done by the owner and a session, 21 November 2026:** an Environment named `site`, which only `main` can deploy
  to, with the owner as its required reviewer (made through the owner's `gh`, at the owner's word). The owner puts
  `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` in it.
- **To build:** the deploy job in `.github/workflows/site.yml` names `environment: site`;
  `crates/sv-cli/tests/site_workflow.rs` fails if it does not; ADR-132 gains a Later entry.
- **Then, on the owner's word, after a deploy through `site` works:** the two repository-wide Cloudflare secrets are
  deleted, since until then the Environment narrows nothing.
- **Not here:** the same for Docker Hub's token, which the review also raised; it is the owner's choice.
