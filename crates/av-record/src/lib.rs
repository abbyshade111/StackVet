//! The records of what an AI coding tool did, and their readers, shared by `sv` and AgentVet (ADR-159, decision 2;
//! backlog 0705). Each module was moved here from `sv-cli` on 21 November 2026, word for word but for where it finds
//! its helpers, and `sv-cli` gives each under its old path, so `sv` says what it said before.
//!
//! - `build_loop` (ADR-076): one line for each call the tool made to `sv`'s MCP server.
//! - `hook_record` (ADR-122): one line for each command the tool ran and each file it wrote, kept by a hook.
//! - `envelope` (ADR-159, decision 8): the task envelope, `agentvet.toml`, read and never judged here. New on 21
//!   November 2026 (backlog 0715), not moved from anywhere.
//! - `held` (ADR-159, decision 8): what the records show a tool did, set against the envelope. New on 21 November
//!   2026 (backlog 0715).
//! - `groups` (ADR-159): what all of these show, gathered under the design's six groups. New on 21 November 2026
//!   (backlog 0715).
//! - `passthrough` (ADR-159, decision 10): the record of each call an agent made to a tool through `av proxy`.
//!   New on 21 November 2026 (backlog 0716).
//! - `proxy_seen` (ADR-159, decision 10): what that record holds, for a report. New on 22 November 2026 (backlog
//!   0716, part 2).
//! - `checkpoint` (ADR-159, decision 12): where the owner signed that each record's chain had got to. New on 22
//!   November 2026 (backlog 0716).
//! - `outside_note` (ADR-159, decision 12): where the build-loop record's chain had got to, noted outside the app's
//!   folder each time a line is written. New on 22 November 2026 (backlog 0716).
//! - `tool_behavior` (ADR-131): what the tool did that is worth a look, read from git, the hook record, and the last
//!   report.
//!
//! Two readers take a report's bytes rather than its folder (`hook_record::tip_in`, `tool_behavior::earlier_in`):
//! showing that `sv` wrote a report is the report seal's work, which stays in `sv-cli`, and each caller does it first.
//!
//! Nothing here prints, and nothing here opens a network connection.

pub mod build_loop;
pub mod checkpoint;
pub mod envelope;
pub mod groups;
pub mod held;
pub mod hook_record;
pub mod outside_note;
pub mod passthrough;
pub mod plain;
pub mod proxy_seen;
pub mod tool_behavior;

#[cfg(test)]
mod tool_behavior_installs_tests;
#[cfg(test)]
mod tool_behavior_tests;

#[cfg(test)]
mod hook_record_tests;

#[cfg(test)]
mod hook_record_claims_tests;
#[cfg(test)]
mod hook_record_kinds_tests;
