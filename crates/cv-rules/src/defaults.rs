//! ConfigVet's table of defaults, `data/config-defaults.json` (ADR-157, decision 3), read through
//! `sv_frameworks::data` as every run-time read of `data/` is (ADR-036).
//!
//! Each entry says what a framework or platform does when a setting is left out, for the versions it names, and the
//! vendor's page it was read from. A rule asks [`Table::holding`] for a default by its id and the version in use, and
//! gets nothing when the version is outside what was read: the setting is then not assessed.

use serde::{Deserialize, Serialize};

pub const FILE: &str = "config-defaults.json";

#[derive(Debug, Clone, Deserialize)]
pub struct Table {
    pub defaults: Vec<KnownDefault>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct KnownDefault {
    pub id: String,
    pub subject: String,
    pub setting: String,
    pub versions: Versions,
    pub default: String,
    pub safe: bool,
    pub why: String,
    pub sources: Vec<Source>,
}

/// The versions a default was read for. One of three forms: a range of major versions of a package
/// (`from_major`..=`to_major`), a range of `major.minor` releases (`from`..=`to`), or a list of an image's majors.
/// `any` is for a platform whose default the vendor states without a version.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Versions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_major: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_major: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub majors: Vec<u32>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub any: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Source {
    pub url: String,
    pub says: String,
    pub read: String,
}

/// The version in use, as far as the folder shows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// No version is involved (a platform default stated for every version).
    Any,
    Major(u32),
    /// Every release from the first `major.minor` up to, not including, the second: what a requirement such as
    /// `>=4.2,<5.0` allows. A pin (`==5.1.3`) is the range of that one minor release.
    Releases((u32, u32), (u32, u32)),
}

impl Versions {
    fn hold_for(&self, version: Version) -> bool {
        if self.any {
            return true;
        }
        match version {
            Version::Any => false,
            Version::Major(major) => {
                self.majors.contains(&major)
                    || matches!((self.from_major, self.to_major), (Some(a), Some(b)) if (a..=b).contains(&major))
            }
            Version::Releases(low, high) => {
                let (Some(from), Some(to)) = (
                    self.from.as_deref().and_then(major_minor),
                    self.to.as_deref().and_then(major_minor),
                ) else {
                    return false;
                };
                // Every release allowed must be one that was read: from `from` up to and including `to`.
                low >= from && high <= (to.0, to.1 + 1) && low < high
            }
        }
    }
}

/// `"4.2"` as `(4, 2)`.
pub fn major_minor(text: &str) -> Option<(u32, u32)> {
    let mut parts = text.trim().split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().map_or(Some(0), |m| m.parse().ok())?;
    Some((major, minor))
}

impl Table {
    /// The table from the data folder, or why it could not be read.
    pub fn load() -> Result<Table, String> {
        let path = sv_frameworks::data::file(FILE);
        sv_frameworks::data::read_json(&path).map_err(|e| format!("{e:#}"))
    }

    pub fn parse(text: &str) -> Result<Table, String> {
        serde_json::from_str(text).map_err(|e| e.to_string())
    }

    /// The default named `id`, when the table holds it for `version`.
    pub fn holding(&self, id: &str, version: Version) -> Option<&KnownDefault> {
        self.defaults
            .iter()
            .find(|d| d.id == id)
            .filter(|d| d.versions.hold_for(version))
    }

    /// The default named `id`, whatever the version: for saying which versions were read.
    pub fn get(&self, id: &str) -> Option<&KnownDefault> {
        self.defaults.iter().find(|d| d.id == id)
    }
}
