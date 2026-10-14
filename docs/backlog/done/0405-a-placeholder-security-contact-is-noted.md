# A placeholder security contact is noted

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 13 October 2026 in the owner's trial in Antigravity. Early in the build, the AI tool wrote the app's
`SECURITY.md`, its policy for reporting a vulnerability. It gave `security@example.com` as the contact and promised
"We acknowledge reports within 48 hours", a commitment the owner never made. `example.com` is reserved for examples
and receives no mail, so a real report would go nowhere. StackVet credited nothing for the file, rightly, but did not
say anything about it either.

To build: a check that a security contact in `SECURITY.md`, `.well-known/security.txt`, or the like is not a
placeholder: an `example.com`, `example.org`, or `example.net` address (RFC 2606), `localhost`, or an obvious stand-in
such as `you@`, `your-email`, or `changeme`. A note for the owner, with the line. Cite a requirement only if one really
speaks to a disclosure contact; read its text first. Fixtures for each form, a real address that must pass, and a
deliberate break.

**What was built (14 October 2026, session documentation).** A new check, config.security-contact-placeholder
(`crates/sv-check/src/security_contact.rs`; ADR-112), reads every file `config.security-contact` finds, line by line,
and notes the first placeholder contact at its line as a low finding, saying how many more there are. It catches:
an address at `example.com`, `example.org`, or `example.net`; one under the test and example names RFC 2606 keeps
(`.test`, `.example`, `.invalid`); `localhost`; a domain beginning `your`; a web address on a security.txt `Contact:` line
at any of these; and the stand-ins `you@`, `your-email` (and spellings), `changeme`, and `<your ...>` or `[your ...]`. Its
fix asks the owner to check any reply time the file promises. It cites no requirement: ASVS 5.0, AISVS 1.0, its Appendix
C, and the Secure by Design checklist were searched and none asks for a disclosure contact. A clean read is a pass with
no requirement, as `config.security-contact`'s is; with no security file it is not assessed and says why.

Tests in `crates/sv-check/src/security_contact_tests.rs`: each form at its line, security.txt where it is served, two
placeholders in one file, a real address written several ways, everyday words that are not stand-ins, and an
`example.com` request that is an example rather than the contact; each setup asserts the file was where `sv` looks.
Breaking the rule (no name treated as reserved) turned the forms test red; leaving the check unwired turned it and the
not-assessed test red. One fixture changed: the MCP build-loop test's `SECURITY.md` gave `security@example.com`, which is
now rightly a finding of its own, so it gives a real-looking address and the test's counts stand as they were. Not
done: the `security-contact` prompt in `data/prompts.json` still counts only config.security-contact in its trials; a
later trial may want to count this check too.
