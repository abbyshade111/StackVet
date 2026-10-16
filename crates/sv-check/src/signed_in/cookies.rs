//! The session a run keeps: the cookies an answer sets, read and kept the way a browser keeps them. Moved out of
//! `signed_in/mod.rs` unchanged (backlog 0324).

use super::*;

// ------------------------------------------------------------------------------------------------
// Sessions

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Cookie {
    pub(super) name: String,
    pub(super) value: String,
    pub(super) http_only: bool,
    pub(super) same_site: Option<String>,
    pub(super) secure: bool,
    pub(super) path: Option<String>,
}

pub(super) fn parse_set_cookie(header: &str) -> Option<Cookie> {
    let mut parts = header.split(';');
    let (name, value) = parts.next()?.split_once('=')?;
    let mut cookie = Cookie {
        name: name.trim().to_owned(),
        value: value.trim().to_owned(),
        http_only: false,
        same_site: None,
        secure: false,
        path: None,
    };
    for attribute in parts {
        let attribute = attribute.trim();
        let (key, val) = attribute.split_once('=').unwrap_or((attribute, ""));
        match key.to_lowercase().as_str() {
            "httponly" => cookie.http_only = true,
            "samesite" => cookie.same_site = Some(val.trim().to_lowercase()),
            "secure" => cookie.secure = true,
            // Only a path a browser would take: one that starts with `/`.
            "path" if val.trim().starts_with('/') => cookie.path = Some(val.trim().to_owned()),
            _ => {}
        }
    }
    Some(cookie)
}

pub(super) fn set_cookies(response: &ProbeResponse) -> Vec<Cookie> {
    response
        .headers
        .iter()
        .filter(|(k, _)| k == "set-cookie")
        .filter_map(|(_, v)| parse_set_cookie(v))
        .collect()
}

/// What a browser would send back: cookies by name, and a bearer token when sign-in gave one.
#[derive(Debug, Clone, Default)]
pub(crate) struct Session {
    pub(super) cookies: Vec<(String, String)>,
    pub(super) bearer: Option<String>,
    /// Each cookie's attributes as the app set them, by name, for handing them to a real browser:
    /// a browser keeps a `__Host-` cookie only when it is `Secure` (family-hub, 3 October 2026).
    pub(super) attributes: Vec<crate::browser::BrowserCookie>,
}

impl Session {
    pub(crate) fn cookies(&self) -> &[(String, String)] {
        &self.cookies
    }

    /// The cookies with the attributes the app gave them, in the order `cookies` has them.
    pub(crate) fn browser_cookies(&self) -> Vec<crate::browser::BrowserCookie> {
        self.cookies
            .iter()
            .map(|(name, value)| {
                let set = self.attributes.iter().find(|c| c.name == *name);
                crate::browser::BrowserCookie {
                    name: name.clone(),
                    value: value.clone(),
                    ..set.cloned().unwrap_or_default()
                }
            })
            .collect()
    }

    pub(crate) fn absorb(&mut self, response: &ProbeResponse) {
        for cookie in set_cookies(response) {
            self.cookies.retain(|(n, _)| *n != cookie.name);
            self.attributes.retain(|c| c.name != cookie.name);
            // An emptied cookie is how most frameworks delete one.
            if !cookie.value.is_empty() {
                self.attributes.push(crate::browser::BrowserCookie {
                    name: cookie.name.clone(),
                    value: String::new(),
                    secure: cookie.secure,
                    http_only: cookie.http_only,
                    path: cookie.path,
                    same_site: cookie.same_site,
                });
                self.cookies.push((cookie.name, cookie.value));
            }
        }
    }

    pub(super) fn headers(&self) -> Vec<(String, String)> {
        let mut out = Vec::new();
        if !self.cookies.is_empty() {
            let line: Vec<String> = self
                .cookies
                .iter()
                .map(|(n, v)| format!("{n}={v}"))
                .collect();
            out.push(("Cookie".to_owned(), line.join("; ")));
        }
        if let Some(token) = &self.bearer {
            out.push(("Authorization".to_owned(), format!("Bearer {token}")));
        }
        out
    }
}
