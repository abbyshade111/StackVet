//! What each optional key of `[stack.run.users]` costs when stackvet.toml leaves it out (backlog 0533): the checks of
//! the running app that do not run without it, and the requirements they speak to. In the three-tool prompt trial of 14
//! October 2026 two of three apps left `owned` out, so the other-user and forged-form checks never ran, and `sv
//! preflight` said nothing. Preflight reads this table; `unset_keys_tests.rs` holds every row to the run itself, so a row
//! names only requirements the report lists as not assessed when that key is missing.

use sv_manifest::UsersSection;

/// One optional key, what leaving it out stops, and the requirements those checks speak to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unset {
    /// The key as stackvet.toml spells it.
    pub key: &'static str,
    /// The checks that do not run without it, in plain words.
    pub checks: &'static str,
    pub ids: &'static [&'static str],
}

pub const UNSET: &[Unset] = &[
    Unset {
        key: "owned",
        checks: "reading one user's record as another user, a forged form sent from another site, the limit on \
                 making records, which fields a record read back hands over, and saved text written back into a page \
                 unencoded",
        ids: &[
            "V8.2.2", "V3.5.1", "V3.5.2", "V2.4.1", "V15.3.1", "V8.2.3", "V1.2.1",
        ],
    },
    Unset {
        key: "update` or `delete` under `owned",
        checks: "changing or deleting one user's record as another user (reading it is still tried)",
        ids: &["V8.2.2"],
    },
    Unset {
        key: "logout",
        checks: "signing out ending the session, a plain link signing the user out, and a sign-out link on the \
                 private pages",
        ids: &["V7.4.1", "V14.3.1", "V3.5.3", "V7.4.4"],
    },
    Unset {
        key: "admin",
        checks: "an ordinary user opening a page only an admin should see",
        ids: &["V8.2.1"],
    },
    Unset {
        key: "admin-actions",
        checks: "an ordinary user making a request only an admin should make",
        ids: &["V8.3.1"],
    },
    Unset {
        key: "once",
        checks: "an action sent twice at the same instant going through twice",
        ids: &["V2.3.4"],
    },
    Unset {
        key: "signup",
        checks: "the password rules, which are asked through the app's own sign-up, and whether the server applies \
                 the rules its own form states",
        ids: &[
            "V6.2.1", "V6.2.4", "V6.2.5", "V6.2.8", "V6.2.9", "V6.2.11", "V6.2.12", "V2.2.2",
        ],
    },
];

/// The rows whose key `users` leaves out. `signup` is counted only beside `seed`: without both, the run stops, and
/// preflight already says so.
pub fn unset_in(users: &UsersSection) -> Vec<&'static Unset> {
    UNSET
        .iter()
        .filter(|row| match row.key {
            "owned" => users.owned.is_none(),
            "update` or `delete` under `owned" => users
                .owned
                .as_ref()
                .is_some_and(|o| o.update.is_none() && o.delete.is_none()),
            "logout" => users.logout.is_none(),
            "admin" => users.admin.is_empty(),
            "admin-actions" => users.admin_actions.is_empty(),
            "once" => users.once.is_none(),
            "signup" => users.signup.is_none() && users.seed.is_some(),
            other => unreachable!("a row with no test of its key: {other}"),
        })
        .collect()
}
