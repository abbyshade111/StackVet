//! Whether `sv probe` reached the address, and the exit code that follows from it (backlog 0235, ADR-029).
//!
//! Kept apart from the command, which runs `curl`, so the decision is tested on every platform: a test that runs the
//! command needs a `curl` it controls, and on Windows that is not reliably the one the command runs (see
//! `tests/probe_exit.rs`).

use crate::exit;
use sv_check::production::Outcome;

/// Whether the address answered at all, decided from the site's own answers: a question nothing came back from
/// leaves nothing in `verified` or `findings`.
pub fn reached(out: &Outcome) -> bool {
    !(out.findings.is_empty() && out.verified.is_empty())
}

/// The exit code for whether the address was reached. An address that was not is not assessed, so a CI step fails
/// rather than passing on an address it never touched.
pub fn exit_code(reached: bool) -> i32 {
    if reached {
        exit::CLEAN
    } else {
        exit::NOT_ASSESSED
    }
}

#[cfg(test)]
#[path = "probe_verdict_tests.rs"]
mod tests;
