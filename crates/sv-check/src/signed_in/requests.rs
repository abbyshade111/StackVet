//! Requests from the templates in stackvet.toml: filling in the values, the seed's and the short user names, sending
//! a form with its anti-forgery token, and reading what came back. Moved out of `signed_in/mod.rs` unchanged (backlog
//! 0324).

use super::*;

// ------------------------------------------------------------------------------------------------
// Requests from templates

/// The values a template's placeholders stand for in one request.
#[derive(Default, Clone)]
pub(super) struct Values<'a> {
    pub(super) user: &'a str,
    pub(super) password: &'a str,
    pub(super) csrf: Option<String>,
    pub(super) marker: &'a str,
    pub(super) id: &'a str,
    pub(super) new_password: &'a str,
    pub(super) new_email: &'a str,
    /// The new phone number, two-factor number, or recovery address (ADR-105).
    pub(super) new_value: &'a str,
    pub(super) code: &'a str,
}

// The seed's values for this run (backlog 0536), set by `run_within` from the accounts, so every request filled on this
// thread can cite them.
thread_local! {
    static SEED_VALUES: std::cell::RefCell<BTreeMap<String, String>> =
        const { std::cell::RefCell::new(BTreeMap::new()) };
    /// How many sign-ups this run has sent: the next numbered value a sign-up takes (ADR-134).
    static SIGN_UPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Sets the seed's values for the run on this thread, and back when dropped.
pub(crate) struct SeedValues(pub(super) BTreeMap<String, String>);

impl SeedValues {
    pub(crate) fn set(values: &BTreeMap<String, String>) -> SeedValues {
        SIGN_UPS.with(|n| n.set(0));
        SeedValues(SEED_VALUES.with(|c| c.replace(values.clone())))
    }
}

impl Drop for SeedValues {
    fn drop(&mut self) {
        SEED_VALUES.with(|c| *c.borrow_mut() = std::mem::take(&mut self.0));
    }
}

/// A sign-up is about to be sent: the numbered seed values move on by one (ADR-134), so each account the run makes takes
/// a value of its own, as an app whose invite codes work once needs.
pub(super) fn next_sign_up() {
    SIGN_UPS.with(|n| n.set(n.get() + 1));
}

/// `text` with each `{seed:NAME}` the seed printed put in its place. When the seed printed numbered values, `NAME_1`,
/// `NAME_2`, and on, `{seed:NAME}` is the one for this sign-up, the first for the first (ADR-134); when it printed none,
/// it is `NAME`, whichever sign-up it is. A name it did not print, or a sign-up past the numbered values it printed, is
/// left as written, so the app refuses it and the refusal is quoted (backlog 0599).
pub(super) fn fill_seed(text: &str) -> String {
    if !text.contains("{seed:") {
        return text.to_owned();
    }
    let sign_up = SIGN_UPS.with(|n| n.get()).max(1);
    SEED_VALUES.with(|values| {
        let values = values.borrow();
        let mut out = text.to_owned();
        for name in cited(text) {
            let numbered = values.contains_key(&format!("{name}_1"));
            let value = if numbered {
                values.get(&format!("{name}_{sign_up}"))
            } else {
                values.get(&name)
            };
            if let Some(value) = value {
                out = out.replace(&format!("{{seed:{name}}}"), value);
            }
        }
        out
    })
}

/// Why an account a check signed up was most likely not made, for the report (backlog 0637, ADR-134): when the sign-up
/// form cites a value the seed printed, that the value may work once, and how to give one per account; when it cites
/// none, that the form may need a value stackvet.toml does not carry.
pub(crate) fn sign_up_value_note(signup: &RequestTemplate) -> String {
    let names = seed_names(signup);
    let Some(name) = names.first() else {
        return "The sign-up form may need a value stackvet.toml does not carry, such as an invite code.".to_owned();
    };
    let (numbered, sign_ups) = SEED_VALUES.with(|values| {
        let values = values.borrow();
        let numbered = (1..)
            .take_while(|n| values.contains_key(&format!("{name}_{n}")))
            .count();
        (numbered, SIGN_UPS.with(|n| n.get()))
    });
    if numbered == 0 {
        format!(
            "The sign-up form cites `{{seed:{name}}}`, one value the seed printed, and this run signs several accounts \
             up with it: a value that works once, as an invite code often does, lets only the first in. Have the seed \
             print one for each account, `{name}_1=…`, `{name}_2=…`, and on, and each sign-up takes the next."
        )
    } else {
        format!(
            "The seed printed {numbered} numbered `{name}_` value{} and this run made {sign_ups} sign-ups, so those \
             past them went without one. Have the seed print more.",
            if numbered == 1 { "" } else { "s" }
        )
    }
}

/// The names `{seed:NAME}` cites in `text`, once each.
fn cited(text: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("{seed:") {
        rest = &rest[start + "{seed:".len()..];
        let Some(end) = rest.find('}') else { break };
        let name = rest[..end].to_owned();
        if !names.contains(&name) {
            names.push(name);
        }
        rest = &rest[end..];
    }
    names
}

/// The names `{seed:NAME}` cites in `template`, in order and once each.
pub(crate) fn seed_names(template: &RequestTemplate) -> Vec<String> {
    let mut names = Vec::new();
    for text in template
        .form
        .values()
        .chain(template.json.values())
        .chain(std::iter::once(&template.path))
    {
        let mut rest = text.as_str();
        while let Some(start) = rest.find("{seed:") {
            rest = &rest[start + "{seed:".len()..];
            let Some(end) = rest.find('}') else { break };
            let name = rest[..end].to_owned();
            if !names.contains(&name) {
                names.push(name);
            }
            rest = &rest[end..];
        }
    }
    names
}

pub(super) fn fill(text: &str, v: &Values) -> String {
    fill_seed(text)
        .replace("{user}", v.user)
        .replace("{new_password}", v.new_password)
        .replace("{new_email}", v.new_email)
        .replace("{new_value}", v.new_value)
        .replace("{password}", v.password)
        .replace("{csrf}", v.csrf.as_deref().unwrap_or(""))
        .replace("{marker}", v.marker)
        .replace("{id}", v.id)
        .replace("{code}", v.code)
}

pub(super) fn uses_csrf(t: &RequestTemplate) -> bool {
    t.form
        .values()
        .chain(t.json.values())
        .any(|v| v.contains("{csrf}"))
}

pub(super) fn form_encode(text: &str) -> String {
    let mut out = String::new();
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

// A user name the app's own rule would take (backlog 0599). In the Cursor and Flash run of 15 October 2026 the app
// limited user names to 3 to 30 characters, and every sign-up with a 38-character address in its `username` field was
// refused. When the sign-up form keeps a `username` field apart from the field for the address (both `{user}`), every
// `username` field in the run is sent a short name made from the address instead, and the address stays where it is.
thread_local! {
    static SHORT_NAMES: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Whether a field's name is a user name's: `username`, `user_name`, `userName`, or one inside brackets
/// (`user[username]`).
pub(super) fn names_username(key: &str) -> bool {
    let last = key.trim_end_matches(']').rsplit('[').next().unwrap_or(key);
    last.to_ascii_lowercase().replace(['_', '-'], "") == "username"
}

/// Whether this sign-up keeps a user name apart from the address: a `username` field and another field both carry
/// `{user}`.
pub(crate) fn wants_short_names(signup: &RequestTemplate) -> bool {
    let fields = || signup.form.iter().chain(signup.json.iter());
    fields().any(|(k, v)| names_username(k) && v.contains("{user}"))
        && fields().any(|(k, v)| !names_username(k) && v.contains("{user}"))
}

/// The short user name for `user`: `sv` and twelve hex digits of a hash of it, fourteen letters and digits that
/// differ from one address to the next and that most user-name rules take.
pub(crate) fn short_name(user: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in user.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("sv{:012x}", hash >> 16)
}

/// Sets short user names for the run on this thread, and back when dropped.
pub(crate) struct ShortNames(pub(super) bool);

impl ShortNames {
    pub(crate) fn set(on: bool) -> ShortNames {
        ShortNames(SHORT_NAMES.with(|c| c.replace(on)))
    }
}

impl Drop for ShortNames {
    fn drop(&mut self) {
        SHORT_NAMES.with(|c| c.set(self.0));
    }
}

/// A field's value: a `username` field's `{user}` is the short name when the run uses short names.
pub(super) fn fill_field(key: &str, text: &str, v: &Values) -> String {
    if SHORT_NAMES.with(std::cell::Cell::get) && names_username(key) {
        fill(&text.replace("{user}", &short_name(v.user)), v)
    } else {
        fill(text, v)
    }
}

pub(super) fn request(
    id: &str,
    t: &RequestTemplate,
    v: &Values,
    session: &Session,
) -> ProbeRequest {
    let mut headers = session.headers();
    let body = if !t.json.is_empty() {
        headers.push(("Content-Type".to_owned(), "application/json".to_owned()));
        let map: serde_json::Map<String, serde_json::Value> = t
            .json
            .iter()
            .map(|(k, val)| (k.clone(), serde_json::Value::String(fill_field(k, val, v))))
            .collect();
        Some(serde_json::Value::Object(map).to_string())
    } else if !t.form.is_empty() {
        headers.push((
            "Content-Type".to_owned(),
            "application/x-www-form-urlencoded".to_owned(),
        ));
        let pairs: Vec<String> = t
            .form
            .iter()
            .map(|(k, val)| format!("{}={}", form_encode(k), form_encode(&fill_field(k, val, v))))
            .collect();
        Some(pairs.join("&"))
    } else {
        Some(String::new())
    };
    if let Some(token) = &v.csrf
        && !token.is_empty()
    {
        // The header most frameworks accept for a token that did not come in a form field.
        headers.push(("X-CSRF-Token".to_owned(), token.clone()));
        headers.push(("X-CSRFToken".to_owned(), token.clone()));
        headers.push(("X-XSRF-TOKEN".to_owned(), token.clone()));
    }
    ProbeRequest {
        id: id.to_owned(),
        method: t.method.to_uppercase(),
        path: fill(&t.path, v),
        headers,
        body: body.map(String::into_bytes),
    }
}

pub(crate) fn get(id: &str, path: &str, session: &Session) -> ProbeRequest {
    ProbeRequest {
        id: id.to_owned(),
        method: "GET".to_owned(),
        path: path.to_owned(),
        headers: session.headers(),
        body: None,
    }
}

/// The finding for the private pages that opened to somebody not signed in, when there are any.
pub(super) fn served_without_signing_in(pages: &[String]) -> Option<crate::Finding> {
    (!pages.is_empty()).then(|| {
        finding_on(
            vec!["private-anonymous".to_owned()],
            &PRIVATE_PAGE,
            "A page meant for signed-in users opens without signing in",
            Severity::High,
            format!(
                "Asked as somebody who had not signed in, the app served {}.",
                pages.join(", ")
            ),
        )
    })
}

/// Whether an answer is the app's page shell: a success with a body exactly the front page's, short
/// enough to have been kept whole. Matching a cut body would set aside a server-rendered page that
/// only begins as the front page does.
pub(super) fn is_page_shell(
    response: &Option<ProbeResponse>,
    front: &Option<ProbeResponse>,
) -> bool {
    let (Some(page), Some(front)) = (response, front) else {
        return false;
    };
    (200..300).contains(&page.status)
        && (200..300).contains(&front.status)
        && !page.body.trim().is_empty()
        && page.body.chars().count() < crate::probes::KEPT_CHARS
        && page.body == front.body
}

pub(crate) fn ok(response: &Option<ProbeResponse>) -> bool {
    response
        .as_ref()
        .is_some_and(|r| (200..300).contains(&r.status))
}

/// Accepted: done, or done and sent elsewhere. A refusal is 4xx; a crash is 5xx.
pub(super) fn accepted(response: &Option<ProbeResponse>) -> bool {
    response
        .as_ref()
        .is_some_and(|r| (200..400).contains(&r.status))
}

pub(crate) fn status(response: &Option<ProbeResponse>) -> String {
    response
        .as_ref()
        .map_or("no answer".to_owned(), |r| r.status.to_string())
}

/// Sends a templated request, fetching a page for its token first when the template needs one.
///
/// The template's own path is tried first — a form usually posts back to the page that shows it —
/// then `pages`, in order: a sign-out button, for one, lives on every page but its own address
/// often shows nothing at all.
pub(super) fn send_template(
    http: &mut dyn Http,
    id: &str,
    t: &RequestTemplate,
    values: &Values,
    session: &mut Session,
    pages: &[String],
) -> (Option<ProbeResponse>, Vec<Cookie>) {
    send_template_as(http, id, t, values, session, pages, |_| {})
}

/// A template's request, ready to send: its values filled in, and the page's anti-forgery token
/// fetched first when it asks for one.
pub(super) fn prepared(
    http: &mut dyn Http,
    id: &str,
    t: &RequestTemplate,
    values: &Values,
    session: &mut Session,
    pages: &[String],
) -> ProbeRequest {
    let v = with_token(http, id, t, values, session, pages);
    request(id, t, &v, session)
}

/// `values`, with the anti-forgery token the request needs, when it needs one: fetched from the
/// request's own page or, failing that, the first of `pages` that has one in its markup, and only
/// when none does, from a cookie. A page that answers without the form (a 405 at the change's own
/// address, its forms being on another page) must not end the search with a cookie's value, which
/// is not the form's token in an app that keeps them apart (backlog 0385).
pub(super) fn with_token<'a>(
    http: &mut dyn Http,
    id: &str,
    t: &RequestTemplate,
    values: &Values<'a>,
    session: &mut Session,
    pages: &[String],
) -> Values<'a> {
    let mut v = values.clone();
    if uses_csrf(t) && v.csrf.is_none() {
        let own = fill(&t.path, &v);
        for path in std::iter::once(&own).chain(pages.iter()) {
            let page = http.send(&get(&format!("{id}-page"), path, session));
            if let Some(page) = &page {
                session.absorb(page);
                v.csrf = page_token(page);
            }
            if v.csrf.is_some() {
                break;
            }
        }
        if v.csrf.is_none() {
            v.csrf = cookie_token(session);
        }
    }
    v
}

/// `send_template`, with the request changed by `adjust` after the token is in it and before it
/// goes: for a header a particular browser would send.
pub(super) fn send_template_as(
    http: &mut dyn Http,
    id: &str,
    t: &RequestTemplate,
    values: &Values,
    session: &mut Session,
    pages: &[String],
    adjust: impl FnOnce(&mut ProbeRequest),
) -> (Option<ProbeResponse>, Vec<Cookie>) {
    let mut sent = prepared(http, id, t, values, session, pages);
    adjust(&mut sent);
    let response = http.send(&sent);
    let cookies = response.as_ref().map(set_cookies).unwrap_or_default();
    if let Some(r) = &response {
        session.absorb(r);
    }
    (response, cookies)
}

/// Sends a template whose own placeholders are already filled in, fetching the page's anti-forgery
/// token first when it asks for one, as `send_template` does for the checks here.
/// Creates the `owned` record as whoever `session` belongs to, with `marker` in it, and gives back
/// the app's answer and the id it gave the record. For the AI checks, which ask the app's own tools
/// for a record by its id (C9.5.3).
pub(crate) fn create_owned(
    http: &mut dyn Http,
    users: &UsersSection,
    session: &mut Session,
    marker: &str,
) -> (Option<ProbeResponse>, Option<String>) {
    let Some(owned) = &users.owned else {
        return (None, None);
    };
    let values = Values {
        marker,
        ..Default::default()
    };
    let (created, _) = send_template(
        http,
        "owned-create-ai",
        &owned.create,
        &values,
        session,
        &users.private,
    );
    let id = created.as_ref().and_then(|r| record_id(owned, r));
    (created, id)
}

pub(crate) fn send_filled(
    http: &mut dyn Http,
    id: &str,
    t: &RequestTemplate,
    session: &mut Session,
    pages: &[String],
) -> Option<ProbeResponse> {
    send_template(http, id, t, &Values::default(), session, pages).0
}

#[cfg(test)]
#[path = "seed_numbered_tests.rs"]
mod seed_numbered_tests;
