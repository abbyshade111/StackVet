# The bill of materials lists the app, its image, its language, and the language's modules it uses

**Status:** done, 13 October 2026

The owner's decision of 13 October 2026, from the Antigravity trial's app, which uses only Python's own library: its
`sbom.cdx.json` listed no components at all, which reads as "nothing ships". The owner: "at the very least python
should have been listed. There's an argument to be made for including the python modules installed -- if one of them
has a vulnerability, how do I know if I'm affected if I don't know I have it?"

To build, each entry saying where StackVet got it:
- **The app itself,** as the bill's subject (CycloneDX `metadata.component`), from `stackvet.toml`.
- **The container image it runs in,** from `stackvet.toml`'s `[stack.run] image`.
- **The language it runs on,** with its version: from the image tag when that is all there is, exact when `--run`
  started it and StackVet can read it there (find out whether it can).
- **Under the language,** each module of its own library the app's code imports (Python's `sqlite3`, `http.server`,
  and so on), marked as part of the language and imported by the app's code, not a separate package: Python's flaws
  are fixed in Python's versions, and a flaw in one module matters only if the code imports it.

`complete` keeps its meaning, every package list and lockfile read. Python first; the same shape for other
languages' built-in libraries (Node's `fs`, `http`) where the code's imports can be read. A decision record (what StackVet
writes into the bill), with fixtures, a check against the CycloneDX 1.5 schema's component types, and deliberate breaks.
