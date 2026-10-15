//! What the sign-in pathway reader takes as a way in (backlog 0491).

use super::{Pathway, from_route, in_text};
use crate::route_guards::RouteLine;

fn route(method: &str, path: &str) -> RouteLine {
    RouteLine {
        file: "app.py".to_owned(),
        line: 7,
        method: method.to_owned(),
        path: path.to_owned(),
        guards: Vec::new(),
    }
}

fn whats(found: &[Pathway]) -> Vec<&str> {
    found.iter().map(|p| p.what.as_str()).collect()
}

#[test]
fn routes_whose_path_says_sign_in_are_ways_in() {
    for (method, path) in [
        ("POST", "/login"),
        ("GET", "/auth/google"),
        ("GET", "/oauth2/callback"),
        ("POST", "/api/token"),
        ("POST", "/sessions"),
        ("GET", "/magic-link/<token>"),
        ("POST", "/api-keys"),
        ("POST", "/sign-in"),
        ("POST", "/webauthn/verify"),
    ] {
        let way = from_route(&route(method, path)).unwrap_or_else(|| panic!("{path}"));
        assert_eq!(way.what, format!("a route, {method} {path}"));
        assert_eq!((way.file.as_str(), way.line), ("app.py", 7));
    }
}

#[test]
fn routes_about_something_else_are_not() {
    for path in [
        "/",
        "/signup",
        "/logout",
        "/notes",
        "/author/3",
        "/password",
        "/healthz",
        "/tokenizer",
    ] {
        assert_eq!(from_route(&route("GET", path)), None, "{path}");
    }
}

#[test]
fn passport_strategies_compared_paths_and_api_keys_are_ways_in() {
    let source = "\
passport.use(new GoogleStrategy({ clientID }, verify));
passport.use('local', new LocalStrategy(verify));
if (url.pathname === '/login/google') {}
if self.path == \"/login\":
const key = req.headers['x-api-key'];
key = request.headers.get(\"X-API-Key\")
// passport.use(new GitHubStrategy(...))
if (url.pathname === '/notes') {}
";
    let found = in_text("server.js", source);
    assert_eq!(
        whats(&found),
        vec![
            "a Passport strategy, GoogleStrategy",
            "a Passport strategy, LocalStrategy",
            "a path the code compares a request with, /login/google",
            "a path the code compares a request with, /login",
            "an API key read by the name `x-api-key`",
            "an API key read by the name `X-API-Key`",
        ]
    );
    assert_eq!(
        found.iter().map(|p| p.line).collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6]
    );
}
