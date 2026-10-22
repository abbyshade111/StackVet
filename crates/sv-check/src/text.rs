//! Small things done to a piece of text in more than one place, written once (backlog 0461): the first line worth
//! showing, and a cut that says how much it left out.

/// The first line of `text` that is not blank, trimmed, or `no detail` when there is none, so a message built from
/// it never ends in nothing.
pub fn first_line(text: &str) -> String {
    text.lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("no detail")
        .to_owned()
}

/// `text`, or its first `kept` characters and how many more there were: `abcd… (6 more characters)`. A text no
/// longer than `kept` comes back whole.
pub fn cut_with_count(text: &str, kept: usize) -> String {
    let total = text.chars().count();
    if total <= kept {
        return text.to_owned();
    }
    let head: String = text.chars().take(kept).collect();
    format!("{head}… ({} more characters)", total - kept)
}

/// The words a person would read on a page, one space between them: the head, scripts, and styles left out, every
/// tag taken away with its attributes, and the common entities read. The report quotes an answer by them, and a
/// refused sign-in is told by them (backlog 0527).
pub fn visible_words(body: &str) -> String {
    let lower = body.to_ascii_lowercase();
    let mut text = String::with_capacity(body.len());
    let mut i = 0;
    while i < body.len() {
        let rest = &lower[i..];
        let skip_to = ["head", "script", "style"].iter().find_map(|tag| {
            rest.starts_with(&format!("<{tag}"))
                .then(|| {
                    rest.find(&format!("</{tag}>"))
                        .map(|end| end + tag.len() + 3)
                })
                .flatten()
        });
        if let Some(n) = skip_to {
            i += n;
            continue;
        }
        if rest.starts_with('<') {
            i += rest.find('>').map_or(rest.len(), |end| end + 1);
            text.push(' ');
            continue;
        }
        let c = body[i..].chars().next().expect("a character");
        text.push(c);
        i += c.len_utf8();
    }
    let text = text
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ");
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;

/// `text` with every character a reader cannot see written out as `\u{…}`: controls, zero-width characters and joiners,
/// the controls that override the direction of text, the byte-order mark, the soft hyphen, variation selectors, and
/// Unicode tag characters (backlog 0623). A prompt the AI checks sent carries some of them on purpose, and the report
/// shows it as text, never as what it would do.
pub fn invisible_written_out(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        let hidden = c.is_control()
            || matches!(c,
                '\u{00AD}' | '\u{200B}'..='\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2060}'..='\u{2064}'
                | '\u{2066}'..='\u{2069}' | '\u{FE00}'..='\u{FE0F}' | '\u{FEFF}' | '\u{E0000}'..='\u{E007F}');
        if hidden {
            out.push_str(&format!("\\u{{{:04X}}}", u32::from(c)));
        } else {
            out.push(c);
        }
    }
    out
}
