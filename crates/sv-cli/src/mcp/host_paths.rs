//! Paths in the person's own form, when `sv` runs in a Linux container on Windows (backlog 0375, ADR-153).
//!
//! `sv connect --docker` on Windows shows the app's folder inside the container at a Linux-style path
//! (`C:\Users\you\app` at `/c/Users/you/app`) and tells the container the Windows one in `SV_HOST_FOLDER`. Every
//! message the server answers with has each path under that folder written under the Windows one instead, so a
//! report's links and a finding's file open in the AI tool; and every message it is sent has each path under the
//! Windows folder read as the one inside. Only the strings of a message change, never its shape, and nothing changes
//! when `SV_HOST_FOLDER` is not set.

use serde_json::Value;

/// The two forms of the app's folder.
pub(super) struct HostPaths {
    /// Inside the container: `/c/Users/you/app`.
    inside: String,
    /// On the person's computer: `C:\Users\you\app`.
    host: String,
}

/// What ends a path written in a sentence: a space, a quote or a backtick, a bracket, a comma or a semicolon, and the
/// colon before a line number.
fn ends_a_path(c: char) -> bool {
    c.is_whitespace() || "\"'`()[]<>,;:".contains(c)
}

/// What a path cannot start right after: a letter, digit, or one of its own separators, which would make the match
/// the end of a longer path.
fn inside_a_word(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '/' | '\\' | '_' | '-' | '.')
}

impl HostPaths {
    /// From the environment the container was started with, for a server whose root is `inside`; `None` when it was
    /// not told a Windows folder.
    pub(super) fn from_env(inside: &std::path::Path) -> Option<Self> {
        let host = std::env::var(crate::connect::HOST_FOLDER_ENV).ok()?;
        Self::new(&inside.display().to_string(), &host)
    }

    /// Two forms taken as they are, for a test on a computer whose folders are not Windows ones.
    #[cfg(test)]
    pub(super) fn given(inside: &str, host: &str) -> Self {
        HostPaths {
            inside: inside.trim_end_matches('/').to_owned(),
            host: host.to_owned(),
        }
    }

    /// `None` unless `host` is a Windows folder whose Linux-style form is `inside`.
    pub(super) fn new(inside: &str, host: &str) -> Option<Self> {
        let host = host.trim_end_matches(['\\', '/']);
        let inside = inside.trim_end_matches('/');
        (crate::connect::inside_path(host)?.eq_ignore_ascii_case(inside)).then(|| HostPaths {
            inside: inside.to_owned(),
            host: host.replace('/', "\\"),
        })
    }

    /// `text` with each path under the folder inside written under the person's: `/c/Users/you/app/a/b.html` as
    /// `C:\Users\you\app\a\b.html`, and a `file://` address as `file:///C:/Users/you/app/a/b.html`.
    pub(super) fn outward(&self, text: &str) -> String {
        let forward_host = self.host.replace('\\', "/");
        let uri = format!("file://{}", self.inside);
        let text = replace_paths(text, &uri, |rest| format!("file:///{forward_host}{rest}"));
        replace_paths(&text, &self.inside, |rest| {
            format!("{}{}", self.host, rest.replace('/', "\\"))
        })
    }

    /// `text` with each path under the person's folder, written with `\` or `/`, read as the one inside.
    pub(super) fn inward(&self, text: &str) -> String {
        let forward = self.host.replace('\\', "/");
        let uri = format!("file:///{forward}");
        let mut out = replace_paths(text, &uri, |rest| format!("file://{}{rest}", self.inside));
        for host in [self.host.as_str(), forward.as_str()] {
            out = replace_paths(&out, host, |rest| {
                format!("{}{}", self.inside, rest.replace('\\', "/"))
            });
        }
        out
    }

    /// Every string in a message, `outward` or `inward`; the message unchanged when it is not JSON.
    pub(super) fn line(&self, line: &str, outward: bool) -> String {
        let Ok(mut value) = serde_json::from_str::<Value>(line) else {
            return line.to_owned();
        };
        self.walk(&mut value, outward);
        value.to_string()
    }

    fn walk(&self, value: &mut Value, outward: bool) {
        match value {
            Value::String(s) => {
                *s = if outward {
                    self.outward(s)
                } else {
                    self.inward(s)
                };
            }
            Value::Array(items) => items.iter_mut().for_each(|v| self.walk(v, outward)),
            Value::Object(map) => map.values_mut().for_each(|v| self.walk(v, outward)),
            _ => {}
        }
    }
}

/// `text` with each `prefix` that starts a path (nothing of a word just before it) and is the whole folder (a
/// separator or the path's end just after it) replaced by `with` of the rest of its path. The drive letter of a
/// Windows prefix is read in either case.
fn replace_paths(text: &str, prefix: &str, with: impl Fn(&str) -> String) -> String {
    let lower_text = text.to_ascii_lowercase();
    let lower_prefix = prefix.to_ascii_lowercase();
    let windows = prefix.as_bytes().get(1) == Some(&b':');
    let mut out = String::new();
    let mut from = 0;
    while let Some(found) = find(&lower_text, text, &lower_prefix, prefix, from, windows) {
        let before = text[..found].chars().next_back();
        let after_at = found + prefix.len();
        let after = text[after_at..].chars().next();
        let starts = before.is_none_or(|c| !inside_a_word(c)) || prefix.starts_with("file:");
        let whole = after.is_none_or(|c| matches!(c, '/' | '\\') || ends_a_path(c));
        if !(starts && whole) {
            out.push_str(&text[from..after_at]);
            from = after_at;
            continue;
        }
        let end = text[after_at..]
            .find(ends_a_path)
            .map_or(text.len(), |i| after_at + i);
        out.push_str(&text[from..found]);
        out.push_str(&with(&text[after_at..end]));
        from = end;
    }
    out.push_str(&text[from..]);
    out
}

/// The next place `prefix` is in `text` from `from`: exactly, or, for a Windows prefix, with the drive letter in either
/// case. ASCII only is folded, so the byte positions of the two texts agree.
fn find(
    lower_text: &str,
    text: &str,
    lower_prefix: &str,
    prefix: &str,
    from: usize,
    windows: bool,
) -> Option<usize> {
    if windows {
        // Only the drive letter may differ in case; the rest must be as written.
        let mut at = from;
        while let Some(i) = lower_text[at..].find(lower_prefix).map(|i| at + i) {
            if text[i + 1..].starts_with(&prefix[1..]) {
                return Some(i);
            }
            at = i + 1;
        }
        None
    } else {
        text[from..].find(prefix).map(|i| from + i)
    }
}

#[cfg(test)]
mod tests;
