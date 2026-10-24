//! The AI coding tools' own instruction files, for `sv rules` (ADR-144, backlog 0665).
//!
//! `sv rules` writes the rules into `AGENTS.md`. Codex, Cursor, GitHub Copilot, and Windsurf read that
//! file on their own; Claude Code reads it only when the app has no `CLAUDE.md`, and an app built with
//! Claude Code nearly always has one. So `plan` finds the `CLAUDE.md` the app already has and the
//! `@AGENTS.md` import to put in it, between `sv`'s markers, and the tools that will still not see the
//! rules, from what is in the folder. It reads; `sv rules` writes.
//!
//! Each tool's behavior here is from its own documentation, read on 24 October 2026 (the table is in
//! ADR-144). Cline's documentation does not say whether `.clinerules` hides `AGENTS.md`, so it is named
//! as unknown, never as covered.

use std::path::Path;

/// What `sv rules` should do with Claude Code's file, and what it should tell the person.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Plan {
    pub claude: Option<Claude>,
    /// One sentence for each tool that will still not see the rules, or whose file `sv` left alone.
    pub notes: Vec<String>,
}

/// The `CLAUDE.md` the app already has, relative to its folder, and what to do with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Claude {
    /// Put `import` between `sv`'s markers in `file`.
    Import {
        file: &'static str,
        import: &'static str,
    },
    /// `file` already imports `AGENTS.md`, outside `sv`'s markers, so nothing is added.
    AlreadyImports { file: &'static str },
}

/// Where Claude Code's project file can be, the import that reaches `AGENTS.md` from it (an import is
/// read relative to the file it is in), and the folder it is written in.
const CLAUDE_FILES: [(&str, &str, &str); 2] = [
    ("CLAUDE.md", "@AGENTS.md", ""),
    (".claude/CLAUDE.md", "@../AGENTS.md", ".claude"),
];

/// The section `sv rules` puts between its markers in a `CLAUDE.md`: the import, and why it is there.
pub fn claude_section(import: &str) -> String {
    format!(
        "Claude Code reads `AGENTS.md` only through the line below, because this project has a \
         `CLAUDE.md`. It brings in the security rules `sv rules` wrote there.\n\n{import}\n"
    )
}

fn is_link(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
}

/// Whether `text`, outside `sv`'s markers, already holds `import` on a line of its own.
fn imports_outside_markers(text: &str, import: &str) -> bool {
    let mut inside = false;
    for line in text.lines() {
        let line = line.trim();
        if line == crate::coding_rules::BEGIN {
            inside = true;
        } else if line == crate::coding_rules::END {
            inside = false;
        } else if !inside && (line == import || line == import.replacen('@', "@./", 1)) {
            return true;
        }
    }
    false
}

/// What to do in `app_dir` after `AGENTS.md` is written.
pub fn plan(app_dir: &Path) -> Plan {
    let mut plan = Plan::default();
    // Whether the app has one of Claude Code's project files at all, written or not.
    let mut has_claude_file = false;
    for (file, import, folder) in CLAUDE_FILES {
        let path = app_dir.join(file);
        if !folder.is_empty() && is_link(&app_dir.join(folder)) {
            if path.exists() {
                has_claude_file = true;
                plan.notes.push(format!(
                    "Claude Code: {folder} is a link to somewhere else, so `sv` did not write into \
                     {file}; add the line `{import}` to it yourself."
                ));
            }
            continue;
        }
        if is_link(&path) {
            plan.notes.push(format!(
                "Claude Code: {file} is a link, so `sv` did not write into it. If it points at \
                 AGENTS.md, Claude Code reads the rules through it; if not, add the line `{import}` \
                 to the file it points at."
            ));
            has_claude_file = true;
            break;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        has_claude_file = true;
        plan.claude = Some(if imports_outside_markers(&text, import) {
            Claude::AlreadyImports { file }
        } else {
            Claude::Import { file, import }
        });
        break;
    }
    if !has_claude_file && app_dir.join("CLAUDE.local.md").is_file() {
        plan.notes.push(
            "Claude Code: this project has a CLAUDE.local.md and no CLAUDE.md, so Claude Code reads \
             CLAUDE.local.md instead of AGENTS.md. `sv` does not write into your own CLAUDE.local.md; \
             add the line `@AGENTS.md` to it to bring the rules in."
                .to_owned(),
        );
    }
    if app_dir.join("AGENTS.override.md").is_file() {
        plan.notes.push(
            "Codex: AGENTS.override.md is in the same folder, and Codex reads it instead of AGENTS.md, \
             so it does not see the rules."
                .to_owned(),
        );
    }
    if let Ok(config) = std::fs::read_to_string(app_dir.join(".aider.conf.yml"))
        && !config.contains("AGENTS.md")
    {
        plan.notes.push(
            "Aider: it reads AGENTS.md only when told to. Add `read: AGENTS.md` to .aider.conf.yml, \
             or start it with `aider --read AGENTS.md`."
                .to_owned(),
        );
    }
    if app_dir.join(".clinerules").exists() {
        plan.notes.push(
            "Cline: this project has .clinerules, and Cline's documentation does not say whether it \
             then reads AGENTS.md too. Check Cline's Rules panel for AGENTS.md."
                .to_owned(),
        );
    }
    plan
}
