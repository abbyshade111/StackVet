//! A PDF this module wrote, read back with nothing but the standard library: for the tests that hold what the file
//! says, and for nobody else. It follows the table of where each object starts, so a file whose table is wrong does
//! not read, and it reads the text the way the layout draws it: one string at a time, each with its font, size, and
//! place.

use super::fonts::{self, Font};

/// One string drawn on a page.
#[derive(Debug, Clone)]
pub struct Drawn {
    pub text: String,
    pub font: Font,
    pub size: f32,
    pub x: f32,
    pub y: f32,
}

impl Drawn {
    /// How wide it is drawn.
    pub fn width(&self) -> f32 {
        fonts::width(&self.text, self.font, self.size)
    }
}

/// One page as read back.
#[derive(Debug, Clone)]
pub struct PageRead {
    pub size: (f32, f32),
    pub drawn: Vec<Drawn>,
    /// The addresses its links go to.
    pub links: Vec<String>,
}

/// The file as read back.
#[derive(Debug, Clone)]
pub struct Read {
    pub pages: Vec<PageRead>,
    /// Every bookmark's title, in order, at every depth.
    pub bookmarks: Vec<String>,
}

impl Read {
    /// Every string on every page, in order, a space between them.
    pub fn text(&self) -> String {
        self.pages
            .iter()
            .flat_map(|p| p.drawn.iter().map(|d| d.text.as_str()))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// The objects by number, found through the table at the end of the file.
fn objects(pdf: &str) -> Result<Vec<String>, String> {
    let start: usize = pdf
        .rsplit("startxref\n")
        .next()
        .and_then(|s| s.lines().next())
        .and_then(|s| s.trim().parse().ok())
        .ok_or("no startxref")?;
    let table = pdf.get(start..).ok_or("startxref past the end")?;
    let mut lines = table.lines();
    if lines.next() != Some("xref") {
        return Err("startxref does not point at the table".into());
    }
    let count: usize = lines
        .next()
        .and_then(|l| l.split(' ').nth(1))
        .and_then(|n| n.parse().ok())
        .ok_or("no count")?;
    let offsets: Vec<usize> = lines
        .take(count)
        .skip(1)
        .map(|l| l[..10].parse().map_err(|_| format!("bad entry {l:?}")))
        .collect::<Result<_, _>>()?;
    offsets
        .iter()
        .enumerate()
        .map(|(i, at)| {
            let head = format!("{} 0 obj\n", i + 1);
            let body = pdf.get(*at..).ok_or("offset past the end")?;
            let body = body
                .strip_prefix(&head)
                .ok_or_else(|| format!("object {} is not where the table says", i + 1))?;
            let end = body.find("\nendobj\n").ok_or("an object does not end")?;
            Ok(body[..end].to_owned())
        })
        .collect()
}

/// The object numbers `N 0 R` in `text`, in order.
fn refs(text: &str) -> Vec<usize> {
    let t: Vec<&str> = text.split_whitespace().collect();
    t.windows(3)
        .filter(|w| w[1] == "0" && w[2].starts_with('R'))
        .filter_map(|w| w[0].trim_start_matches('[').parse().ok())
        .collect()
}

/// What follows `key` in a dictionary, up to the next space.
fn after<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let at = text.find(key)? + key.len();
    text[at..].split_whitespace().next()
}

/// A UTF-16 hexadecimal title, as the writer writes one.
fn title(hex: &str) -> String {
    let hex = hex.trim_start_matches("<FEFF").trim_end_matches('>');
    let units: Vec<u16> = (0..hex.len() / 4)
        .filter_map(|i| u16::from_str_radix(&hex[i * 4..i * 4 + 4], 16).ok())
        .collect();
    String::from_utf16_lossy(&units)
}

/// Reads `pdf` back.
pub fn read(pdf: &str) -> Result<Read, String> {
    if !pdf.starts_with("%PDF-1.4\n") || !pdf.ends_with("%%EOF\n") {
        return Err("not a PDF this writer made".into());
    }
    let objs = objects(pdf)?;
    let get = |n: usize| objs.get(n - 1).ok_or_else(|| format!("no object {n}"));
    let trailer = pdf.rsplit("trailer\n").next().ok_or("no trailer")?;
    let root: usize = after(trailer, "/Root ")
        .and_then(|n| n.parse().ok())
        .ok_or("no root")?;
    let catalog = get(root)?;
    let pages_root: usize = after(catalog, "/Pages ")
        .and_then(|n| n.parse().ok())
        .ok_or("no pages")?;
    let kids_dict = get(pages_root)?;
    let kids = &kids_dict[kids_dict.find("/Kids [").ok_or("no kids")?..];
    let kids = &kids[..kids.find(']').ok_or("kids do not end")?];
    let mut pages = Vec::new();
    for id in refs(kids) {
        let page = get(id)?;
        let mb = &page[page.find("/MediaBox [0 0 ").ok_or("no media box")? + 15..];
        let mut nums = mb.split([' ', ']']).filter_map(|n| n.parse::<f32>().ok());
        let size = (
            nums.next().ok_or("no width")?,
            nums.next().ok_or("no height")?,
        );
        let contents: usize = after(page, "/Contents ")
            .and_then(|n| n.parse().ok())
            .ok_or("no contents")?;
        let stream = get(contents)?;
        let length: usize = after(stream, "/Length ")
            .and_then(|n| n.parse().ok())
            .ok_or("no length")?;
        let body = &stream[stream.find("stream\n").ok_or("no stream")? + 7..];
        let ops = body.get(..length).ok_or("stream shorter than its length")?;
        if !body[length..].starts_with("\nendstream") {
            return Err("a stream's length is wrong".into());
        }
        let mut drawn = Vec::new();
        let (mut font, mut size_pt, mut x, mut y) = (Font::Regular, 0.0, 0.0, 0.0);
        let tokens: Vec<&str> = ops.split_whitespace().collect();
        for (i, t) in tokens.iter().enumerate() {
            match *t {
                "Tf" if i >= 2 => {
                    font = Font::from_key(tokens[i - 2].trim_start_matches('/'))
                        .ok_or("unknown font")?;
                    size_pt = tokens[i - 1].parse().map_err(|_| "bad size")?;
                }
                "Td" if i >= 2 => {
                    x = tokens[i - 2].parse().map_err(|_| "bad x")?;
                    y = tokens[i - 1].parse().map_err(|_| "bad y")?;
                }
                "Tj" if i >= 1 => {
                    let hex = tokens[i - 1].trim_start_matches('<').trim_end_matches('>');
                    let text: String = (0..hex.len() / 2)
                        .filter_map(|k| u8::from_str_radix(&hex[k * 2..k * 2 + 2], 16).ok())
                        .map(fonts::decode)
                        .collect();
                    drawn.push(Drawn {
                        text,
                        font,
                        size: size_pt,
                        x,
                        y,
                    });
                }
                _ => {}
            }
        }
        let mut links = Vec::new();
        if let Some(at) = page.find("/Annots [") {
            let list = &page[at..];
            for a in refs(&list[..list.find(']').unwrap_or(list.len())]) {
                let annot = get(a)?;
                if let Some(u) = annot.find("/URI (") {
                    let rest = &annot[u + 6..];
                    links.push(
                        rest[..rest.find(") >>").unwrap_or(rest.len())]
                            .replace("\\(", "(")
                            .replace("\\)", ")"),
                    );
                }
            }
        }
        pages.push(PageRead { size, drawn, links });
    }
    let mut bookmarks = Vec::new();
    for o in &objs {
        if o.contains("/Dest [")
            && let Some(t) = after(o, "/Title ")
        {
            bookmarks.push(title(t));
        }
    }
    Ok(Read { pages, bookmarks })
}
