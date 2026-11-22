//! `cv live cloudflare <zone>`: one zone's settings, read with a token the person gives, through a fixed list of `GET`
//! requests to `api.cloudflare.com` (ADR-157, Later, 22 November 2026; backlog 0732).
//!
//! What the token is, and whether ConfigVet may use it, is settled before anything about the zone is asked:
//!
//! - **A Global API Key (`cfk_`) and Wrangler's own sign-in token (`cfoat_`) are refused before anything is sent:**
//!   either can change the whole account. ConfigVet never sends a Global API Key's headers (`X-Auth-Key`) at all.
//! - **The token must be active,** as Cloudflare's verify endpoint says, or nothing more is asked.
//! - **Whether it can write cannot be told** without "API Tokens Read", which would let ConfigVet read every other token
//!   too, and which the owner chose not to ask for (22 November 2026). So the report says, before any finding, that
//!   ConfigVet could not confirm the token is read-only. Nothing here ever sends a write to find out.

use crate::{Answer, Client, Got, Sent, Transport};
use serde::Serialize;
use serde_json::Value;
use sv_core::Secret;

/// The one host every request goes to.
pub const HOST: &str = "api.cloudflare.com";
/// The API's own folder on that host; every path is under it.
pub const BASE: &str = "/client/v4";
/// Sent beside the token.
pub const HEADERS: &[&str] = &["Accept: application/json"];
/// The one place ConfigVet takes a token from: the name Cloudflare's own Wrangler reads.
pub const TOKEN_ENV: &str = "CLOUDFLARE_API_TOKEN";

/// The zone settings read, one request each, in this order (reading them all at once is deprecated).
pub const SETTINGS: &[&str] = &[
    "ssl",
    "ssl_automatic_mode",
    "min_tls_version",
    "tls_1_3",
    "always_use_https",
    "security_header",
    "automatic_https_rewrites",
];

/// A zone named as a person names it, such as `stackvet.dev`: letters, digits, `-`, and `.`, with at least one dot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone(pub String);

impl Zone {
    pub fn parse(text: &str) -> Result<Zone, String> {
        let name = text.trim().trim_end_matches('.').to_ascii_lowercase();
        let labels: Vec<&str> = name.split('.').collect();
        let ok = name.len() <= 253
            && labels.len() >= 2
            && labels.iter().all(|l| {
                !l.is_empty()
                    && l.len() <= 63
                    && !l.starts_with('-')
                    && !l.ends_with('-')
                    && l.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
            });
        if ok {
            Ok(Zone(name))
        } else {
            Err(format!(
                "{text:?} is not a zone's name; give the domain as Cloudflare lists it, such as stackvet.dev"
            ))
        }
    }
}

/// What kind of credential it is, from how it begins (Cloudflare's documented prefixes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TokenKind {
    /// `cfut_`: a user's API token.
    User,
    /// `cfat_`: an account's API token.
    Account,
    /// `cfk_`: a Global API Key, which can do anything the account can. Refused.
    GlobalKey,
    /// `cfoat_`: the token Wrangler signs in with, the person's whole account. Refused.
    WranglerSignIn,
    /// No prefix: a token made before Cloudflare gave them one.
    Unknown,
}

pub fn token_kind(token: &str) -> TokenKind {
    if token.starts_with("cfut_") {
        TokenKind::User
    } else if token.starts_with("cfat_") {
        TokenKind::Account
    } else if token.starts_with("cfk_") {
        TokenKind::GlobalKey
    } else if token.starts_with("cfoat_") {
        TokenKind::WranglerSignIn
    } else {
        TokenKind::Unknown
    }
}

