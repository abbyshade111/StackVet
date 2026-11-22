# The ConfigVet page says `cv live github` is here, and what it reads

**Status:** claimed by Claude Code session (packaging and ConfigVet), 21 November 2026

Filed 21 November 2026, at the owner's word. `cv live github` merged in #2509 (backlog 0717; ADR-157, "As built, part
4"), and `docs/website/configvet-page.md` still says reading live hosting accounts "comes later" and heads a section
"Files now, live accounts later".

- **To build:** the page says, from ADR-157's "As built, part 4" and its Later entry of 21 November and from
  `docs/INSTALL.md`'s section on it, and only from those: what `cv live github` reads; that it is off until typed;
  that every request is a `GET` to `api.github.com`, listed in its report; how the token is handed over and why a
  fine-grained one cannot be confirmed read-only; and what a setting counts as. Cloudflare and drift stay "later".
  The page's sources list gains the part 4 entries. Its install lines stay `docs/INSTALL.md`'s (backlog 0701).
- **Done when:** the built page says it, `crates/sv-cli/tests/website.rs` passes, and the owner has read it before the
  site is deployed.
