//! The blocks laid out on pages. The model is plain: every block becomes lines of known height, and pages are filled
//! a line at a time, so a paragraph, a list, or a block of code breaks between any two of its lines. A heading is
//! kept with the lines after it. A table is drawn a row at a time: a row that does not fit goes to the next page,
//! where the table's header row is drawn again first, and a row taller than what is left of a page is split between
//! its lines, the rest of every cell going on together after the header on the next page.

use super::fonts::{self, Font};
use super::read::{Block, Look, Span};
use super::write::{Bookmark, Link, Page, Size, num};

/// The margin on every side, in points (three quarters of an inch).
pub const MARGIN: f32 = 54.0;
/// Room under the page's text for the line that says which page it is.
const FOOT: f32 = 22.0;
const BODY: f32 = 10.0;
const SMALL: f32 = 8.5;
const CODE: f32 = 8.0;
const LEADING: f32 = 1.35;
const PAD: f32 = 3.0;

const DARK: &str = "0.1 0.11 0.13";
const MUTED: &str = "0.35 0.38 0.42";
const LINK: &str = "0.04 0.34 0.82";
const RULE: &str = "0.78 0.81 0.84";
const PANEL: &str = "0.95 0.96 0.97";

/// A run of text placed on a line.
#[derive(Debug, Clone)]
struct Run {
    text: String,
    x: f32,
    font: Font,
    size: f32,
    link: Option<String>,
}

/// One line of wrapped text: its runs, how tall it is, and where its baseline sits below its top.
#[derive(Debug, Clone, Default)]
struct Line {
    runs: Vec<Run>,
    height: f32,
    width: f32,
}

impl Line {
    fn baseline(&self) -> f32 {
        let size = self.runs.iter().map(|r| r.size).fold(0.0, f32::max);
        (self.height - size) / 2.0 + size * 0.8
    }
}

/// The font a span's look asks for, on top of the block's own.
fn font_for(look: Look, base: Font) -> Font {
    if look.code {
        return Font::Mono;
    }
    let mut f = base;
    if look.bold {
        f = f.bold();
    }
    if look.italic {
        f = f.italic();
    }
    f
}

/// One word to place: its text, how it is drawn, and whether a space comes before it.
struct Word {
    text: String,
    font: Font,
    size: f32,
    link: Option<String>,
    space: bool,
}

fn words(spans: &[Span], base: Font, size: f32) -> Vec<Word> {
    let mut out = Vec::new();
    let mut space = false;
    for s in spans {
        let font = font_for(s.look, base);
        // Code is drawn a little smaller, so it reads as the same size beside the text.
        let size = if s.look.code { size * 0.92 } else { size };
        let mut word = String::new();
        for c in s.text.chars() {
            if c.is_whitespace() {
                if !word.is_empty() {
                    out.push(Word {
                        text: std::mem::take(&mut word),
                        font,
                        size,
                        link: s.link.clone(),
                        space,
                    });
                }
                space = true;
            } else {
                word.push(c);
            }
        }
        if !word.is_empty() {
            out.push(Word {
                text: word,
                font,
                size,
                link: s.link.clone(),
                space,
            });
            space = false;
        }
    }
    out
}

