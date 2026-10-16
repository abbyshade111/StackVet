//! Which capability answers the owner confirmed through `sv review` (backlog 0606, the owner's choice of 16 October
//! 2026). A confirmed answer is shown as the owner's; it switches no requirement on or off, and weighs no more in any
//! count than an answer nobody recorded (ADR-015: what the owner says never removes a requirement).

use crate::seal::{Checker, as_strs, capabilities_review_fields};

/// The names of the answers confirmed through `sv review` and still the same, empty when none is: the seal must hold on
/// this computer, and each answer must still say what it said when confirmed.
pub fn confirmed(manifest: &sv_manifest::Manifest, seals: &Checker) -> Vec<String> {
    let Some(entry) = &manifest.capabilities_review else {
        return Vec::new();
    };
    if seals
        .recorded(
            entry.seal.as_deref(),
            &as_strs(&capabilities_review_fields(entry)),
        )
        .is_err()
    {
        return Vec::new();
    }
    entry
        .answers
        .keys()
        .filter(|name| entry.holds_for(manifest, name))
        .cloned()
        .collect()
}

/// The answers `manifest` gives that are not confirmed here, as (name, answer): what `sv review` offers.
pub fn unconfirmed(manifest: &sv_manifest::Manifest, seals: &Checker) -> Vec<(String, bool)> {
    let done = confirmed(manifest, seals);
    manifest
        .claims()
        .into_iter()
        .filter_map(|(condition, said)| said.map(|s| (condition.name().to_owned(), s)))
        .filter(|(name, _)| !done.contains(name))
        .collect()
}
