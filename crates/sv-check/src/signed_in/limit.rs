//! How the answers to wrong attempts past a stated limit are read, for wrong passwords (V6.3.1,
//! `signin::brute_force_check`) and wrong emailed codes (V6.6.3, `codes`). The two checks carried
//! this reading line for line, and every fix had to be made twice by hand (backlog 0423); each now
//! only words what this says.

use super::signin::{Slowing, slowing};

/// What the answers to `allowed + 2` wrong attempts say.
#[derive(Debug, PartialEq)]
pub(super) enum PastTheLimit {
    /// The first attempt was already refused (or not answered), so whatever the app is refusing,
    /// it is not refusing because of the number stated.
    AlreadyRefusing { first: u16 },
    /// Past the limit the app crashed (500 or above) or did not answer (0): not pushing back, and
    /// not a status that changed, a refusal, or a delay either (backlog 0280).
    NotReal { first: u16, last: u16, next: u16 },
    /// Every answer up to and including the first past the limit was the same, and only the second
    /// past it differed: one more let through than stated (the review of 6 October, item 13).
    OneMoreLetThrough { first: u16, next: u16 },
    /// Real answers past the limit, read for a refusal, a changed status, and a delay.
    Read(Reading),
}

#[derive(Debug, PartialEq)]
pub(super) struct Reading {
    pub first: u16,
    /// The answer to the first attempt past the limit.
    pub last: u16,
    /// That answer says refused outright: 423 or 429.
    pub refused: bool,
    /// That answer differs from the first attempt's.
    pub status_changed: bool,
    pub slowing: Slowing,
    /// The times of the two attempts past the limit.
    pub past: [u128; 2],
    /// The quickest attempt within the limit.
    pub quickest: u128,
}

impl Reading {
    /// A refusal or a different answer: pushing back that is not only a delay.
    pub fn answered_differently(&self) -> bool {
        self.refused || self.status_changed
    }
}

/// Reads `answers` (status, where 0 is no answer at all, and milliseconds) to `allowed + 2` wrong
/// attempts, `allowed` at least 1. `controls` are the times of requests no limit would slow, sent
/// beside the attempts past the limit, for `slowing`.
pub(super) fn past_the_limit(
    answers: &[(u16, u128)],
    allowed: u32,
    controls: &[u128],
) -> PastTheLimit {
    let allowed = allowed as usize;
    assert!(
        allowed >= 1 && answers.len() == allowed + 2,
        "{} answers for a limit of {allowed}",
        answers.len()
    );
    let (first, first_ms) = answers[0];
    if matches!(first, 0 | 423 | 429) {
        return PastTheLimit::AlreadyRefusing { first };
    }
    let last = answers[allowed].0;
    let next = answers[allowed + 1].0;
    let real = |status: u16| status != 0 && status < 500;
    if !real(last) || !real(next) {
        return PastTheLimit::NotReal { first, last, next };
    }
    let refused = matches!(last, 423 | 429);
    let status_changed = last != first;
    // Read only from real answers, every one up to the limit the same.
    let alike = answers[..=allowed].iter().all(|a| a.0 == first);
    if !(status_changed || refused) && alike && real(first) && next != first {
        return PastTheLimit::OneMoreLetThrough { first, next };
    }
    let times: Vec<u128> = answers.iter().map(|a| a.1).collect();
    let (within, past) = times.split_at(allowed);
    PastTheLimit::Read(Reading {
        first,
        last,
        refused,
        status_changed,
        slowing: slowing(within, past, controls),
        past: [past[0], past[1]],
        quickest: within.iter().copied().min().unwrap_or(first_ms),
    })
}
