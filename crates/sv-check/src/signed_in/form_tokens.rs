//! Anti-forgery tokens: the field and cookie names they go by, and finding one in a page or a cookie. Moved out of
//! `signed_in/mod.rs` unchanged (backlog 0324).

use super::*;

// ------------------------------------------------------------------------------------------------
// Anti-forgery tokens

/// The field and cookie names frameworks put their anti-forgery token under.
pub(super) const CSRF_FIELDS: &[&str] = &[
    "csrf_token",
    "csrfmiddlewaretoken",
    "authenticity_token",
    "_csrf",
    "_token",
    "csrf",
    "__requestverificationtoken",
];
pub(super) const CSRF_COOKIES: &[&str] =
    &["csrftoken", "xsrf-token", "_csrf", "csrf_token", "csrf"];

/// The token a page offers, from a hidden field, a `<meta>` tag, or a cookie.
pub(super) fn csrf_token(page: &ProbeResponse, session: &Session) -> Option<String> {
    page_token(page).or_else(|| cookie_token(session))
}

/// The token a page's own markup offers: a hidden field or a `<meta>` tag.
pub(super) fn page_token(page: &ProbeResponse) -> Option<String> {
    for tag in tags(&page.body, "input") {
        let is_token = attribute(&tag, "name")
            .is_some_and(|n| CSRF_FIELDS.contains(&n.to_lowercase().as_str()));
        if is_token && let Some(value) = attribute(&tag, "value") {
            return Some(value);
        }
    }
    for tag in tags(&page.body, "meta") {
        let names_csrf = attribute(&tag, "name").is_some_and(|n| n.to_lowercase().contains("csrf"));
        if names_csrf && let Some(value) = attribute(&tag, "content") {
            return Some(value);
        }
    }
    None
}

/// The token a cookie the session holds carries, for an app that checks one sent back.
pub(super) fn cookie_token(session: &Session) -> Option<String> {
    session
        .cookies
        .iter()
        .find(|(n, _)| CSRF_COOKIES.contains(&n.to_lowercase().as_str()))
        .map(|(_, v)| v.clone())
}

/// Every `<name ...>` tag in a page, as its text.
pub(super) fn tags(body: &str, name: &str) -> Vec<String> {
    // ASCII lowering only: positions found in `lower` cut `body`, and full lowering changes some
    // characters' length (`İ` grows from two bytes to three), which moves every position after it.
    let lower = body.to_ascii_lowercase();
    let open = format!("<{name}");
    lower
        .match_indices(&open)
        .filter(|(i, _)| {
            // `<input` but not `<inputs`: the next character ends the tag name.
            lower[i + open.len()..]
                .chars()
                .next()
                .is_some_and(|c| c.is_whitespace() || c == '>' || c == '/')
        })
        .map(|(i, _)| {
            let end = lower[i..].find('>').map_or(body.len(), |e| i + e);
            body[i..end].to_owned()
        })
        .collect()
}

/// An attribute's value from inside one tag, quoted with `"`, with `'`, or not at all — all three
/// are valid HTML, and a page written with unquoted attributes was the first real app this met.
///
/// Each name's pattern is compiled once and kept, rather than on every call for every tag of every
/// page (backlog 0463); the names are this crate's own, a handful in all.
pub(super) fn attribute(tag: &str, name: &str) -> Option<String> {
    static PATTERNS: std::sync::LazyLock<
        std::sync::Mutex<std::collections::HashMap<String, regex::Regex>>,
    > = std::sync::LazyLock::new(Default::default);
    let pattern = {
        let mut kept = PATTERNS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match kept.get(name) {
            Some(pattern) => pattern.clone(),
            None => {
                let pattern = regex::Regex::new(&format!(
                    r#"(?i)(?:^|\s){}\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s"'>]+))"#,
                    regex::escape(name)
                ))
                .ok()?;
                kept.insert(name.to_owned(), pattern.clone());
                pattern
            }
        }
    };
    let captures = pattern.captures(tag)?;
    (1..=3)
        .find_map(|i| captures.get(i))
        .map(|m| m.as_str().to_owned())
}
