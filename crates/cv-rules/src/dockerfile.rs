//! A Dockerfile, read for what a container built from it starts with: each stage's base, its `ENV` settings, and its
//! `USER`, each with its line.
//!
//! Only the instructions the rules need are read (`FROM`, `ENV`, `USER`); the rest are passed over. A line ending in
//! a backslash runs on to the next, and a comment line is skipped, as Docker reads them. A value that names a
//! variable (`$NODE_ENV`, `${USER}`) is kept as written, and the rules treat it as unknown, since what it stands for is
//! decided at build time.

/// One `KEY=value` (or `KEY value`) set by an `ENV`, with the line its instruction starts on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Setting {
    pub name: String,
    pub value: String,
    pub line: usize,
}

/// One stage: from `FROM` to the next `FROM`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage {
    /// The image named after `FROM`, without `--platform=`.
    pub from: String,
    pub from_line: usize,
    /// The name after `AS`, lowercased, as later stages name it.
    pub alias: Option<String>,
    pub env: Vec<Setting>,
    /// The last `USER` in the stage, as written, with its line.
    pub user: Option<(String, usize)>,
}

/// What the final stage starts with, after following the stages it is built from in this file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effective {
    /// The image the chain of stages starts from, outside this file, with its `FROM` line.
    pub base: String,
    pub base_line: usize,
    /// Each variable the chain sets, the last setting winning; `final_stage` says whether that setting is in the
    /// final stage itself.
    pub env: Vec<(Setting, bool)>,
    pub user: Option<(String, usize)>,
}

impl Effective {
    pub fn env(&self, name: &str) -> Option<&(Setting, bool)> {
        self.env.iter().rev().find(|(s, _)| s.name == name)
    }
}

/// Reads a Dockerfile's stages, or says why it could not.
pub fn parse(text: &str) -> Result<Vec<Stage>, String> {
    let mut stages: Vec<Stage> = Vec::new();
    for (line, instruction) in logical_lines(text) {
        let (word, rest) = match instruction.split_once(char::is_whitespace) {
            Some((w, r)) => (w.to_ascii_uppercase(), r.trim()),
            None => (instruction.to_ascii_uppercase(), ""),
        };
        match word.as_str() {
            "FROM" => {
                let words: Vec<&str> = rest
                    .split_whitespace()
                    .filter(|w| !w.starts_with("--"))
                    .collect();
                let Some(from) = words.first() else {
                    return Err(format!("the FROM on line {line} names no image"));
                };
                let alias = match words.as_slice() {
                    [_, as_word, name, ..] if as_word.eq_ignore_ascii_case("as") => {
                        Some(name.to_ascii_lowercase())
                    }
                    _ => None,
                };
                stages.push(Stage {
                    from: (*from).to_owned(),
                    from_line: line,
                    alias,
                    env: Vec::new(),
                    user: None,
                });
            }
            "ENV" | "USER" if stages.is_empty() => {
                return Err(format!("line {line} comes before the first FROM"));
            }
            "ENV" => {
                let stage = stages.last_mut().expect("checked above");
                stage.env.extend(env_settings(rest, line));
            }
            "USER" => {
                let stage = stages.last_mut().expect("checked above");
                stage.user = Some((rest.to_owned(), line));
            }
            _ => {}
        }
    }
    if stages.is_empty() {
        return Err("the file has no FROM, so it is not a Dockerfile cv can read".to_owned());
    }
    Ok(stages)
}

/// What the final stage starts with. A stage built `FROM` an earlier stage's name carries that stage's settings, as
/// Docker builds it; the chain ends at the first image that is not a stage of this file.
pub fn effective(stages: &[Stage]) -> Effective {
    let last = stages.len() - 1;
    let mut chain = vec![last];
    let mut at = last;
    while let Some(earlier) = stages[..at]
        .iter()
        .rposition(|s| s.alias.as_deref() == Some(stages[at].from.to_ascii_lowercase().as_str()))
    {
        chain.push(earlier);
        at = earlier;
    }
    let first = stages[*chain.last().expect("never empty")].clone();
    let mut env = Vec::new();
    let mut user = None;
    for &index in chain.iter().rev() {
        let stage = &stages[index];
        for setting in &stage.env {
            env.push((setting.clone(), index == last));
        }
        if stage.user.is_some() {
            user.clone_from(&stage.user);
        }
    }
    Effective {
        base: first.from,
        base_line: first.from_line,
        env,
        user,
    }
}

/// The file's instructions, a backslash at a line's end joining it to the next, each with the line it starts on.
fn logical_lines(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut current: Option<(usize, String)> = None;
    for (index, raw) in text.lines().enumerate() {
        let trimmed = raw.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        let (body, continues) = match trimmed.strip_suffix('\\') {
            Some(body) => (body.trim_end(), true),
            None => (trimmed, false),
        };
        match current.as_mut() {
            Some((_, text)) => {
                text.push(' ');
                text.push_str(body);
            }
            None if !body.is_empty() => current = Some((index + 1, body.to_owned())),
            None => {}
        }
        if !continues && let Some(done) = current.take() {
            out.push(done);
        }
    }
    out.extend(current);
    out
}

/// `KEY=value KEY2="two words"`, or the older `KEY value with spaces`.
fn env_settings(rest: &str, line: usize) -> Vec<Setting> {
    let first = rest.split_whitespace().next().unwrap_or_default();
    if !first.contains('=') {
        return match rest.split_once(char::is_whitespace) {
            Some((name, value)) => vec![Setting {
                name: name.to_owned(),
                value: unquote(value.trim()),
                line,
            }],
            None => Vec::new(),
        };
    }
    let mut out = Vec::new();
    for word in words(rest) {
        if let Some((name, value)) = word.split_once('=') {
            out.push(Setting {
                name: name.to_owned(),
                value: unquote(value),
                line,
            });
        }
    }
    out
}

/// Splits on spaces outside quotes, keeping the quotes for `unquote`.
fn words(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut word = String::new();
    let mut quote: Option<char> = None;
    for c in text.chars() {
        match (quote, c) {
            (None, '"' | '\'') => {
                quote = Some(c);
                word.push(c);
            }
            (Some(q), _) if c == q => {
                quote = None;
                word.push(c);
            }
            (None, c) if c.is_whitespace() => {
                if !word.is_empty() {
                    out.push(std::mem::take(&mut word));
                }
            }
            _ => word.push(c),
        }
    }
    if !word.is_empty() {
        out.push(word);
    }
    out
}

fn unquote(value: &str) -> String {
    let mut out = String::new();
    let mut quote: Option<char> = None;
    for c in value.chars() {
        match (quote, c) {
            (None, '"' | '\'') => quote = Some(c),
            (Some(q), _) if c == q => quote = None,
            _ => out.push(c),
        }
    }
    out
}
