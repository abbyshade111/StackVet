//! The report in short pages, one per thing a person does with it (backlog 0590, ADR-121, Later).
//!
//! On a small app compliance.md ran to 22,000 words in sixteen sections, and a person who is not a programmer could not
//! find the one thing they needed. Each page here holds the sections for one purpose, cut whole from the reports `sv`
//! already writes, so a page says nothing those do not and drops none of their counts or caveats: what to fix now, the
//! decisions only the person can make, what was not examined, and what is there for information. compliance.md stays
//! whole as the long reference, security.md stays one file for whoever the report is handed to, the checklist before
//! going live is its own page already (`checklist`), and report.html is the front page that leads to each.

use crate::Report;

/// One page of the report.
pub struct Page {
    /// Its name in the report folder.
    pub file: &'static str,
    /// Its title.
    pub title: &'static str,
    /// What it is for, in one line, as the front page says it.
    pub purpose: &'static str,
}

pub const FIX_NOW: Page = Page {
    file: "fix-now.md",
    title: "Fix now",
    purpose: "What the checks found that needs attention, worst first, each with where it is and what to do",
};
pub const DECISIONS: Page = Page {
    file: "your-decisions.md",
    title: "Your decisions",
    purpose: "The questions only you can answer, and the checks only you can make",
};
pub const NOT_EXAMINED: Page = Page {
    file: "not-examined.md",
    title: "What was not examined",
    purpose: "What this report could not look at, and why, in one place",
};
pub const INFORMATION: Page = Page {
    file: "information.md",
    title: "Information",
    purpose: "What is worth knowing and asks nothing of you: findings in test code, checks that found nothing, what \
              your AI coding tool's files let it do, and the text of each requirement",
};

/// The four pages cut from the reports, in the order the front page lists them.
pub const PAGES: [&Page; 4] = [&FIX_NOW, &DECISIONS, &NOT_EXAMINED, &INFORMATION];

/// The `## ` sections of a Markdown report, each its heading line and everything under it to the next.
fn sections(markdown: &str) -> Vec<(&str, String)> {
    let mut out: Vec<(&str, String)> = Vec::new();
    for line in markdown.split_inclusive('\n') {
        if let Some(heading) = line.strip_prefix("## ") {
            out.push((heading.trim_end(), line.to_owned()));
        } else if let Some((_, body)) = out.last_mut() {
            body.push_str(line);
        }
    }
    out
}

/// The sections of `markdown` whose heading `wanted` takes, in their order there.
fn cut(markdown: &str, wanted: impl Fn(&str) -> bool) -> Vec<String> {
    sections(markdown)
        .into_iter()
        .filter(|(heading, _)| wanted(heading))
        .map(|(_, body)| body)
        .collect()
}

/// A page: its header, then `parts`, or, when there are none, `empty`.
fn page(report: &Report, page: &Page, parts: Vec<String>, empty: &str) -> String {
    use crate::markdown::inert;
    let mut out = format!(
        "# {}: {}\n\n",
        page.title,
        inert(&report.app_name).replace(['\n', '\r'], " ")
    );
    match &report.generated {
        Some(when) => out.push_str(&format!(
            "Produced by `sv` {} on {when}.\n\n",
            report.sv.describe()
        )),
        None => out.push_str(&format!("Produced by `sv` {}.\n\n", report.sv.describe())),
    }
    out.push_str(&crate::folder::markdown_line(page.file));
    out.push_str(
        "This page is one part of the report, cut whole from it: it credits nothing on its own, and the counts \
         stay in `compliance.md`. Like every file in this folder it is sealed with the report, so `sv` can tell \
         whether it is the page it wrote.\n\n",
    );
    if parts.is_empty() {
        out.push_str(empty);
        out.push_str("\n\n");
    }
    for part in parts {
        out.push_str(&part);
        if !part.ends_with("\n\n") {
            out.push('\n');
        }
    }
    out
}

/// `fix-now.md`: the things to fix from security.md, each in full, worst first.
pub fn fix_now(report: &Report) -> String {
    let security = crate::markdown::security(report);
    let parts = cut(&security, |h| {
        h.ends_with(" to fix") || h.starts_with("Nothing was found")
    });
    page(
        report,
        &FIX_NOW,
        parts,
        "The checks that ran found nothing to fix. That is not the same as the app being secure: read \
         `not-examined.md` for what they could not look at.",
    )
}

/// `your-decisions.md`: what only the person can check, and the requirements waiting on their answer.
pub fn decisions(report: &Report) -> String {
    use crate::markdown::inert;
    let compliance = crate::markdown::compliance(report);
    let mut parts = cut(&compliance, |h| h == "What only you can check");
    if !report.undecided.is_empty() {
        let mut part = String::from(
            "## Requirements nobody has placed\n\nEach of these applies or not depending on an answer only you can \
             give. Answer it in `stackvet.toml`, or with your AI coding tool, and write the report again.\n\n",
        );
        for un in &report.undecided {
            part.push_str(&format!(
                "- **{}** {}: {}\n",
                inert(&un.id),
                inert(&un.description),
                inert(&un.blocked_on.join(" Or: "))
            ));
        }
        part.push('\n');
        parts.push(part);
    }
    page(
        report,
        &DECISIONS,
        parts,
        "Nothing is waiting on your answer for this app.",
    )
}

/// `not-examined.md`: what the report could not look at, the outside tools' runs, and the comments that ask a scanner
/// to look away.
pub fn not_examined(report: &Report) -> String {
    let compliance = crate::markdown::compliance(report);
    let parts = cut(&compliance, |h| {
        matches!(
            h,
            "What was not examined" | "Outside tools" | "Comments that tell a scanner to look away"
        )
    });
    page(
        report,
        &NOT_EXAMINED,
        parts,
        "This report names nothing it could not examine.",
    )
}

/// `information.md`: what asks nothing of the person.
pub fn information(report: &Report) -> String {
    let compliance = crate::markdown::compliance(report);
    let security = crate::markdown::security(report);
    let mut parts = cut(&security, |h| {
        h.starts_with("Worth knowing") || h.contains("in test code, information only")
    });
    parts.extend(cut(&compliance, |h| {
        h.starts_with("How the app is built with AI")
            || h == "What your AI coding tool's files let it do"
            || h == crate::hook::HEADING
            || h == crate::tool_behavior::HEADING
            || h == "Findings about requirements this app is not being assessed against"
            || h.starts_with("Checks that ran and found nothing")
            || h == "Secure by Design controls above this app's target level"
            || h == "What the app says about itself"
            || h == "Appendix: what each requirement asks for"
    }));
    page(report, &INFORMATION, parts, "Nothing here for this app.")
}
