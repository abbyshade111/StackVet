# The weak-key check reads comments, and the certificate check reads one language's switch-off inside another language's text

**Status:** claimed by claude/0633-triage, 23 October 2026

From the triage of StackVet's findings on its own repository (backlog 0633, 23 October 2026). Two checks of StackVet's own read text that cannot do what they look for:

- **The weak-key check (V11.6.1, `crates/sv-check/src/weak_keys.rs`) reads comments.** Its 9 findings on StackVet are
  the `//` comments that describe its own patterns (`// Node: generateKeyPair('rsa', { modulusLength: 1024 })`). A
  comment makes no key. The certificate check already skips comment lines; this check does not.
- **The certificate check (`crates/sv-check/src/cert_checks.rs`) reads one language's switch-off in another's
  text.** Its 2 findings are Rust string literals naming Python's `ssl._create_default_https_context =
  ssl._create_unverified_context` and Node's `https.globalAgent.options.rejectUnauthorized = false`. Python's line does
  something only in Python, and Node's only in JavaScript; in a Rust, Go, or Java file it is text. A shell script, a
  Dockerfile, a notebook, or a YAML file can run either (`python -c "..."`), so those stay read.

To do: skip comment lines in the weak-key check as the certificate check does; read each language's switch-off only in
files of that language or of no programming language of their own; and test both, with the cases that must still
fire (a key made in code, a switch-off in a `.py` file and in a shell script's `python -c`). ADR-137, proposed.
