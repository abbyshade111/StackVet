//! The reports' Markdown read into blocks the layout can place. Only the Markdown `sv`'s own renderers write is read
//! (`crate::markdown`): headings, paragraphs, lists, tables, fenced code, the `<details>` and `<summary>` lines that fold
//! a long list away on GitHub, bold, italic, code spans, links, and the escapes `markdown::inert` and `markdown::cell`
//! write. Anything else is kept as the text it is, never dropped: what is read here is what the PDF says.

/// How a run of text is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Look {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
}

/// A run of text in one look, and the address it links to, if any.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub text: String,
    pub look: Look,
    pub link: Option<String>,
}

/// One block of the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Heading {
        level: usize,
        text: Vec<Span>,
    },
    Paragraph(Vec<Span>),
    /// One item of a list: what it is marked with (`•` or `3.`), how deep it is, and its text.
    Item {
        marker: String,
        depth: usize,
        text: Vec<Span>,
    },
    Table {
        header: Vec<Vec<Span>>,
        /// Whether each column is aligned to the right, as `---:` says.
        right: Vec<bool>,
        rows: Vec<Vec<Vec<Span>>>,
    },
    /// Lines shown as they are, in the fixed-width font.
    Code(Vec<String>),
}

impl Block {
    /// The text of every span in the block, for tests.
    pub fn text(&self) -> String {
        let join = |spans: &[Span]| spans.iter().map(|s| s.text.as_str()).collect::<String>();
        match self {
            Block::Heading { text, .. } | Block::Paragraph(text) | Block::Item { text, .. } => {
                join(text)
            }
            Block::Table { header, rows, .. } => std::iter::once(header)
                .chain(rows)
                .flat_map(|row| row.iter().map(|c| join(c)))
                .collect::<Vec<_>>()
                .join(" "),
            Block::Code(lines) => lines.join("\n"),
        }
    }
}

/// A fence that opens fenced code: three or more backticks at the start of a line.
fn fence_of(line: &str) -> Option<usize> {
    let n = line.chars().take_while(|c| *c == '`').count();
    (n >= 3).then_some(n)
}

/// A list item's marker and text, and how far it is indented.
fn item_of(line: &str) -> Option<(usize, String, &str)> {
    let indent = line.chars().take_while(|c| *c == ' ').count();
    let rest = &line[indent..];
    if let Some(text) = rest.strip_prefix("- ").or_else(|| rest.strip_prefix("* ")) {
        return Some((indent, "\u{2022}".to_owned(), text));
    }
    let digits = rest.chars().take_while(char::is_ascii_digit).count();
    if digits > 0
        && digits < 4
        && let Some(text) = rest[digits..].strip_prefix(". ")
    {
        return Some((indent, rest[..=digits].to_owned(), text));
    }
    None
}

/// The heading level and text of a line, if it is a heading.
fn heading_of(line: &str) -> Option<(usize, &str)> {
    let n = line.chars().take_while(|c| *c == '#').count();
    if (1..=6).contains(&n)
        && let Some(text) = line[n..].strip_prefix(' ')
    {
        return Some((n, text.trim_end_matches(['#', ' '])));
    }
    None
}

/// The cells of a table row, split at each pipe that is not escaped.
fn cells_of(line: &str) -> Vec<String> {
    let line = line.trim();
    let line = line.strip_prefix('|').unwrap_or(line);
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                cell.push(c);
                if let Some(next) = chars.next() {
                    cell.push(next);
                }
            }
            '|' => cells.push(std::mem::take(&mut cell)),
            _ => cell.push(c),
        }
    }
    if !cell.trim().is_empty() {
        cells.push(cell);
    }
    cells.into_iter().map(|c| c.trim().to_owned()).collect()
}

/// Whether a line is a table's separator row, `|---|---:|`.
fn separator(line: &str) -> bool {
    let cells = cells_of(line);
    !cells.is_empty()
        && cells.iter().all(|c| {
            let c = c.trim_matches(':');
            !c.is_empty() && c.chars().all(|ch| ch == '-')
        })
}