/// The spans wrapped into lines no wider than `width`. A word longer than a line (a path, a hash) is cut where it
/// runs out, after a `/`, `-`, `_`, or `.` when there is one.
fn wrap(spans: &[Span], base: Font, size: f32, width: f32) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut line = Line::default();
    let height = size * LEADING;
    let finish = |line: &mut Line, lines: &mut Vec<Line>| {
        line.height = line
            .runs
            .iter()
            .map(|r| r.size * LEADING)
            .fold(height, f32::max);
        lines.push(std::mem::take(line));
    };
    let put = |line: &mut Line, text: &str, w: &Word, gap: f32| {
        let x = line.width + gap;
        match line.runs.last_mut() {
            Some(last) if last.font == w.font && last.size == w.size && last.link == w.link => {
                if gap > 0.0 {
                    last.text.push(' ');
                }
                last.text.push_str(text);
            }
            _ => line.runs.push(Run {
                text: text.to_owned(),
                x,
                font: w.font,
                size: w.size,
                link: w.link.clone(),
            }),
        }
        line.width = x + fonts::width(text, w.font, w.size);
    };
    for w in words(spans, base, size) {
        let ww = fonts::width(&w.text, w.font, w.size);
        let space = |line: &Line| {
            if line.runs.is_empty() || !w.space {
                0.0
            } else {
                let last = line.runs.last().unwrap();
                // The gap is measured in the font a merged run draws it in.
                if last.font == w.font && last.size == w.size && last.link == w.link {
                    fonts::width(" ", w.font, w.size)
                } else if last.font == Font::Mono {
                    // After code, the space is the text's: Courier's is wider than the eye expects.
                    fonts::width(" ", w.font, w.size)
                } else {
                    fonts::width(" ", last.font, last.size)
                }
            }
        };
        let mut gap = space(&line);
        if !line.runs.is_empty() && line.width + gap + ww > width {
            finish(&mut line, &mut lines);
            gap = 0.0;
        }
        if line.width + gap + ww <= width {
            put(&mut line, &w.text, &w, gap);
            continue;
        }
        // Too long for a line of its own: cut it.
        let mut piece = String::new();
        for c in w.text.chars() {
            let mut tried = piece.clone();
            tried.push(c);
            if !piece.is_empty() && line.width + gap + fonts::width(&tried, w.font, w.size) > width
            {
                let cut = piece.rfind(['/', '-', '_', '.']).map(|i| i + 1);
                let keep = match cut {
                    Some(k) if k > 0 && k < piece.len() => k,
                    _ => piece.len(),
                };
                let rest = piece.split_off(keep);
                put(&mut line, &piece, &w, gap);
                finish(&mut line, &mut lines);
                gap = 0.0;
                piece = rest;
            }
            piece.push(c);
        }
        if !piece.is_empty() {
            put(&mut line, &piece, &w, gap);
        }
    }
    if !line.runs.is_empty() {
        finish(&mut line, &mut lines);
    }
    lines
}

/// The hexadecimal string the fonts draw `text` as.
fn hex(text: &str) -> String {
    fonts::encode(text)
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect()
}

/// The pages being filled.
struct Pager {
    size: Size,
    pages: Vec<Page>,
    /// How far down the current page the next line starts, from the top of the paper.
    y: f32,
    /// Each heading kept as a bookmark: its level, title, page, and height on the page in the PDF's coordinates.
    marks: Vec<(usize, String, usize, f32)>,
}

impl Pager {
    fn new(size: Size) -> Pager {
        Pager {
            size,
            pages: vec![Page::default()],
            y: MARGIN,
            marks: Vec::new(),
        }
    }

    fn bottom(&self) -> f32 {
        self.size.1 - MARGIN - FOOT
    }

    fn left(&self) -> f32 {
        self.bottom() - self.y
    }

    fn at_top(&self) -> bool {
        self.y <= MARGIN
    }

    fn new_page(&mut self) {
        self.pages.push(Page::default());
        self.y = MARGIN;
    }

    /// A new page unless `height` fits on this one, or this one is empty.
    fn need(&mut self, height: f32) {
        if height > self.left() && !self.at_top() {
            self.new_page();
        }
    }

    fn page(&mut self) -> &mut Page {
        self.pages.last_mut().expect("there is always a page")
    }

    fn text(&mut self, x: f32, baseline: f32, text: &str, font: Font, size: f32, color: &str) {
        let bytes = hex(text);
        if bytes.is_empty() {
            return;
        }
        let y = self.size.1 - baseline;
        let op = format!(
            "BT /{} {} Tf {color} rg {} {} Td <{bytes}> Tj ET\n",
            font.key(),
            num(size),
            num(x),
            num(y)
        );
        self.page().ops.push_str(&op);
    }

    /// A filled rectangle, given by its top left corner from the top of the paper.
    fn rect(&mut self, x: f32, top: f32, w: f32, h: f32, color: &str) {
        let y = self.size.1 - top - h;
        let op = format!(
            "{color} rg {} {} {} {} re f\n",
            num(x),
            num(y),
            num(w),
            num(h)
        );
        self.page().ops.push_str(&op);
    }