/// What was settled about the token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "is", rename_all = "kebab-case")]
pub enum Standing {
    /// Refused before anything was sent, for what it is.
    Refused { why: String },
    /// Cloudflare says it is active; whether it can write could not be told.
    ActiveCouldNotConfirm,
    /// Cloudflare did not say it is active: nothing more was read.
    NotActive { why: String },
    /// Nothing came back about it.
    NotChecked,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenSaid {
    pub kind: TokenKind,
    /// The token as `Secret::redact` shows a key: its first characters and its length.
    pub token: Secret,
    pub standing: Standing,
}

/// One cache rule, as Cloudflare showed it: what it matches and what it does.
#[derive(Debug, Clone, Serialize)]
pub struct CacheRule {
    pub description: String,
    pub expression: String,
    pub enabled: bool,
    /// Whether it makes what it matches eligible for the cache (`Some(true)`), bypasses the cache (`Some(false)`), or
    /// does not say.
    pub cache: Option<bool>,
    /// The edge cache's time-to-live mode, such as `override_origin` or `respect_origin`.
    pub edge_ttl_mode: Option<String>,
    /// The rule's whole `action_parameters`, as shown.
    pub parameters: Value,
}

/// The settings as read. A field left `None` was not read; `unread` says why.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Settings {
    pub zone_id: Option<String>,
    pub plan: Option<String>,
    /// Each setting's `value` as Cloudflare gave it, by its id.
    pub values: std::collections::BTreeMap<String, Value>,
    /// The cache rules; `Some(empty)` when the zone has none.
    pub cache_rules: Option<Vec<CacheRule>>,
}

impl Settings {
    pub fn text(&self, id: &str) -> Option<&str> {
        self.values.get(id).and_then(Value::as_str)
    }
}

/// A whole reading: the token, every request, the settings, and what could not be read.
#[derive(Debug, Clone, Serialize)]
pub struct Reading {
    pub provider: &'static str,
    pub zone: String,
    pub token: TokenSaid,
    pub requests: Vec<Sent>,
    pub settings: Settings,
    /// Each setting that could not be read, and why, in a person's words.
    pub unread: Vec<String>,
    /// Why the reading stopped before the end, when it did: nothing after it was asked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<String>,
}

/// What a refused or missing answer means, in a person's words, with Cloudflare's own message where it gave one.
fn unanswered(what: &str, status: u16, answer: &Answer) -> String {
    let said = serde_json::from_slice::<Value>(&answer.body)
        .ok()
        .and_then(|v| {
            v["errors"]
                .as_array()?
                .iter()
                .filter_map(|e| e["message"].as_str())
                .map(|m| m.chars().take(160).collect::<String>())
                .next()
        })
        .map(|m| format!(" ({m})"))
        .unwrap_or_default();
    format!("{what}: Cloudflare answered {status}{said}")
}

/// Reads `zone`'s settings with `token`, through `transport`.
pub fn read(zone: &Zone, token: &str, transport: &mut dyn Transport) -> Reading {
    let kind = token_kind(token);
    let mut reading = Reading {
        provider: "cloudflare",
        zone: zone.0.clone(),
        token: TokenSaid {
            kind,
            token: Secret::redact(token),
            standing: Standing::NotChecked,
        },
        requests: Vec::new(),
        settings: Settings::default(),
        unread: Vec::new(),
        stopped: None,
    };
    let refused = match kind {
        TokenKind::GlobalKey => Some(
            "it is a Global API Key, which can do anything the account can; make an API token with read access only",
        ),
        TokenKind::WranglerSignIn => Some(
            "it is Wrangler's own sign-in token, which can change the whole account; make an API token with read \
             access only",
        ),
        _ => None,
    };
    if let Some(why) = refused {
        reading.token.standing = Standing::Refused {
            why: why.to_owned(),
        };
        reading.stopped = Some(format!(
            "The token was refused, and nothing was sent: {why}."
        ));
        return reading;
    }
    let mut client = Client::new(HOST, BASE, "Cloudflare", transport, token);
    read_into(&mut client, zone, &mut reading);
    reading.requests = client.requests;
    reading
}

