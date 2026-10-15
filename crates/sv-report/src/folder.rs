//! What each file in a report folder holds and who it is for (backlog 0413). The owner found `security.md` and
//! `compliance.md` read alike on a small app, and the README called them "the same". They are not: each file has a
//! reader of its own, and this table says so once. `report.html` shows it at its foot, the two Markdown reports say
//! their own line at their top, and `sv-cli`'s `report_files_described` test holds the README, the guide, and the
//! website's reports page to the same names and readers, in the order `sv-cli`'s `report_files::REPORT_FILES` writes
//! them.

/// One file of a report folder, described for a person.
pub struct FolderFile {
    /// Its name in the report folder.
    pub name: &'static str,
    /// What it holds.
    pub holds: &'static str,
    /// Who it is for.
    pub for_whom: &'static str,
}

/// The fourteen files, in the order they are written.
pub const FILES: [FolderFile; 14] = [
    FolderFile {
        name: "report.html",
        holds: "The whole report on one page: what to fix first, what was not examined, and every requirement that \
                applies",
        for_whom: "You, in a browser: double-click it",
    },
    FolderFile {
        name: "compliance.md",
        holds: "Every requirement that applies, chapter by chapter, with what is known of each; the threats; what \
                only you can check; and the tests worth writing",
        for_whom: "Anyone who needs the whole picture against the standards, you or someone you show it to",
    },
    FolderFile {
        name: "security.md",
        holds: "What the checks found, each in full (where, the code, how sure, why it matters, what to do, and how \
                to set it aside), then what was not examined",
        for_whom: "Whoever fixes the code, you or your AI coding tool",
    },
    FolderFile {
        name: crate::LOW_FINDINGS_FILE,
        holds: "Every low-severity finding in full, where the other reports group them by rule",
        for_whom: "You or your AI coding tool, when you want the detail",
    },
    // Backlog 0589, ADR-121.
    FolderFile {
        name: crate::checklist::FILE,
        holds: "What to do or confirm before the app is on the internet, in the order you would do it, with what \
                could not be verified at its end. It credits nothing",
        for_whom: "You, or whoever puts the app on the internet",
    },
    // The report in short pages, one per purpose (backlog 0590).
    FolderFile {
        name: crate::pages::FIX_NOW.file,
        holds: "What needs attention, worst first, each with where it is and what to do: the findings of security.md \
                that are things to fix",
        for_whom: "You or your AI coding tool, first",
    },
    FolderFile {
        name: crate::pages::DECISIONS.file,
        holds: "The questions only you can answer and the checks only you can make, from compliance.md",
        for_whom: "You",
    },
    FolderFile {
        name: crate::pages::NOT_EXAMINED.file,
        holds: "What this report could not look at, and why, with the outside tools' runs, from compliance.md",
        for_whom: "You, before you read anything as safe",
    },
    FolderFile {
        name: crate::pages::INFORMATION.file,
        holds: "What asks nothing of you: findings in test code, checks that found nothing, your AI coding tool's \
                files, and the text of each requirement, from compliance.md and security.md",
        for_whom: "You, when you want the rest",
    },
    FolderFile {
        name: crate::SARIF_FILE,
        holds: "The findings as SARIF, a standard format for findings (named findings.sarif before 14 October 2026)",
        for_whom: "Code editors and GitHub's code scanning, which show a finding beside its line",
    },
    FolderFile {
        name: "report.json",
        holds: "The report's data as JSON",
        for_whom: "Programs, among them sv review, sv compare, and sv dashboard",
    },
    FolderFile {
        name: crate::seen::FILE,
        holds: "What sv saw of the running app, with every key and sign-in in it cut short",
        for_whom: "You, to see what a finding rests on",
    },
    FolderFile {
        name: crate::SBOM_FILE,
        holds: "The app's bill of materials, every package it uses with its version, as CycloneDX JSON",
        for_whom: "Tools that read a bill of materials",
    },
    // Written by sv itself (ADR-116).
    FolderFile {
        name: crate::pdf::FILE,
        holds: "The whole report as one file to read, print, or send: compliance.md's order, with each finding in \
                full. US Letter paper unless stackvet.toml's report-paper setting asks for A4; its fonts show Western \
                European letters only, so report.html is the complete version",
        for_whom: "You, or whoever you hand it to",
    },
];

/// The file named `name`. Panics on a name not in the table: every caller names one of its own.
pub fn about(name: &str) -> &'static FolderFile {
    FILES
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("{name} is not a report file"))
}

/// The line a Markdown report opens with, saying what it is and who it is for.
pub fn markdown_line(name: &str) -> String {
    let file = about(name);
    format!(
        "**This file:** {}. **For:** {}. The end of `report.html` lists every file in this folder and who each is \
         for.\n\n",
        file.holds, file.for_whom
    )
}