    /// Draws `line` with its left edge at `x` and its top at `top`, and links what links.
    fn line_at(&mut self, x: f32, top: f32, line: &Line, color: &str) {
        let base = top + line.baseline();
        for run in &line.runs {
            let w = fonts::width(&run.text, run.font, run.size);
            let color = if run.link.is_some() { LINK } else { color };
            self.text(x + run.x, base, &run.text, run.font, run.size, color);
            if let Some(uri) = &run.link {
                self.rect(x + run.x, base + 1.2, w, 0.5, LINK);
                let h = self.size.1;
                self.page().links.push(Link {
                    rect: [x + run.x, h - top - line.height, x + run.x + w, h - top],
                    uri: uri.clone(),
                });
            }
        }
    }

    /// Draws lines from the current place down, each on the page it fits on.
    fn lines(&mut self, x: f32, lines: &[Line], color: &str) {
        for line in lines {
            self.need(line.height);
            let top = self.y;
            self.line_at(x, top, line, color);
            self.y += line.height;
        }
    }
}

/// How wide `text` would be on one line, and its longest word.
fn natural(spans: &[Span], size: f32, base: Font) -> (f32, f32) {
    let ws = words(spans, base, size);
    let mut total = 0.0;
    let mut longest: f32 = 0.0;
    for (i, w) in ws.iter().enumerate() {
        let ww = fonts::width(&w.text, w.font, w.size);
        longest = longest.max(ww);
        if i > 0 && w.space {
            total += fonts::width(" ", w.font, w.size);
        }
        total += ww;
    }
    (total, longest)
}

/// Each column's width: at least its longest word where the page allows, and the rest shared by how much more each
/// would take to show its cells on one line.
fn column_widths(header: &[Vec<Span>], rows: &[Vec<Vec<Span>>], width: f32) -> Vec<f32> {
    let n = header
        .len()
        .max(rows.iter().map(Vec::len).max().unwrap_or(0));
    let mut most = vec![0.0f32; n];
    let mut least = vec![0.0f32; n];
    let cap = width * 0.4;
    for (r, row) in std::iter::once(header)
        .chain(rows.iter().map(Vec::as_slice))
        .enumerate()
    {
        let base = if r == 0 { Font::Bold } else { Font::Regular };
        for (i, cell) in row.iter().enumerate().take(n) {
            let (all, longest) = natural(cell, SMALL, base);
            most[i] = most[i].max(all + 2.0 * PAD);
            least[i] = least[i].max((longest + 2.0 * PAD).min(cap));
        }
    }
    let total_least: f32 = least.iter().sum();
    if total_least >= width {
        return least.iter().map(|l| l * width / total_least).collect();
    }
    let total_most: f32 = most.iter().sum();
    if total_most <= width {
        // Everything fits on one line: the spare room goes to the widest columns, so the table spans the page.
        return most.iter().map(|m| m * width / total_most).collect();
    }
    let spare = width - total_least;
    let want: Vec<f32> = most
        .iter()
        .zip(&least)
        .map(|(m, l)| (m - l).max(0.0))
        .collect();
    let total_want: f32 = want.iter().sum();
    least
        .iter()
        .zip(&want)
        .map(|(l, w)| {
            l + if total_want > 0.0 {
                spare * w / total_want
            } else {
                0.0
            }
        })
        .collect()
}

/// One row of a table, its cells wrapped.
struct Row {
    cells: Vec<Vec<Line>>,
    bold: bool,
}

impl Row {
    fn height(&self, from: usize) -> f32 {
        self.cells
            .iter()
            .map(|c| c.iter().skip(from).map(|l| l.height).sum::<f32>())
            .fold(0.0, f32::max)
            + 2.0 * PAD
    }

    fn done(&self, from: usize) -> bool {
        self.cells.iter().all(|c| c.len() <= from)
    }
}

