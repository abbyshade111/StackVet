//! Ticking the checklist before going live (backlog 0604; ADR-121, decision 4).
//!
//! The checklist is `before-going-live.md`, written again with each report, so a box ticked there would be lost and
//! would prove nothing. Here, after everything else `sv review` goes through, each line of the last sealed report's
//! checklist that is not yet ticked is offered, and is ticked only when the person types `owner`:
//!
//! - a line naming a requirement `sv` accepts a check by hand for (`data/human-checks.json`) is recorded as that check,
//!   `[checked-by-hand]` with `result = "done"`, `by = "owner"`, today, the line as `how`, and a seal, as `sv review`
//!   records one now. A tick widens nothing: a requirement outside that file is never recorded as checked by hand;
//! - any other tick is a sealed note in `[[checklist-done]]`, which the next checklist shows as done, with when, and
//!   which credits nothing.

use super::*;

/// One line of the last sealed report's checklist, as `report.json` keeps it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Offered {
    pub key: String,
    pub what: String,
    pub requirements: Vec<String>,
    /// Ticked already, as the report that wrote it says.
    pub done: bool,
}

/// The checklist lines of the sealed report in `folder`, or why none is read.
pub(super) fn checklist_lines(folder: &Path) -> Result<Vec<Offered>, String> {
    let report = sealed_report(folder)?;
    let lines = report
        .get("checklist")
        .and_then(|c| c.as_array())
        .ok_or_else(|| {
            "its report.json has no checklist, as a report written before 16 October 2026 has not"
                .to_owned()
        })?;
    Ok(lines
        .iter()
        .filter_map(|line| {
            let text = |k: &str| line.get(k).and_then(|v| v.as_str()).map(str::to_owned);
            Some(Offered {
                key: text("key")?,
                what: text("what")?,
                requirements: line
                    .get("requirements")
                    .and_then(|r| r.as_array())
                    .map(|ids| {
                        ids.iter()
                            .filter_map(|i| i.as_str().map(str::to_owned))
                            .collect()
                    })
                    .unwrap_or_default(),
                done: line.get("done_on").is_some(),
            })
        })
        .collect())
}

