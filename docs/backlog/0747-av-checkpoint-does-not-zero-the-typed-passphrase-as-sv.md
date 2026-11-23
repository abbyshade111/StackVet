# av checkpoint does not zero the typed passphrase, as sv review does

**Status:** open

Found by the ninth weekly review of the decision records (0742, 23 November 2026), reading ADR-043 against `main`.

ADR-043's entry of 8 October 2026 says "Each typed passphrase is now zeroed when it goes out of scope", which
`sv review` does with `zeroize::Zeroizing` (`crates/sv-cli/src/review.rs`). `av checkpoint` (#2506) asks for the same
key's passphrase and holds it as a plain `String` (`passphrase` in `crates/av-cli/src/checkpoint.rs`); neither
`av-cli` nor `av-record` uses `zeroize`. So the same passphrase is typed into a second program that does not wipe it
from memory when done. Small, since both run at the owner's own terminal, but the record's sentence is no longer true
of every place the passphrase is typed. The review wrote the fact on ADR-043.

**To do.** Hold the passphrase in `Zeroizing<String>` in `av checkpoint`, through to the call that opens the key, as
`sv review` does (`zeroize` is already built for `sv-check` and `sv-cli`, so it is no new download; it is a new line
in `av-cli`'s manifest, which ADR-159 and ADR-020 govern). A test beside `checkpoint_tests.rs`.
