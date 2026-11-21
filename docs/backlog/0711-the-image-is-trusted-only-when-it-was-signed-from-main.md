# The image is trusted only when it was signed from main

**Status:** partly done: the owner's choice on taking the workflows permission away from the sessions' App (ADR-158), asked on 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A1). Title and text
kept to the fix until it is built, as the review asked.

- **To build:** the `gh attestation verify` line in `action.yml` adds `--source-ref refs/heads/main`, and every place
  that tells a person how to check the image (the setup prompts in `docs/prompts/`, `docs/INSTALL.md`) does the same.
  A test fails when one of them leaves it out. The done notes of backlog 0289 and 0422 and ADR-108's text that say
  the `publish` Environment alone keeps the image to `main` are corrected by Later entries.
- **The owner's:** whether to take the `workflows` permission away from the sessions' App (ADR-158), after which a
  workflow change is pushed by the owner or with `--workflows` only if the owner gives it back.
