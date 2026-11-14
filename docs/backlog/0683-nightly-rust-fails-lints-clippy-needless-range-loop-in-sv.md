# Nightly: Rust fails: lints (clippy needless_range_loop in sv-report pdf layout)

**Status:** open

Found by the nightly CI read on 13 November 2026.

**What failed.** The scheduled Rust run on `main` at commit `ba58c8c`, run
https://github.com/abbyshade111/StackVet/actions/runs/43412923828, failed in its "lints" job
(`cargo clippy --all-targets -- -D warnings`). Every test suite in the same run passed. The message:

```
error: the loop variable `i` is used to index `p.pages`
   --> crates/sv-report/src/pdf/layout.rs:617:14
617 |     for i in 0..n {
note: for this index operation
   --> crates/sv-report/src/pdf/layout.rs:633:9
633 |         p.pages[i].ops.push_str(&format!(
= note: `-D clippy::needless-range-loop` implied by `-D warnings`
```

**Why now.** No pull request brought it. The same commit, `ba58c8c` (pull request #2405, the last merged), passed the
scheduled Rust run the night before (run 43205533084, 12 November). What changed is the runner's toolchain: the lints
job updated stable Rust from 1.99.1 to 1.100.0 (released 9 November), whose Clippy now flags this loop. So `main` is red
on the Rust workflow with no merge to blame, and every pull request opened from now on will fail its lints job until
this is fixed.

**What fixing it means.** Rewrite the loop at `crates/sv-report/src/pdf/layout.rs:617` to walk `p.pages` with an
iterator (Clippy suggests `.iter_mut().enumerate().take(n)`), or, if the index is truly needed, allow the lint on that
loop with a comment saying why. Then run `cargo clippy --all-targets -- -D warnings` on Rust 1.100 to check nothing else
new is flagged; the run stopped at the first crate that failed, so other crates may have more.

Seen again on 14 November 2026: Rust (scheduled), run <https://github.com/abbyshade111/StackVet/actions/runs/43584943376>, commit `089dbac`. The same error at `crates/sv-report/src/pdf/layout.rs:617` on Rust 1.100.0; every test suite passed. The push run for the same commit (run <https://github.com/abbyshade111/StackVet/actions/runs/43423897373>, 13 November) failed the same way. No open pull request fixes it yet.
