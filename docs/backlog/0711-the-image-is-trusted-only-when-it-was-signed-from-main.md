
**Done, 21 November 2026:** `action.yml`, the README, and the publish job's comment require `--source-ref
refs/heads/main`; `crates/sv-cli/tests/image_signed_from_main.rs` holds every image signature check to it; ADR-051 and
ADR-081, Later; 0289's done note corrected. **The owner's choice, 21 November 2026:** the sessions' App keeps the
`workflows` permission, since the signature check, not the permission, is what keeps a branch's image from passing
(ADR-158, Later).
