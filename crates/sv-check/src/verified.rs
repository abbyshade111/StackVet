//! What a check looked at and found nothing wrong with.
//!
//! The mirror of `Finding`, and the harder of the two to get right. A finding is a claim about
//! something that is there; this is a claim about something that is *not*, and a claim of absence is
//! only worth the coverage behind it. Three rules hold everywhere one of these is produced:
//!
//! **Fail closed.** A check emits nothing unless it read everything it would have needed to read. A
//! credential scan that skipped four files has not established that the app holds no credentials; it
//! has established nothing, and the skipped files are already reported as a gap. Emitting a weakened
//! claim instead would put a reassuring line in the report next to a caveat nobody reads.
//!
//! **Say the scope in the words a person would use.** `scope` is printed beside the claim, because
//! "checked" means nothing without "over what". A reader who can see *12 Python files* can tell at a
//! glance that the Ruby half of their app was not part of it.
//!
//! **Only what the check actually tests.** `requirement_ids` here are the same ids the check cites
//! when it fails. A check that names more requirements when it passes than when it fails is claiming
//! credit for work it did not do, and that is the one direction this type must never move in.

// Moved to `sv-core` on 20 November 2026, unchanged (backlog 0686, ADR-157), and re-exported here under its old path.
pub use sv_core::verified::*;
