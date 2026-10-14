//! The report as a PDF, `report.pdf`, written by `sv` itself (ADR-115): no browser, no print dialog, no new
//! dependency, and nothing downloaded. It is laid out from the same Markdown the other reports are written from
//! (`markdown::pdf_text`: `compliance.md`'s order, each finding written out in full), so it cannot say something they
//! do not. US Letter unless stackvet.toml asks for A4.
//!
//! What it costs: the fonts every PDF reader has show Western European letters and common punctuation only, so any
//! other character prints as `?`, and the first page says that `report.html` is the complete version. A test fails
//! when `sv`'s own words use a character the fonts lack (`pdf/fonts_tests.rs`).

pub mod back;
pub mod fonts;
pub mod layout;
pub mod read;
pub mod write;

#[cfg(test)]
mod pdf_tests;

use serde::Serialize;

/// The paper the PDF is laid out on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Paper {
    /// 8.5 by 11 inches, as in v1.
    #[default]
    Letter,
    /// 210 by 297 millimeters.
    A4,
}

impl Paper {
    /// Width and height in points.
    pub fn size(self) -> write::Size {
        match self {
            Paper::Letter => (612.0, 792.0),
            Paper::A4 => (595.28, 841.89),
        }
    }

    /// The paper stackvet.toml's `report-paper` names: `a4`, or anything else for US Letter.
    pub fn named(name: Option<&str>) -> Paper {
        match name.map(|n| n.trim().to_ascii_lowercase()) {
            Some(n) if n == "a4" => Paper::A4,
            _ => Paper::Letter,
        }
    }

    /// The paper's name, as the report says it.
    pub fn name(self) -> &'static str {
        match self {
            Paper::Letter => "US Letter",
            Paper::A4 => "A4",
        }
    }
}

/// The file's name in the report folder.
pub const FILE: &str = "report.pdf";

/// What the first page says about the fonts, under the line saying which `sv` wrote the report.
pub const FONTS_NOTE: &str = "This is the report as one file to read or print. The fonts every PDF reader has show \
    Western European letters only, so any other character in it prints as ?. report.html, beside it, is the complete \
    version.";

/// `markdown` laid out as a PDF on `paper`, every page's foot naming `title`.
pub fn document(markdown: &str, title: &str, generated: Option<&str>, paper: Paper) -> String {
    let blocks = read::blocks(markdown);
    let (pages, bookmarks) = layout::pages(&blocks, title, paper.size());
    write::pdf(&pages, &bookmarks, title, generated, paper.size())
}

/// The report as `report.pdf`.
pub fn render(report: &crate::Report) -> String {
    let title = format!("{} - StackVet report", crate::one_line(&report.app_name));
    document(
        &crate::markdown::pdf_text(report),
        &title,
        report.generated.as_deref(),
        report.paper,
    )
}
