//! The tests that start an example app in a container and wait for it to come up, gathered into one test
//! program (backlog 0339). `cargo test` runs its programs one after another and a program's tests on
//! threads at once, so as four files these were four waits in a row; as modules of one program they wait
//! together. Each test still makes its own copy of its app, under a name no other test here uses, and
//! still fails under `SV_REQUIRE_BACKEND=1` when no container backend answers.
//!
//! Only tests that need nothing from the others belong here: none sends a signal, holds a folder's lock,
//! or runs an app in the same folder as another test. Those stay in programs of their own (`interrupt.rs`,
//! `killed_run.rs`, `report_lock.rs`, `run_progress.rs` and `report_suite_timings.rs`, which both run
//! `examples/tested-notes` in place).

mod email_change_example;
mod host_cookie;
mod loopback_start;
mod seen_record;
