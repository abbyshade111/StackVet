# A placeholder security contact is noted

**Status:** claimed by documentation, 14 October 2026

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
