//! The lines of the app's code each code finding is on, with the line before and the line after, for the report's
//! "The code" under the finding (backlog 0378, part 2). A finding said which file and line; a person had to open the
//! file to see what it meant.
//!
//! Read once, when the report is put together, from the app's own folder only: a path that climbs out of it or starts
//! at the root is not read, and neither is a file too large to be code someone wrote. Each finding's lines are redacted
//! together, as one piece of text, so a key that runs over two lines is cut as the secrets scan would cut it.

use std::collections::BTreeMap;
use std::path::{Component, Path};
use sv_check::secrets::{SecretRules, redact_text_in};

/// The most findings whose lines are kept: a report is read by a person.
pub const MOST_FINDINGS: usize = 300;

/// The most characters of one line kept.
pub const LINE_CHARS: usize = 200;

/// How many lines apart two places a locator found can be and still be one place: a statement and the one or two
/// lines that finish it (backlog 0562).
const SAME_PLACE: usize = 2;

/// A file larger than this is not read for its lines.
const MOST_BYTES: u64 = 2_000_000;

/// The lines each code finding is on, keyed by its file and line, with the lines around it, redacted.
pub fn read(
    app_dir: &Path,
    findings: &[sv_check::Finding],
    rules: &SecretRules,
) -> BTreeMap<(String, usize), Vec<(usize, String)>> {
    let mut out = BTreeMap::new();
    let mut texts: BTreeMap<String, Option<Vec<String>>> = BTreeMap::new();
    for f in findings {
        if out.len() == MOST_FINDINGS {
            break;
        }
        let (file, line) = (&f.location.file, f.location.line);
        if line == 0 || out.contains_key(&(file.clone(), line)) || !inside(file) {
            continue;
        }
        let Some(kept) = window(app_dir, &mut texts, file, line, rules) else {
            continue;
        };
        out.insert((file.clone(), line), kept);
    }
    out
}

/// The line `line` of `file`, with the line before and the line after, redacted together; `None` when the file is
/// not one to read or has no such line. `texts` keeps each file read once.
fn window(
    app_dir: &Path,
    texts: &mut BTreeMap<String, Option<Vec<String>>>,
    file: &str,
    line: usize,
    rules: &SecretRules,
) -> Option<Vec<(usize, String)>> {
    if line == 0 || !inside(file) {
        return None;
    }
    let lines = texts
        .entry(file.to_owned())
        .or_insert_with(|| file_lines(app_dir, &app_dir.join(file)))
        .as_ref()?;
    if line > lines.len() {
        return None;
    }
    let first = line.saturating_sub(1).max(1);
    let last = (line + 1).min(lines.len());
    let window: Vec<&str> = (first..=last).map(|n| lines[n - 1].as_str()).collect();
    let (redacted, _) = redact_text_in(rules, file, &window.join("\n"));
    Some(
        (first..)
            .zip(redacted.split('\n'))
            .map(|(n, text)| (n, bounded(text)))
            .collect(),
    )
}

