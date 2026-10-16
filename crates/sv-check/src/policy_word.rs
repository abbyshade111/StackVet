//! `[policy] fix-within-days` as the documentation V15.1.1 asks for (backlog 0498, the owner's choice of 16 October
//! 2026): the time frames for fixing a known vulnerability, in a place `sv audit` already reads. Confirmed by the owner
//! through `sv review`, sealed, and still the same, they are V15.1.1 *documented by the owner*; written but not
//! confirmed, they are the AI coding tool's word, *stated*, as an unconfirmed answer in the security notes is (ADR-026).
//! Neither is a check: `sv` cannot see whether the app is fixed that fast, only that the owner says it will be.

use crate::seal::{Checker, as_strs, policy_review_fields, recorded_where};
use crate::verified::{Tier, Verified};

const REQUIREMENT: &str = "V15.1.1";

/// The owner's word on V15.1.1 from `fix-within-days`, or `None` when stackvet.toml gives no time frames.
pub fn fix_within_days(manifest: &sv_manifest::Manifest, seals: &Checker) -> Option<Verified> {
    let days = manifest.policy.fix_within_days.as_ref()?;
    let said = days.said();
    let unconfirmed = |why: String| {
        Verified::new(
            "policy.stated-by-ai",
            &[REQUIREMENT],
            format!(
                "stackvet.toml's `[policy] fix-within-days` says a known vulnerability is fixed {said}, but {why}, so \
                 it counts as your AI coding tool's word. If these are your time frames, run `sv review` in your own \
                 terminal to confirm them."
            ),
        )
        .resting_on(Tier::Stated)
    };
    let Some(entry) = &manifest.policy_review else {
        return Some(unconfirmed(
            "nobody confirmed it through `sv review`".to_owned(),
        ));
    };
    match seals.recorded(entry.seal.as_deref(), &as_strs(&policy_review_fields(entry))) {
        Err(why) => Some(unconfirmed(why.why())),
        Ok(_) if !entry.still_holds_for(manifest) => Some(unconfirmed(format!(
            "it changed after it was confirmed on {}",
            entry.on
        ))),
        Ok(sealed) => Some(
            Verified::new(
                "policy.documented",
                &[REQUIREMENT],
                format!(
                    "stackvet.toml's `[policy] fix-within-days`: a known vulnerability is fixed {said}, confirmed by \
                     the owner on {}{}. Your word on how soon, not a check that it happens; `sv audit` holds the \
                     packages to it.",
                    entry.on,
                    recorded_where(&sealed)
                ),
            )
            .resting_on(Tier::Documented),
        ),
    }
}