/// Goes through the checklist lines not yet ticked, and records each one the owner ticks.
#[allow(clippy::too_many_arguments)]
pub(super) fn tick_checklist(
    app_dir: &Path,
    manifest: &sv_manifest::Manifest,
    manifest_path: &Path,
    doc: &mut toml_edit::DocumentMut,
    today: Day,
    key: &Signer,
    checker: &Checker,
    input: &mut dyn BufRead,
    out: &mut dyn Write,
) -> Result<()> {
    let folder = app_dir.join(sv_frameworks::names::REPORT_DIR);
    let lines = match checklist_lines(&folder) {
        Ok(lines) => lines,
        Err(why) => {
            writeln!(
                out,
                "\nThe checklist before going live was not offered: no report sv can show it wrote was read in {} \
                 ({why}). Write the report with `sv report`, then run `sv review` again to tick its lines.",
                folder.display()
            )?;
            return Ok(());
        }
    };
    let accepted: std::collections::BTreeSet<String> =
        sv_check::human::HumanChecks::load(&sv_frameworks::data::file("human-checks.json"))?
            .checks
            .into_iter()
            .map(|c| c.id)
            .collect();
    // A line about requirements that all have a check by hand recorded already is answered there; a line ticked
    // already is done.
    let by_hand = |line: &Offered| -> Vec<String> {
        line.requirements
            .iter()
            .filter(|id| accepted.contains(*id) && !manifest.checked_by_hand.contains_key(*id))
            .cloned()
            .collect()
    };
    let waiting: Vec<&Offered> = lines
        .iter()
        .filter(|line| !line.done && !manifest.checklist_done.iter().any(|e| e.key == line.key))
        .filter(|line| {
            let named_accepted = line.requirements.iter().any(|id| accepted.contains(id));
            !named_accepted || !by_hand(line).is_empty()
        })
        .collect();
    if waiting.is_empty() {
        return Ok(());
    }
    writeln!(
        out,
        "\nThe checklist before going live: {} not yet ticked. Tick a line only when you have done it. A tick on a line \
         about a requirement sv accepts a check by hand for is recorded as your check by hand of it, today; any other \
         tick only marks the line done in the next checklist. A tick credits nothing else.",
        match waiting.len() {
            1 => "1 line".to_owned(),
            n => format!("{n} lines"),
        }
    )?;
    let total = waiting.len();
    // Requirements recorded by hand in this run: a requirement named on two lines is recorded once, from the first.
    let mut recorded_now: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (n, line) in waiting.into_iter().enumerate() {
        let named_accepted = line.requirements.iter().any(|id| accepted.contains(id));
        if named_accepted && by_hand(line).iter().all(|id| recorded_now.contains(id)) {
            writeln!(
                out,
                "\n[{} of {total}] {} Its check by hand was recorded above, from another line.",
                n + 1,
                line.what
            )?;
            continue;
        }
        writeln!(
            out,
            "\n[{} of {total}] {}{}",
            n + 1,
            line.what,
            if line.requirements.is_empty() {
                String::new()
            } else {
                format!(" ({})", line.requirements.join(", "))
            }
        )?;
        let ticked = loop {
            let Some(answer) = ask(
                input,
                out,
                "Type `owner` if you have done this and it is your app, to tick it; or press Enter to leave it.\n> ",
            )?
            else {
                return Ok(());
            };
            if answer.is_empty() {
                writeln!(out, "  Left as it is.")?;
                break false;
            }
            if answer.eq_ignore_ascii_case(sv_check::design::OWNER) {
                break true;
            }
            writeln!(
                out,
                "  Only the app's owner ticks the checklist. Type `owner` if that is you."
            )?;
        };
        if !ticked {
            continue;
        }
        let hand: Vec<String> = by_hand(line)
            .into_iter()
            .filter(|id| !recorded_now.contains(id))
            .collect();
        // A line that also names requirements no check by hand is accepted for is noted done as well, so the next
        // checklist shows it ticked.
        let also_note = line.requirements.iter().any(|id| !accepted.contains(id));
        if hand.is_empty() || also_note {
            let mut entry = sv_manifest::ChecklistDone {
                key: line.key.clone(),
                what: line.what.clone(),
                by: sv_check::design::OWNER.to_owned(),
                on: today.show(),
                seal: None,
            };
            entry.seal = Some(
                key.seal(&sv_check::seal::as_strs(
                    &sv_check::seal::checklist_done_fields(&entry),
                ))
                .map_err(anyhow::Error::msg)?,
            );
            push_checklist_done(doc, &entry);
            let wanted = entry.key.clone();
            save(manifest_path, doc, &|m| {
                m.checklist_done.iter().any(|e| {
                    e.key == wanted
                        && sv_check::seal::owner_recorded(
                            checker,
                            e.seal.as_deref(),
                            &sv_check::seal::checklist_done_fields(e),
                        )
                        .is_ok()
                })
            })?;
            writeln!(
                out,
                "  Ticked. The next checklist shows it done; it credits nothing."
            )?;
        }
        if !hand.is_empty() {
            for id in &hand {
                let mut entry = sv_manifest::HandCheck {
                    result: "done".to_owned(),
                    on: Some(today.show()),
                    by: Some(sv_check::design::OWNER.to_owned()),
                    how: Some(format!(
                        "Ticked on the checklist before going live: {}",
                        line.what
                    )),
                    confirmed: None,
                    seal: None,
                };
                entry.seal = Some(
                    key.seal(&sv_check::seal::as_strs(
                        &sv_check::seal::hand_check_fields(id, &entry),
                    ))
                    .map_err(anyhow::Error::msg)?,
                );
                set_hand_check(doc, id, &entry);
            }
            save(manifest_path, doc, &|m| {
                hand.iter().all(|id| {
                    m.checked_by_hand.get(id).is_some_and(|h| {
                        sv_check::seal::owner_recorded(
                            checker,
                            h.seal.as_deref(),
                            &sv_check::seal::hand_check_fields(id, h),
                        )
                        .is_ok()
                    })
                })
            })?;
            writeln!(
                out,
                "  Recorded as your check by hand of {}, today.",
                hand.join(", ")
            )?;
            recorded_now.extend(hand);
        }
    }
    Ok(())
}

/// A `[[checklist-done]]` entry, added after any there.
fn push_checklist_done(doc: &mut toml_edit::DocumentMut, entry: &sv_manifest::ChecklistDone) {
    let mut table = toml_edit::Table::new();
    table.insert("key", toml_edit::value(&entry.key));
    table.insert("what", toml_edit::value(&entry.what));
    table.insert("by", toml_edit::value(&entry.by));
    table.insert("on", toml_edit::value(&entry.on));
    if let Some(seal) = &entry.seal {
        table.insert("seal", toml_edit::value(seal));
    }
    match doc
        .get_mut("checklist-done")
        .and_then(|item| item.as_array_of_tables_mut())
    {
        Some(tables) => tables.push(table),
        None => {
            let mut tables = toml_edit::ArrayOfTables::new();
            tables.push(table);
            doc.insert("checklist-done", toml_edit::Item::ArrayOfTables(tables));
        }
    }
}

/// A `[checked-by-hand]` entry for `id`, written whole; the table is made when there is none.
fn set_hand_check(doc: &mut toml_edit::DocumentMut, id: &str, entry: &sv_manifest::HandCheck) {
    let mut table = toml_edit::Table::new();
    table.insert("result", toml_edit::value(&entry.result));
    for (name, value) in [
        ("on", &entry.on),
        ("by", &entry.by),
        ("how", &entry.how),
        ("seal", &entry.seal),
    ] {
        if let Some(value) = value {
            table.insert(name, toml_edit::value(value));
        }
    }
    if doc
        .get("checked-by-hand")
        .and_then(|i| i.as_table())
        .is_none()
    {
        let mut outer = toml_edit::Table::new();
        outer.set_implicit(true);
        doc.insert("checked-by-hand", toml_edit::Item::Table(outer));
    }
    if let Some(outer) = doc
        .get_mut("checked-by-hand")
        .and_then(|i| i.as_table_mut())
    {
        outer.insert(id, toml_edit::Item::Table(table));
    }
}

#[cfg(test)]
#[path = "ticks_tests.rs"]
mod tests;
