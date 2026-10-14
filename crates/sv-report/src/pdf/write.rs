//! The bytes of a PDF: its objects, the table of where each starts, and the trailer. Nothing here knows about
//! reports; it takes finished pages (drawing operators as text) and writes them out, PDF 1.4 with the five standard
//! fonts and nothing else.
//!
//! Every byte is ASCII: the pages' text is written as hexadecimal strings and their streams are not compressed, so
//! the file is text a test can read back with nothing but the standard library, and it travels everywhere the other
//! report files do. The cost is size, about twice what a compressed file would be; a report of a hundred pages is a
//! few megabytes.

use super::fonts::Font;

/// A link on a page: the rectangle that is clicked, in the PDF's coordinates, and where it goes.
#[derive(Debug, Clone)]
pub struct Link {
    pub rect: [f32; 4],
    pub uri: String,
}

/// One page, drawn.
#[derive(Debug, Clone, Default)]
pub struct Page {
    pub ops: String,
    pub links: Vec<Link>,
}

/// One bookmark: its title, the page it goes to, how far down that page in the PDF's coordinates, and the bookmarks
/// under it.
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub title: String,
    pub page: usize,
    pub y: f32,
    pub children: Vec<Bookmark>,
}

/// A number as the PDF writes it: at most two decimals, no trailing zeros.
pub fn num(n: f32) -> String {
    let r = (n * 100.0).round() / 100.0;
    let s = format!("{r:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" {
        "0".to_owned()
    } else {
        s.to_owned()
    }
}

/// A PDF text string for a title: UTF-16 with a byte-order mark, in hexadecimal, which is right for any character.
fn text_string(text: &str) -> String {
    let mut out = String::from("<FEFF");
    for unit in text.encode_utf16() {
        out.push_str(&format!("{unit:04X}"));
    }
    out.push('>');
    out
}

/// An address as a PDF string: `\`, `(`, and `)` escaped, and anything outside printable ASCII percent-encoded.
fn uri_string(uri: &str) -> String {
    let mut out = String::from("(");
    for b in uri.bytes() {
        match b {
            b'\\' | b'(' | b')' => {
                out.push('\\');
                out.push(b as char);
            }
            0x21..=0x7e => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out.push(')');
    out
}

/// The PDF date for a report's `generated` time (`2026-10-14 14:50:14 UTC`), if it reads as one.
fn pdf_date(generated: &str) -> Option<String> {
    let digits: String = generated.chars().filter(char::is_ascii_digit).collect();
    (digits.len() >= 14 && generated.contains("UTC")).then(|| format!("D:{}Z", &digits[..14]))
}

/// The paper's width and height, in points.
pub type Size = (f32, f32);

/// The whole file.
pub fn pdf(
    pages: &[Page],
    bookmarks: &[Bookmark],
    title: &str,
    generated: Option<&str>,
    size: Size,
) -> String {
    let mut objects: Vec<String> = Vec::new();
    let mut reserve = || {
        objects.push(String::new());
        objects.len()
    };
    let catalog = reserve();
    let pages_root = reserve();
    let info = reserve();
    let fonts: Vec<usize> = Font::ALL.iter().map(|_| reserve()).collect();
    let page_ids: Vec<usize> = pages.iter().map(|_| reserve()).collect();
    let set = |objects: &mut Vec<String>, id: usize, body: String| objects[id - 1] = body;
    let add = |objects: &mut Vec<String>, body: String| {
        objects.push(body);
        objects.len()
    };

    let mut font_dict = String::from("<<");
    for (font, id) in Font::ALL.iter().zip(&fonts) {
        set(
            &mut objects,
            *id,
            format!(
                "<< /Type /Font /Subtype /Type1 /BaseFont /{} /Encoding /WinAnsiEncoding >>",
                font.base_name()
            ),
        );
        font_dict.push_str(&format!(" /{} {id} 0 R", font.key()));
    }
    font_dict.push_str(" >>");

    for (page, id) in pages.iter().zip(&page_ids) {
        let stream = add(
            &mut objects,
            format!(
                "<< /Length {} >>\nstream\n{}\nendstream",
                page.ops.len(),
                page.ops
            ),
        );
        let annots: Vec<usize> = page
            .links
            .iter()
            .map(|l| {
                add(
                    &mut objects,
                    format!(
                        "<< /Type /Annot /Subtype /Link /Rect [{} {} {} {}] /Border [0 0 0] /A << /S /URI /URI {} >> >>",
                        num(l.rect[0]),
                        num(l.rect[1]),
                        num(l.rect[2]),
                        num(l.rect[3]),
                        uri_string(&l.uri)
                    ),
                )
            })
            .collect();
        let annots = if annots.is_empty() {
            String::new()
        } else {
            format!(
                " /Annots [{}]",
                annots
                    .iter()
                    .map(|a| format!("{a} 0 R"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        set(
            &mut objects,
            *id,
            format!(
                "<< /Type /Page /Parent {pages_root} 0 R /MediaBox [0 0 {} {}] /Resources << /Font {font_dict} >> /Contents {stream} 0 R{annots} >>",
                num(size.0),
                num(size.1)
            ),
        );
    }
    set(
        &mut objects,
        pages_root,
        format!(
            "<< /Type /Pages /Kids [{}] /Count {} >>",
            page_ids
                .iter()
                .map(|p| format!("{p} 0 R"))
                .collect::<Vec<_>>()
                .join(" "),
            page_ids.len()
        ),
    );

    let outline = if bookmarks.is_empty() || page_ids.is_empty() {
        String::new()
    } else {
        let root = add(&mut objects, String::new());
        let (first, last, count) = write_bookmarks(&mut objects, bookmarks, root, &page_ids);
        set(
            &mut objects,
            root,
            format!("<< /Type /Outlines /First {first} 0 R /Last {last} 0 R /Count {count} >>"),
        );
        format!(" /Outlines {root} 0 R /PageMode /UseOutlines")
    };
    set(
        &mut objects,
        catalog,
        format!(
            "<< /Type /Catalog /Pages {pages_root} 0 R /Lang (en-US) /ViewerPreferences << /DisplayDocTitle true >>{outline} >>"
        ),
    );
    let date = generated
        .and_then(pdf_date)
        .map(|d| format!(" /CreationDate ({d})"))
        .unwrap_or_default();
    set(
        &mut objects,
        info,
        format!(
            "<< /Title {} /Producer (StackVet) /Creator (StackVet){date} >>",
            text_string(title)
        ),
    );

    let mut out = String::from("%PDF-1.4\n");
    let mut offsets = Vec::with_capacity(objects.len());
    for (i, body) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.push_str(&format!("{} 0 obj\n{body}\nendobj\n", i + 1));
    }
    let xref = out.len();
    out.push_str(&format!(
        "xref\n0 {}\n0000000000 65535 f \n",
        objects.len() + 1
    ));
    for o in offsets {
        out.push_str(&format!("{o:010} 00000 n \n"));
    }
    out.push_str(&format!(
        "trailer\n<< /Size {} /Root {catalog} 0 R /Info {info} 0 R >>\nstartxref\n{xref}\n%%EOF\n",
        objects.len() + 1
    ));
    out
}

/// Writes one level of bookmarks under `parent`, and says its first, its last, and how many are shown open.
fn write_bookmarks(
    objects: &mut Vec<String>,
    marks: &[Bookmark],
    parent: usize,
    page_ids: &[usize],
) -> (usize, usize, usize) {
    let ids: Vec<usize> = marks
        .iter()
        .map(|_| {
            objects.push(String::new());
            objects.len()
        })
        .collect();
    for (i, mark) in marks.iter().enumerate() {
        let page = page_ids[mark.page.min(page_ids.len() - 1)];
        let mut parts = vec![
            format!("/Title {}", text_string(&mark.title)),
            format!("/Parent {parent} 0 R"),
            format!("/Dest [{page} 0 R /XYZ 0 {} null]", num(mark.y)),
        ];
        if i > 0 {
            parts.push(format!("/Prev {} 0 R", ids[i - 1]));
        }
        if i + 1 < ids.len() {
            parts.push(format!("/Next {} 0 R", ids[i + 1]));
        }
        if !mark.children.is_empty() {
            let (first, last, _) = write_bookmarks(objects, &mark.children, ids[i], page_ids);
            // Shown closed: a negative count is the number of bookmarks folded under it.
            parts.push(format!(
                "/First {first} 0 R /Last {last} 0 R /Count -{}",
                mark.children.len()
            ));
        }
        objects[ids[i] - 1] = format!("<< {} >>", parts.join(" "));
    }
    (ids[0], ids[ids.len() - 1], marks.len())
}
