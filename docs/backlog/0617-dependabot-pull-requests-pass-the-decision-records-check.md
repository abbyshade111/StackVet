# Dependabot pull requests pass the Decision records check when they only move pins (20 October 2026)

**Status:** claimed by claude/dependabot-ruleset, 20 October 2026

Every Dependabot pull request open on 20 October 2026 (#2139 to #2145) failed the required "Decision records" check,
and each earlier one needed a session to write its `ADR-0NN: unchanged, because ...` lines by hand. Most of them only
move a pin: an action's commit fingerprint and version comment, a base image's tag and digest, a requirement's version
and hashes. The owner's choices of 20 October 2026, asked in a session:

1. **A pin-only Dependabot pull request owes no line.** `tools/adr_check.py` treats a governed record as unchanged when
   the pull request is Dependabot's and every line it adds or removes is a pin; any other line, a `Cargo.toml`
   requirement included, still asks. The diff is what is trusted, not the author. The record is a new ADR, written
   as `Status: proposed` here and accepted in the pull request that builds it.
   **Part status:** open
2. **The fuzz lock moves with the workspace's.** `.github/dependabot.yml`'s cargo entry covers `/fuzz` as well as `/`,
   so `fuzz_lock` stops failing a bump (#2141).
   **Part status:** open
3. **The browser test does not name a version.** `the_browser_version_is_still_read_from_its_tag_and_not_its_digest`
   asserts the tag is a dotted version, so a new browser tag (#2140) needs no edit to it.
   **Part status:** open
4. **Node's major stays the owner's.** The ignore names `library/node` as well as `node`, the name Dependabot now uses
   (#2139 offered Node 25 past it).
   **Part status:** open
