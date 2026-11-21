//! The two rule ids `Finding`'s methods name, kept here so those methods can move with the type (backlog 0686).
//! `sv-check` names them by their old paths, `secrets::ASSIGNMENT_RULE` and `suite::NAME_MISMATCH`, which are these.

/// The rule that reports a value that looks like a credential assigned to a name that says so, written in
/// `sv-check`'s `secrets` rather than in `data/secret-rules.json`.
pub const ASSIGNMENT_RULE: &str = "secrets.credential-assignment";

/// The rule that says a test named for a requirement shares no words with it. Information only: see
/// `Finding::withholds_credit`.
pub const NAME_MISMATCH: &str = "tests.name-does-not-match-requirement";
