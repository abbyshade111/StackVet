# Nightly: Windows build warns: four unused names in code kept for Unix

**Status:** done, 24 October 2026

Recorded by the nightly CI routine on 21 October 2026. No test failed: the scheduled Windows run on `main` passed,
4,095 tests in 323 groups with none failing, on `1eb7c5e`
(<https://github.com/abbyshade111/StackVet/actions/runs/39577917611>). Its build of `sv` printed four compiler
warnings, which nothing on Linux shows, because each is about code that only a Unix build uses:

    crates\sv-check\src\adapters.rs:31    unused import: sv_frameworks::paths::Canonical
    crates\sv-check\src\adapters.rs:951   variants Nowhere, OnlyRelative, and InsideApp of Located are never constructed
    crates\sv-check\src\adapters.rs:2658  variable does not need to be mutable: let mut builder (the mode is set under #[cfg(unix)])
    crates\sv-cli\src\review.rs:226       drop(shown) on a value of type () (quiet::hide_typing returns nothing on Windows)

CI's lints (`cargo clippy --all-targets -- -D warnings`) run on Linux only, which is why nothing went red; this
session's own Linux release build of the same `main` printed no warnings. Whether they were there the night before
was not read. They touch only the Windows build, but a warning nobody reads hides the next one that matters.

The fix is small and local: put the import, the three variants' uses, and the `mut` behind the same `#[cfg(unix)]` as
the code that needs them (or `#[cfg_attr(not(unix), allow(...))]`), and on Windows make `hidden` not `drop` a `()`.
Whether the Windows workflow should also fail on warnings is a separate question for the owner. This routine records;
it fixes nothing.

Seen again on 23 October 2026: Windows (scheduled), run <https://github.com/abbyshade111/StackVet/actions/runs/39932434655>, commit `3970e3b`. The same four warnings (adapters.rs lines 31, 951, and 2658; review.rs line 226); 4,179 tests passed, 0 failed.

Seen again on 24 October 2026: Windows (scheduled), run <https://github.com/abbyshade111/StackVet/actions/runs/40080492823>, commit `125d516`. The same four warnings, two of them moved down by later changes (adapters.rs lines 31, 951, and 2676; review.rs line 231); 4,233 tests passed, 0 failed.

**Built, 24 October 2026.** The `Canonical` import is behind `#[cfg(unix)]`, as the only code that uses it is; the
three `Located` variants that only the Unix locator makes are allowed as unused off Unix
(`#[cfg_attr(not(unix), allow(dead_code))]`); the directory builder's `mut`, needed only for the Unix mode, is allowed
off Unix the same way; and `hidden` in `crates/sv-cli/src/review.rs` hides the typing for one block instead of
dropping what `hide_typing` returns, which on Windows is `()`. On this Mac, clippy is clean on `sv-check` and
`sv-cli`, all of `sv-check`'s tests pass, and so do all of `sv-cli`'s once 0631's fix is in; the passphrase prompts'
tests among them hold the typing being shown again after the answer. Not verified here: the Windows build itself, as
no Windows target is installed on this Mac; the Windows workflow runs on every change to `main`, and its first run after
this merges is the check that the four warnings are gone. Whether the Windows workflow should fail on warnings is
still the owner's question, not taken up here.