/// Gives each question a locator speaks to the places it found, with their lines (backlog 0487): the first
/// `human::MOST_POINTERS` whose lines could be read, and how many more. The same places on both of the report's lists
/// of questions, so `report.json` can still name the one list by the other's ids.
pub fn point_questions(
    app_dir: &Path,
    locators: &sv_check::locate::Locators,
    found: &sv_check::locate::Found,
    rules: &SecretRules,
    report: &mut sv_report::Report,
) {
    use sv_check::human::{MOST_POINTERS, Pointer, RestsOn};
    let mut texts: BTreeMap<String, Option<Vec<String>>> = BTreeMap::new();
    // Per question, each locator that found a place it could show: what it finds, the places that could be read (at
    // most `MOST_POINTERS`), and how many more it found. A place within `SAME_PLACE` lines of one already taken for
    // the question, in the same file, is the same place (backlog 0562): a refusal written as its log line and then its
    // answer (`event(..., status=403)` above `send(403, ...)`) was shown once and counted again among the "more".
    let mut by_question: BTreeMap<String, Vec<(String, Vec<Pointer>, usize)>> = BTreeMap::new();
    let mut taken: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new();
    for locator in locators.all() {
        let Some(places) = found.places.get(&locator.id) else {
            continue;
        };
        for id in &locator.questions {
            let taken = taken.entry(id.clone()).or_default();
            let mut shown = Vec::new();
            let mut more = found.more.get(&locator.id).copied().unwrap_or(0);
            for place in places {
                if taken.iter().any(|(file, line)| {
                    *file == place.file && line.abs_diff(place.line) <= SAME_PLACE
                }) {
                    continue;
                }
                taken.push((place.file.clone(), place.line));
                if shown.len() == MOST_POINTERS {
                    more += 1;
                    continue;
                }
                match window(app_dir, &mut texts, &place.file, place.line, rules) {
                    Some(lines) => shown.push(Pointer {
                        file: place.file.clone(),
                        line: place.line,
                        lines,
                    }),
                    None => more += 1,
                }
            }
            if !shown.is_empty() {
                by_question.entry(id.clone()).or_default().push((
                    locator.finds.clone(),
                    shown,
                    more,
                ));
            }
        }
    }
    // Several locators can speak to one question (backlog 0516): the first place of each, then the second, in the
    // order `data/locators.json` lists them, so a question shows what every locator found before more of any one.
    let by_question: BTreeMap<String, RestsOn> = by_question
        .into_iter()
        .map(|(id, found)| {
            let finds = found
                .iter()
                .map(|(finds, _, _)| finds.as_str())
                .collect::<Vec<_>>()
                .join("; ");
            let mut places = Vec::new();
            let mut more: usize = found.iter().map(|(_, _, more)| more).sum();
            for round in 0..MOST_POINTERS {
                for (_, shown, _) in &found {
                    if let Some(place) = shown.get(round) {
                        if places.len() < MOST_POINTERS {
                            places.push(place.clone());
                        } else {
                            more += 1;
                        }
                    }
                }
            }
            (
                id,
                RestsOn {
                    finds,
                    places,
                    more,
                },
            )
        })
        .collect();
    for item in report
        .only_you_can_check
        .iter_mut()
        .chain(report.questions_for_you.iter_mut())
    {
        item.rests_on = by_question.get(&item.id).cloned();
    }
}

/// Gives each question a package locator speaks to the packages of the app's it names (backlog 0490), and each question
/// the reason nothing can check it where its catalog gives one, on both of the report's lists of questions.
pub fn point_to_packages(
    locators: &sv_check::locate::Locators,
    components: &[sv_check::sbom::Component],
    report: &mut sv_report::Report,
) {
    use sv_check::human::InThePackages;
    let mut by_question: BTreeMap<String, InThePackages> = BTreeMap::new();
    for locator in locators.packages() {
        let packages = locator.find(components);
        if packages.is_empty() {
            continue;
        }
        for id in &locator.questions {
            // The first package locator to speak to a question wins; two speaking to one would say two things.
            by_question
                .entry(id.clone())
                .or_insert_with(|| InThePackages {
                    finds: locator.finds.clone(),
                    packages: packages.clone(),
                    says: locator.says.clone(),
                });
        }
    }
    for item in report
        .only_you_can_check
        .iter_mut()
        .chain(report.questions_for_you.iter_mut())
    {
        item.in_the_packages = by_question.get(&item.id).cloned();
    }
}

/// Whether `file` names a place inside the app's folder: relative, and never climbing out of it.
fn inside(file: &str) -> bool {
    let path = Path::new(file);
    !file.is_empty()
        && path
            .components()
            .all(|c| matches!(c, Component::Normal(_) | Component::CurDir))
}

/// The file's lines, when it is a file of a size a person writes and reads as text, and its real place is inside
/// the app's folder: a link that points out of it is not followed.
fn file_lines(app_dir: &Path, path: &Path) -> Option<Vec<String>> {
    use sv_frameworks::paths::Canonical;
    let root = app_dir.canonical().ok()?;
    if !path.canonical().ok()?.starts_with(&root) {
        return None;
    }
    let meta = std::fs::symlink_metadata(path).ok()?;
    if !meta.is_file() || meta.len() > MOST_BYTES {
        return None;
    }
    let text = std::fs::read_to_string(path).ok()?;
    Some(text.lines().map(str::to_owned).collect())
}

/// A line, cut at `LINE_CHARS` characters with how many more there were said.
fn bounded(text: &str) -> String {
    sv_check::text::cut_with_count(text, LINE_CHARS)
}

#[cfg(test)]
mod code_lines_tests;
