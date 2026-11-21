//! What a check says, shared by every crate that makes or reads findings and credits: `Finding` and `Verified`, the
//! severity, confidence, and tier they carry, and `Secret`, which redacts on the way in.
//!
//! Moved out of `sv-check` on 20 November 2026, unchanged (backlog 0686), so a crate that needs the types does not
//! build every check and seventeen grammars to get them: the first step of ADR-157, for ConfigVet. `sv-check`
//! re-exports all of it under the old paths. It depends on `sv-scan` alone, for which files are test code and which
//! language a file is in, which two of `Finding`'s methods ask.

pub mod finding;
pub mod rule_ids;
pub mod text;
pub mod verified;
pub mod words;

pub use finding::{Confidence, Finding, Location, Secret, Severity};
pub use verified::{Tier, Verified};
