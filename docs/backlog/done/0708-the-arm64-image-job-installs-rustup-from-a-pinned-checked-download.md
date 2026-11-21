# The arm64 image job installs rustup from a pinned, checked download

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A6, its fourth point).
The `image-arm64` job in `.github/workflows/rust.yml` installs rustup, where the runner has none, by piping
`https://sh.rustup.rs` to `sh`: whatever that address serves that day runs, unpinned, in a job whose image `publish`
later pushes. Every action in the workflows is pinned to a commit fingerprint; this download is the one step that is
not.

- **To build:** download a named rustup version's `rustup-init` for the runner's platform from
  `static.rust-lang.org/rustup/archive/<version>/<target>/`, check it against a SHA-256 written in the workflow (read
  from rustup's published checksum when the pin is made, and said where it came from), and run it only when the
  check passes. A comment names the version and how to move the pin.
- **Check:** with the written checksum changed by one character the step fails before running anything; with the real
  one the job builds as before.
- **Records:** the record that governs `rust.yml`'s pinning gains a Later entry.

**Done, 21 November 2026:** the `image-arm64` job downloads rustup 1.29.3 for its platform, checks it against the
SHA-256 written in `rust.yml`, and runs it only after; `crates/sv-cli/tests/workflow_pinned_installers.rs` holds it,
and no workflow may pipe a download into a shell. The runner had rustup already, so CI skips this path; in an arm64
Linux container without rustup, a checksum one character off stopped the step before anything ran, and the real one
installed rustup 1.29.3. ADR-080, Later.
