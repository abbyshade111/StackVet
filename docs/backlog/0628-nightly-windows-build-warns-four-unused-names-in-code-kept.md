# Nightly: Windows build warns: four unused names in code kept for Unix

**Status:** open

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