/// The document's blocks, in order.
pub fn blocks(markdown: &str) -> Vec<Block> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut out = Vec::new();
    let mut paragraph: Vec<&str> = Vec::new();
    let flush = |paragraph: &mut Vec<&str>, out: &mut Vec<Block>| {
        if !paragraph.is_empty() {
            out.push(Block::Paragraph(spans(&paragraph.join(" "))));
            paragraph.clear();
        }
    };
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            flush(&mut paragraph, &mut out);
            i += 1;
            continue;
        }
        if let Some(n) = fence_of(trimmed) {
            flush(&mut paragraph, &mut out);
            let mut code = Vec::new();
            i += 1;
            while i < lines.len() {
                let l = lines[i].trim_end();
                if fence_of(l.trim()).is_some_and(|m| m >= n) && l.trim().chars().all(|c| c == '`')
                {
                    break;
                }
                code.push(l.to_owned());
                i += 1;
            }
            out.push(Block::Code(code));
            i += 1;
            continue;
        }
        if trimmed == "<details>" || trimmed == "</details>" {
            flush(&mut paragraph, &mut out);
            i += 1;
            continue;
        }
        if let Some(inner) = trimmed
            .strip_prefix("<summary>")
            .and_then(|s| s.strip_suffix("</summary>"))
        {
            flush(&mut paragraph, &mut out);
            let mut text = spans(inner);
            for s in &mut text {
                s.look.bold = true;
            }
            out.push(Block::Paragraph(text));
            i += 1;
            continue;
        }
        if let Some((level, text)) = heading_of(trimmed) {
            flush(&mut paragraph, &mut out);
            out.push(Block::Heading {
                level,
                text: spans(text),
            });
            i += 1;
            continue;
        }
        if trimmed.starts_with('|') && i + 1 < lines.len() && separator(lines[i + 1]) {
            flush(&mut paragraph, &mut out);
            let header: Vec<Vec<Span>> = cells_of(trimmed).iter().map(|c| spans(c)).collect();
            let right = cells_of(lines[i + 1])
                .iter()
                .map(|c| c.ends_with(':') && !c.starts_with(':'))
                .collect();
            i += 2;
            let mut rows = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('|') {
                rows.push(cells_of(lines[i]).iter().map(|c| spans(c)).collect());
                i += 1;
            }
            out.push(Block::Table {
                header,
                right,
                rows,
            });
            continue;
        }
        if let Some((indent, marker, text)) = item_of(line) {
            flush(&mut paragraph, &mut out);
            let mut text = text.to_owned();
            i += 1;
            // Lines indented under the item, that start nothing of their own, go on with it.
            while i < lines.len() {
                let next = lines[i];
                let next_indent = next.chars().take_while(|c| *c == ' ').count();
                if next.trim().is_empty()
                    || next_indent <= indent
                    || item_of(next).is_some()
                    || fence_of(next.trim()).is_some()
                {
                    break;
                }
                text.push(' ');
                text.push_str(next.trim());
                i += 1;
            }
            out.push(Block::Item {
                marker,
                depth: indent / 2,
                text: spans(&text),
            });
            continue;
        }
        if let Some(quoted) = trimmed
            .strip_prefix("> ")
            .or_else(|| trimmed.strip_prefix('>'))
        {
            flush(&mut paragraph, &mut out);
            let mut text = spans(quoted);
            for s in &mut text {
                s.look.italic = true;
            }
            out.push(Block::Item {
                marker: String::new(),
                depth: 1,
                text,
            });
            i += 1;
            continue;
        }
        paragraph.push(trimmed);
        i += 1;
    }
    flush(&mut paragraph, &mut out);
    out
}

/// The entities `markdown::inert` and the renderers write, as the characters they stand for.
const ENTITIES: [(&str, char); 5] = [
    ("&lt;", '<'),
    ("&gt;", '>'),
    ("&amp;", '&'),
    ("&quot;", '"'),
    ("&#39;", '\''),
];