/// Draws one row's lines from `from` on, as many as fit, and says where it stopped.
fn draw_row(p: &mut Pager, row: &Row, widths: &[f32], right: &[bool], from: usize) -> usize {
    let top = p.y;
    let room = p.left() - 2.0 * PAD;
    // How many lines fit, counted by the tallest cell.
    let mut upto = from;
    loop {
        let next = upto + 1;
        let h = row
            .cells
            .iter()
            .map(|c| {
                c.iter()
                    .skip(from)
                    .take(next - from)
                    .map(|l| l.height)
                    .sum::<f32>()
            })
            .fold(0.0, f32::max);
        if h > room || row.done(upto) {
            break;
        }
        upto = next;
    }
    if upto == from {
        // Not a line fits: draw one anyway, so a page always moves the table on.
        upto = from + 1;
    }
    let mut height: f32 = 0.0;
    for cell in &row.cells {
        height = height.max(
            cell.iter()
                .skip(from)
                .take(upto - from)
                .map(|l| l.height)
                .sum(),
        );
    }
    let height = height + 2.0 * PAD;
    let total: f32 = widths.iter().sum();
    if row.bold {
        p.rect(MARGIN, top, total, height, PANEL);
    }
    let mut x = MARGIN;
    for (i, cell) in row.cells.iter().enumerate() {
        let w = widths.get(i).copied().unwrap_or(0.0);
        let mut y = top + PAD;
        for line in cell.iter().skip(from).take(upto - from) {
            let at = if right.get(i).copied().unwrap_or(false) {
                x + w - PAD - line.width
            } else {
                x + PAD
            };
            p.line_at(at, y, line, DARK);
            y += line.height;
        }
        x += w;
    }
    p.rect(MARGIN, top + height - 0.5, total, 0.5, RULE);
    p.y += height;
    upto
}

fn table(p: &mut Pager, header: &[Vec<Span>], right: &[bool], rows: &[Vec<Vec<Span>>]) {
    let width = p.size.0 - 2.0 * MARGIN;
    let widths = column_widths(header, rows, width);
    let wrap_row = |cells: &[Vec<Span>], bold: bool| Row {
        cells: widths
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let base = if bold { Font::Bold } else { Font::Regular };
                cells
                    .get(i)
                    .map(|c| wrap(c, base, SMALL, (w - 2.0 * PAD).max(1.0)))
                    .unwrap_or_default()
            })
            .collect(),
        bold,
    };
    let head = wrap_row(header, true);
    let body: Vec<Row> = rows.iter().map(|r| wrap_row(r, false)).collect();
    // The header with at least the first row's first line, or none of the table on this page.
    let first = body.first().map_or(0.0, |r| {
        r.cells
            .iter()
            .filter_map(|c| c.first())
            .map(|l| l.height)
            .fold(0.0, f32::max)
    });
    p.need(head.height(0) + first + 2.0 * PAD);
    draw_row(p, &head, &widths, right, 0);
    for row in &body {
        // A row that fits on a page of its own goes whole to the next page; a longer one is split.
        let fits_a_page = row.height(0) <= p.bottom() - MARGIN - head.height(0);
        if row.height(0) > p.left() && (fits_a_page || p.left() < 4.0 * SMALL * LEADING) {
            p.new_page();
            draw_row(p, &head, &widths, right, 0);
        }
        let mut from = 0;
        loop {
            from = draw_row(p, row, &widths, right, from);
            if row.done(from) {
                break;
            }
            p.new_page();
            draw_row(p, &head, &widths, right, 0);
        }
    }
}

/// The heading's size and the space above it, by level.
fn heading_look(level: usize) -> (f32, f32) {
    match level {
        1 => (18.0, 0.0),
        2 => (14.0, 16.0),
        3 => (12.0, 12.0),
        _ => (10.5, 10.0),
    }
}

