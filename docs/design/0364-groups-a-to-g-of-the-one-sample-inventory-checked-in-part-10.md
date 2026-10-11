# Groups A to G of the one-sample inventory checked in part (10 October 2026)

Backlog 0006, part 4, the owner's word of 10 October 2026, and ADR-053 (Later, accepted the same day). Built by
session securevibe-e2.

**The problem.** A running check that tries one page, one request, one account, or one action, and then gives a
requirement plain *checked*, claims more than it tried when the requirement asks about every page, request, or
operation it names. ADR-053 gave such a credit the status *checked in part*, which a report shows and does not count
as checked.

**What was built.** Credits whose check tried one sample of what the requirement names are marked in part, in the
groups the owner accepted:

- **A.** One or two anonymous responses, where the requirement says every response.
- **B.** One private page, one sign-in, or one session.
- **C.** One request, one flow, or one WebSocket handshake.
- **D.** One upload route and one file.
- **E.** One log line or event.
- **F.** One browser page or form.
- **G.** One of several operations the requirement names: an email code alone for V6.5.1 and V6.5.5, a TOTP alone for
  V6.5.1 and V6.5.5, one flow the manifest names, and so on.

Group H (the private and admin pages the owner lists) is marked in part only when one page is listed, through
`Verified::in_part_if`; with several pages, `sv` asks each and the credit stays plain. A test asserts both cases.

**What is not done.** The inventory was not audited credit by credit to the end. The build covers the checks the
inventory named in these groups, in `signed_in/` (admin, codes, flows, forgery, sessions, tokens, totp, uploads,
burst, once, archives, passwords, and signin), `browser.rs`, `ai.rs`, `logs.rs`, `mcp_server.rs`, and `probes.rs`. A
check outside that list that gives plain *checked* from one sample is not yet marked; backlog 0006 part 4 stays partly
done until that audit is finished, and backlog 0232 asks for more samples in each group.

**Tests.**

- Each marked credit's existing test now also asserts the credit is in part, through `credited_in_part` in
  `crates/sv-check/src/signed_in/in_part_tests.rs` and the sibling assertions in each check's own tests.
- Group H's both cases are asserted in `crates/sv-check/src/browser/tests/in_part_tests.rs`.
- The full run of the three crates passed in its completed test binaries. Eight MCP tests in one run failed on
  "the check did not finish within 50 seconds", a timeout under load; the same eight pass when run alone (126 of 126).
  The Docker-backed tests were not run in this session; CI requires a container backend.

**Broken on purpose, each put back:**

- The mark removed from one `codes.rs` credit (EMAIL_CODE_REUSABLE): 1 test red.
- The mark removed from one `totp.rs` credit (TOTP_REUSED): 2 tests red.
- The mark removed from one `uploads.rs` credit (the four upload questions): 1 test red.

**Records.** ADR-053, Later, the accepted note of 10 October 2026, is in place; this entry is the detail.