fn read_into(client: &mut Client, zone: &Zone, reading: &mut Reading) {
    // 1. The zone, by its name: its id, its plan, and the account that holds it.
    let (zone_id, account_id) = match client.get(&format!("/zones?name={}", zone.0)) {
        Ok(Got::Json(value, _)) => {
            let found = value["result"].as_array().and_then(|list| list.first());
            match found.and_then(|z| Some((z, z["id"].as_str()?))) {
                Some((z, id)) => {
                    reading.settings.plan = z["plan"]["name"].as_str().map(str::to_owned);
                    (
                        id.to_owned(),
                        z["account"]["id"].as_str().map(str::to_owned),
                    )
                }
                None => {
                    reading.stopped = Some(format!(
                        "Cloudflare showed no zone named {} to this token: it needs Zone Read for that zone",
                        zone.0
                    ));
                    return;
                }
            }
        }
        Ok(Got::Status(status, answer)) => {
            reading.stopped = Some(unanswered("The zone could not be found", status, &answer));
            return;
        }
        Err(why) => {
            reading.stopped = Some(why);
            return;
        }
    };
    reading.settings.zone_id = Some(zone_id.clone());

    // 2. The token: active, or nothing more is asked. An account's token is verified under its account.
    let verify = match (reading.token.kind, &account_id) {
        (TokenKind::Account, Some(account)) => format!("/accounts/{account}/tokens/verify"),
        _ => "/user/tokens/verify".to_owned(),
    };
    reading.token.standing = match client.get(&verify) {
        Ok(Got::Json(value, _)) => match value["result"]["status"].as_str() {
            Some("active") => Standing::ActiveCouldNotConfirm,
            Some(other) => Standing::NotActive {
                why: format!("Cloudflare says the token is {other}"),
            },
            None => Standing::NotActive {
                why: "Cloudflare's answer named no status for the token".to_owned(),
            },
        },
        Ok(Got::Status(status, answer)) => Standing::NotActive {
            why: unanswered("Cloudflare would not verify the token", status, &answer),
        },
        Err(why) => Standing::NotActive { why },
    };
    if let Standing::NotActive { why } = &reading.token.standing {
        reading.stopped = Some(format!("{why}, so cv read nothing more."));
        return;
    }

    // 3. Each setting, one request each.
    for id in SETTINGS {
        match client.get(&format!("/zones/{zone_id}/settings/{id}")) {
            Ok(Got::Json(value, _)) if !value["result"]["value"].is_null() => {
                reading
                    .settings
                    .values
                    .insert((*id).to_owned(), value["result"]["value"].clone());
            }
            Ok(Got::Json(..)) => reading
                .unread
                .push(format!("`{id}`: Cloudflare's answer held no value")),
            Ok(Got::Status(status, answer)) => {
                reading
                    .unread
                    .push(unanswered(&format!("`{id}`"), status, &answer))
            }
            Err(why) => {
                reading.unread.push(format!("`{id}`: {why}"));
                if why.contains("limit") {
                    return;
                }
            }
        }
    }

    // 4. The cache rules: the zone's entrypoint ruleset for its cache settings. None at all is an answer too.
    match client.get(&format!(
        "/zones/{zone_id}/rulesets/phases/http_request_cache_settings/entrypoint"
    )) {
        Ok(Got::Json(value, _)) => {
            let rules = value["result"]["rules"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|r| CacheRule {
                    description: r["description"].as_str().unwrap_or("").to_owned(),
                    expression: r["expression"].as_str().unwrap_or("").to_owned(),
                    enabled: r["enabled"].as_bool().unwrap_or(true),
                    cache: r["action_parameters"]["cache"].as_bool(),
                    edge_ttl_mode: r["action_parameters"]["edge_ttl"]["mode"]
                        .as_str()
                        .map(str::to_owned),
                    parameters: r["action_parameters"].clone(),
                })
                .collect();
            reading.settings.cache_rules = Some(rules);
        }
        Ok(Got::Status(404, _)) => reading.settings.cache_rules = Some(Vec::new()),
        Ok(Got::Status(status, answer)) => {
            reading
                .unread
                .push(unanswered("The cache rules", status, &answer))
        }
        Err(why) => reading.unread.push(format!("The cache rules: {why}")),
    }
}

#[cfg(test)]
#[path = "cloudflare_tests.rs"]
pub(crate) mod cloudflare_tests;