/// The blocks laid out, then each page given its number. Returns the pages and their bookmarks.
pub fn pages(blocks: &[Block], footer: &str, paper: Size) -> (Vec<Page>, Vec<Bookmark>) {
    let mut p = Pager::new(paper);
    let width = paper.0 - 2.0 * MARGIN;
    for (i, block) in blocks.iter().enumerate() {
        match block {
            Block::Heading { level, text } => {
                let (size, above) = heading_look(*level);
                let lines = wrap(text, Font::Bold, size, width);
                let h: f32 = lines.iter().map(|l| l.height).sum();
                if !p.at_top() {
                    p.y += above;
                }
                // Kept with what follows: the heading and three lines after it, or a new page.
                p.need(h + 3.0 * BODY * LEADING);
                let title: String = text.iter().map(|s| s.text.as_str()).collect();
                if *level <= 3 {
                    let page = p.pages.len() - 1;
                    p.marks.push((*level, title, page, paper.1 - p.y));
                }
                p.lines(MARGIN, &lines, DARK);
                p.y += 4.0;
            }
            Block::Paragraph(text) => {
                let lines = wrap(text, Font::Regular, BODY, width);
                p.lines(MARGIN, &lines, DARK);
                p.y += 6.0;
            }
            Block::Item {
                marker,
                depth,
                text,
            } => {
                let indent = 14.0 * (*depth as f32 + 1.0);
                let lines = wrap(text, Font::Regular, BODY, width - indent);
                if let Some(first) = lines.first() {
                    p.need(first.height);
                    let base = p.y + first.baseline();
                    let mw = fonts::width(marker, Font::Regular, BODY);
                    p.text(
                        MARGIN + indent - 4.0 - mw,
                        base,
                        marker,
                        Font::Regular,
                        BODY,
                        DARK,
                    );
                }
                p.lines(MARGIN + indent, &lines, DARK);
                let next_is_item = matches!(blocks.get(i + 1), Some(Block::Item { .. }));
                p.y += if next_is_item { 2.0 } else { 6.0 };
            }
            Block::Table {
                header,
                right,
                rows,
            } => {
                table(&mut p, header, right, rows);
                p.y += 8.0;
            }
            Block::Code(code) => {
                let per_line = ((width - 2.0 * PAD) / (0.6 * CODE)).floor().max(1.0) as usize;
                let lh = CODE * LEADING;
                for text in code {
                    let chars: Vec<char> = text.chars().collect();
                    let pieces: Vec<String> = if chars.is_empty() {
                        vec![String::new()]
                    } else {
                        chars.chunks(per_line).map(|c| c.iter().collect()).collect()
                    };
                    for piece in pieces {
                        p.need(lh);
                        let top = p.y;
                        p.rect(MARGIN, top, width, lh, PANEL);
                        p.text(
                            MARGIN + PAD,
                            top + (lh - CODE) / 2.0 + CODE * 0.8,
                            &piece,
                            Font::Mono,
                            CODE,
                            DARK,
                        );
                        p.y += lh;
                    }
                }
                p.y += 6.0;
            }
        }
    }
    let n = p.pages.len();
    let size = paper;
    // The page's foot sits on the bottom margin, under the room kept for it.
    let base = size.1 - MARGIN;
    for i in 0..n {
        let said = format!("Page {} of {n}", i + 1);
        let w = fonts::width(&said, Font::Regular, SMALL);
        p.pages[i].ops.push_str(&format!(
            "BT /{} {} Tf {MUTED} rg {} {} Td <{}> Tj ET\n",
            Font::Regular.key(),
            num(SMALL),
            num(size.0 - MARGIN - w),
            num(size.1 - base),
            hex(&said)
        ));
        let mut foot = footer.to_owned();
        let room = size.0 - 2.0 * MARGIN - w - 24.0;
        while !foot.is_empty() && fonts::width(&foot, Font::Regular, SMALL) > room {
            foot.pop();
        }
        p.pages[i].ops.push_str(&format!(
            "BT /{} {} Tf {MUTED} rg {} {} Td <{}> Tj ET\n",
            Font::Regular.key(),
            num(SMALL),
            num(MARGIN),
            num(size.1 - base),
            hex(&foot)
        ));
    }
    (p.pages, tree(&p.marks))
}

/// The headings as bookmarks, each under the last heading of a higher level before it.
fn tree(marks: &[(usize, String, usize, f32)]) -> Vec<Bookmark> {
    fn take(marks: &[(usize, String, usize, f32)], i: &mut usize, level: usize) -> Vec<Bookmark> {
        let mut out = Vec::new();
        while *i < marks.len() && marks[*i].0 >= level {
            let (l, title, page, y) = marks[*i].clone();
            *i += 1;
            let children = take(marks, i, l + 1);
            out.push(Bookmark {
                title,
                page,
                y,
                children,
            });
        }
        out
    }
    let mut i = 0;
    let mut out = Vec::new();
    while i < marks.len() {
        let level = marks[i].0;
        out.extend(take(marks, &mut i, level));
    }
    out
}
