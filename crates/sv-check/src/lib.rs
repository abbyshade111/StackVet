//! The checks that produce findings: what is wrong with this app, and what it is evidence about.
//!
//! `sv-scan` answers questions about what the app *is*. This answers what is *wrong with it*. The two are
//! kept apart because they fail differently: a scanner that cannot tell whether an app uses XML makes a
//! requirement not-assessed, while a check that cannot run makes a finding absent — and an absent finding
//! looks exactly like a clean result unless something says otherwise. `Coverage` is that something.

/// How a pip requirements file is read, line by line, shared with `sv-run`'s install step, which reads
/// `sv-scan` only through this crate (backlog 0291).
pub use sv_scan::requirements;
pub mod adapters;
pub mod advisories;
pub mod ai;
pub mod ai_tool;
pub mod ast;
pub mod browser;
pub mod browser_storage;
pub mod bundled;
pub mod cert_checks;
pub mod ci_codeql;
pub mod client_tech;
pub mod coding_rules;
pub mod config;
pub mod confirm;
pub mod crypto_inventory;
pub mod cvss;
pub mod cvss4;
mod cvss4_tables;
pub mod decisions;
pub mod design;
pub mod encoding;
pub mod fetch;
pub mod finding;
pub mod git;
pub mod grants;
pub mod hand;
pub mod hosted_rules;
pub mod human;
pub mod junit;
pub mod launch;
pub mod level_hints;
pub mod live_tls;
pub mod locate;
pub mod logged_ids;
pub mod logs;
pub mod look_away;
pub mod manifest_lock;
pub mod mcp_server;
pub mod model_files;
pub mod notes;
pub mod oidc;
pub mod personal_fields;
pub mod probes;
pub mod production;
pub mod prompts;
pub mod public_keys;
pub mod review;
pub mod rich_text;
pub mod route_guards;
pub mod running;
pub mod sbom;
pub mod script;
pub mod seal;
pub mod secrets;
pub mod security_contact;
pub mod session_settings;
pub mod sign_in_paths;
pub mod signed;
pub mod signed_in;
mod ssh_format;
pub mod stand_in;
pub mod suite;
pub mod template_escaping;
pub mod test_report;
pub mod text;
// Built always, not only for this crate's tests, so the tests of `sv-run` and `sv-cli` write their
// programs the same way (backlog 0501); nothing in `sv` calls it.
#[doc(hidden)]
pub mod test_support;
pub mod totp;
pub mod verified;
pub mod weak_keys;
pub mod workflows;

pub use finding::{Confidence, Finding, Location, Secret, Severity};
pub use sbom::{Sbom, build as build_sbom, to_cyclonedx};
pub use secrets::{SecretRules, SecretScan, scan_text};
pub use verified::{Tier, Verified};