/// A paragraph's text as spans: code spans, bold, italic, links, escapes, and the `<strong>` a summary line uses.
pub fn spans(text: &str) -> Vec<Span> {
    let chars: Vec<char> = text.chars().collect();
    let mut out: Vec<Span> = Vec::new();
    let mut look = Look::default();
    let mut link: Option<String> = None;
    // Where a link's text ends and its address begins, when one is open.
    let mut link_end: Option<(usize, usize)> = None;
    let mut buf = String::new();
    let push = |out: &mut Vec<Span>, buf: &mut String, look: Look, link: &Option<String>| {
        if buf.is_empty() {
            return;
        }
        match out.last_mut() {
            Some(last) if last.look == look && last.link == *link => last.text.push_str(buf),
            _ => out.push(Span {
                text: buf.clone(),
                look,
                link: link.clone(),
            }),
        }
        buf.clear();
    };
    let run_at = |i: usize| chars[i..].iter().take_while(|c| **c == '`').count();
    let starts = |i: usize, s: &str| {
        chars[i..]
            .iter()
            .take(s.chars().count())
            .copied()
            .eq(s.chars())
    };
    let mut i = 0;
    while i < chars.len() {
        if let Some((end, after)) = link_end
            && i == end
        {
            push(&mut out, &mut buf, look, &link);
            link = None;
            link_end = None;
            i = after;
            continue;
        }
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() && chars[i + 1].is_ascii_punctuation() {
            buf.push(chars[i + 1]);
            i += 2;
            continue;
        }
        if c == '&'
            && let Some((name, ch)) = ENTITIES.iter().find(|(name, _)| starts(i, name))
        {
            buf.push(*ch);
            i += name.len();
            continue;
        }
        if c == '`' {
            let n = run_at(i);
            let mut j = i + n;
            let mut close = None;
            while j < chars.len() {
                if chars[j] == '`' {
                    let m = run_at(j);
                    if m == n {
                        close = Some(j);
                        break;
                    }
                    j += m;
                } else {
                    j += 1;
                }
            }
            if let Some(j) = close {
                push(&mut out, &mut buf, look, &link);
                let inner: String = chars[i + n..j].iter().collect();
                let inner = if inner.len() > 1 && inner.starts_with(' ') && inner.ends_with(' ') {
                    inner[1..inner.len() - 1].to_owned()
                } else {
                    inner
                };
                buf.push_str(&inner);
                push(&mut out, &mut buf, Look { code: true, ..look }, &link);
                i = j + n;
                continue;
            }
            for _ in 0..n {
                buf.push('`');
            }
            i += n;
            continue;
        }
        if starts(i, "<strong>") || starts(i, "</strong>") {
            push(&mut out, &mut buf, look, &link);
            look.bold = starts(i, "<strong>");
            i += if look.bold { 8 } else { 9 };
            continue;
        }
        if starts(i, "**") {
            push(&mut out, &mut buf, look, &link);
            look.bold = !look.bold;
            i += 2;
            continue;
        }
        if c == '*' {
            // Emphasis opens before a word and closes after one; a star with spaces on both sides is a star.
            let before = i.checked_sub(1).map(|k| chars[k]);
            let after = chars.get(i + 1).copied();
            let opens = !look.italic && after.is_some_and(|a| !a.is_whitespace());
            let closes = look.italic && before.is_some_and(|b| !b.is_whitespace());
            if opens || closes {
                push(&mut out, &mut buf, look, &link);
                look.italic = !look.italic;
                i += 1;
                continue;
            }
        }
        if c == '['
            && link.is_none()
            && let Some((end, address, after)) = link_at(&chars, i)
        {
            push(&mut out, &mut buf, look, &link);
            link = Some(address);
            link_end = Some((end, after));
            i += 1;
            continue;
        }
        buf.push(c);
        i += 1;
    }
    push(&mut out, &mut buf, look, &link);
    out
}

/// A link opened by the `[` at `i`: where its text ends, its address, and where the text after it starts.
fn link_at(chars: &[char], i: usize) -> Option<(usize, String, usize)> {
    let mut depth = 0;
    let mut j = i;
    while j < chars.len() {
        match chars[j] {
            '\\' => j += 1,
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        j += 1;
    }
    if j >= chars.len() || chars.get(j + 1) != Some(&'(') {
        return None;
    }
    let close = chars[j + 2..].iter().position(|c| *c == ')')? + j + 2;
    let address: String = chars[j + 2..close].iter().collect();
    if address.is_empty() || address.contains(char::is_whitespace) {
        return None;
    }
    Some((j, address, close + 1))
}
