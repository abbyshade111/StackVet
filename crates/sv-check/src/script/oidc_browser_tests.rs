//! Backlog 0629: with a browser in the run, the sign-in through the test provider is given one too, and its storage
//! after signing out is asked (V14.3.1); with none, the provider's way to the app asks for no browser, as before.

use super::*;
use std::cell::RefCell;

struct Recorded {
    withs: RefCell<Vec<With>>,
}

struct Quiet;
impl Http for Quiet {
    fn send(&mut self, _request: &ProbeRequest) -> Option<ProbeResponse> {
        None
    }
}

impl Services for Recorded {
    fn http<'s>(&'s self, _target: Target, with: With) -> Box<dyn Http + 's> {
        self.withs.borrow_mut().push(with);
        Box::new(Quiet)
    }
    fn model_canary(&self) -> Option<String> {
        None
    }
    fn app_log(&self) -> String {
        String::new()
    }
    fn seed(
        &self,
        _target: Target,
        _seed: &str,
        _accounts: &Accounts,
    ) -> Result<std::collections::BTreeMap<String, String>, String> {
        Ok(Default::default())
    }
    fn start_switched_off(&self, _setting: &str) -> bool {
        false
    }
    fn remove_switched_off(&self) {}
    fn liveness(&self, after: &str) -> Liveness {
        Liveness {
            after: after.to_owned(),
            status: "running".to_owned(),
            restarts: 0,
            exit_code: 0,
            out_of_memory: false,
            answered: true,
        }
    }
}

/// The provider's way to the app, and what the sign-in through it said of V14.3.1.
fn run_with(users: &UsersSection) -> (With, Vec<String>) {
    let services = Recorded {
        withs: RefCell::new(Vec::new()),
    };
    let policy = PolicySection::default();
    let oidc = OidcSection {
        start: "/login/oidc".into(),
        private: "/account".into(),
        ..Default::default()
    };
    let plan = Plan {
        users: Some(users),
        policy: &policy,
        oidc: Some(&oidc),
        oauth_client: None,
        ai: None,
        mcp_server: None,
        fetch: None,
        health_path: "/",
        slow: false,
        accounts: None,
        mcp_token: None,
    };
    let out = run(&services, &plan, &[]);
    let provider = *services
        .withs
        .borrow()
        .iter()
        .find(|w| w.provider)
        .expect("the provider's way to the app was asked for");
    let said = out
        .oidc
        .expect("the sign-in through the provider ran")
        .not_assessed
        .into_iter()
        .filter(|(ids, _)| ids == "V14.3.1")
        .map(|(_, why)| why)
        .collect();
    (provider, said)
}

#[test]
fn with_a_browser_the_providers_sign_in_has_one_and_its_storage_is_asked() {
    let users = UsersSection {
        browser: Some(Default::default()),
        ..Default::default()
    };
    let (provider, said) = run_with(&users);
    assert!(provider.browser, "{provider:?}");
    // Asked, and said why it could not be: this harness's app answers nothing.
    assert!(
        said.iter()
            .any(|w| w.starts_with("In a real browser, after a sign-in through the test provider")),
        "{said:?}"
    );
}

#[test]
fn without_a_browser_the_providers_way_in_asks_for_none_and_says_nothing_of_storage() {
    let (provider, said) = run_with(&UsersSection::default());
    assert!(!provider.browser, "{provider:?}");
    assert!(said.is_empty(), "{said:?}");
}
