//! Values no sensible form would take, saved through the `owned` record's `create` (V2.2.1; backlog 0489, the owner's
//! choice of 20 October 2026; ADR-053, Later).
//!
//! Two kinds, both as the first user, once their own record has been read back:
//!
//! - **A text of 100,000 characters** in the marker field. Stored whole is a finding, at low severity: some fields
//!   take long text on purpose (a note's body), so it says the app holds the field to no length, not that it is wrong
//!   to. Refused as invalid is *checked in part*. The value carries the reflection mark at both ends, so the runner,
//!   which keeps only the start of an answer, keeps the text around each; the far end coming back is what shows it
//!   was stored whole, and the start coming back without it is a value cut short, which is said and credits nothing.
//! - **The ranges `limits` declares.** For each field, the value one below its least and one above its most; a record
//!   stored with either is a finding. Credited in part, for the declared fields only, when every value outside is
//!   refused and one inside is stored: the control, without which a refusal may be about something else (a string
//!   where a number was wanted, a form refused for any reason).

use super::*;
use crate::probes::REFLECTION_MARK;

/// How long the long text is: far past any title or comment, short of any upload limit.
pub(super) const LONG_TEXT: usize = 100_000;

/// The long text: the mark and a word to know the start by, filler, then the mark again and a word for the end.
pub(super) fn long_value() -> String {
    format!(
        "{REFLECTION_MARK}LongStart{}{REFLECTION_MARK}LongEnd",
        "x".repeat(LONG_TEXT)
    )
}

/// What a page showed of the long text: its end (stored whole), its start alone (cut short), or nothing of it.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Came {
    Whole,
    CutShort,
    NotSeen,
}

pub(super) fn came_back(response: &Option<ProbeResponse>) -> Came {
    let body = response
        .as_ref()
        .filter(|_| ok(response))
        .map_or("", |r| r.body.as_str());
    if body.contains(&format!("{REFLECTION_MARK}LongEnd")) {
        Came::Whole
    } else if body.contains(&format!("{REFLECTION_MARK}LongStart")) {
        Came::CutShort
    } else {
        Came::NotSeen
    }
}

/// A refusal that says the value was judged: 400, 413, or 422. A 403, a 429, or a crash says something else.
fn refused_as_invalid(response: &Option<ProbeResponse>) -> bool {
    response
        .as_ref()
        .is_some_and(|r| matches!(r.status, 400 | 413 | 422))
}

/// Runs as the first user, once their own record has been read back.
pub(super) fn input_limits_check(
    http: &mut dyn Http,
    users: &UsersSection,
    owned: &sv_manifest::OwnedSection,
    a: &Session,
    out: &mut Outcome,
) {
    let mut session = a.clone();
    long_text(http, users, owned, &mut session, out);
    declared_limits(http, users, owned, &mut session, out);
}

fn long_text(
    http: &mut dyn Http,
    users: &UsersSection,
    owned: &sv_manifest::OwnedSection,
    session: &mut Session,
    out: &mut Outcome,
) {
    let value = long_value();
    let (created, _) = send_template(
        http,
        "input-long-create",
        &owned.create,
        &Values {
            marker: &value,
            ..Default::default()
        },
        session,
        &users.private,
    );
    let what = format!("a record whose text is {LONG_TEXT} characters long");
    if refused_as_invalid(&created) {
        out.steps
            .push(format!("A saved {what}: refused ({})", status(&created)));
        out.verified.push(
            crate::Verified::new(
                INPUT_LENGTH.rule_id,
                INPUT_LENGTH.requirement_ids,
                format!(
                    "{what} refused as invalid ({}) through {}: one field, one kind of value no form should take",
                    status(&created),
                    owned.create.path
                ),
            )
            .in_part(),
        );
        return;
    }
    if !accepted(&created) {
        out.steps
            .push(format!("A saved {what}: {}", status(&created)));
        out.not_assessed.push((
            "V2.2.1".to_owned(),
            format!(
                "Whether text is held to a length: saving {what} was answered {}, which is not a refusal of the \
                 value, so it says nothing either way.",
                status(&created)
            ),
        ));
        return;
    }
    let Some(path) = created.as_ref().and_then(|r| record_path(owned, r)) else {
        out.not_assessed.push((
            "V2.2.1".to_owned(),
            format!("Whether text is held to a length: {what} was taken, and where it is read back is not known."),
        ));
        return;
    };
    let read = http.send(&get("input-long-read", &path, session));
    match came_back(&read) {
        Came::Whole => {
            out.steps.push(format!("A saved {what}: kept whole, and shown back at {path}"));
            out.findings.push(finding_on(
                vec!["input-long-create".to_owned(), "input-long-read".to_owned()],
                &INPUT_LENGTH,
                "Text is saved at any length",
                Severity::Low,
                format!(
                    "The first test user saved a record whose text was {LONG_TEXT} characters long through {}, and \
                     {path} showed it back whole: the app holds that field to no length. Some fields take long text \
                     on purpose; this says the limit is not there, not that the text is wrong.",
                    owned.create.path
                ),
            ));
        }
        Came::CutShort => out.steps.push(format!(
            "A saved {what}: taken, and shown back at {path} cut short; a value shortened as it is saved is not refused, \
             so nothing is credited"
        )),
        Came::NotSeen => out.not_assessed.push((
            "V2.2.1".to_owned(),
            format!("Whether text is held to a length: {what} was taken, and {path} did not show it."),
        )),
    }
}

fn declared_limits(
    http: &mut dyn Http,
    users: &UsersSection,
    owned: &sv_manifest::OwnedSection,
    session: &mut Session,
    out: &mut Outcome,
) {
    let mut checked: Vec<String> = Vec::new();
    for limit in &owned.limits {
        let field = &limit.field;
        if !owned.create.form.contains_key(field) && !owned.create.json.contains_key(field) {
            out.not_assessed.push((
                "V2.2.1".to_owned(),
                format!("`limits` names `{field}`, which `owned.create` does not send, so its range was not tried."),
            ));
            continue;
        }
        if limit.min > limit.max {
            out.not_assessed.push((
                "V2.2.1".to_owned(),
                format!("`limits` gives `{field}` a least value above its most, so its range was not tried."),
            ));
            continue;
        }
        let (Some(below), Some(above)) = (limit.min.checked_sub(1), limit.max.checked_add(1))
        else {
            continue;
        };
        let mut stored = Vec::new();
        let mut refused = 0;
        for (n, value) in [(0, limit.min), (1, below), (2, above)] {
            let id = format!("input-limit-{field}-{n}");
            let marker = format!("{REFLECTION_MARK}Limit{n}{field}");
            let mut create = owned.create.clone();
            for map in [&mut create.form, &mut create.json] {
                if let Some(v) = map.get_mut(field) {
                    *v = value.to_string();
                }
            }
            let (created, _) = send_template(
                http,
                &id,
                &create,
                &Values {
                    marker: &marker,
                    ..Default::default()
                },
                session,
                &users.private,
            );
            let kept = accepted(&created)
                && created
                    .as_ref()
                    .and_then(|r| record_path(owned, r))
                    .map(|path| http.send(&get(&format!("{id}-read"), &path, session)))
                    .is_some_and(|read| read.is_some_and(|r| r.body.contains(&marker)));
            match (n, kept) {
                (0, false) => {
                    out.not_assessed.push((
                        "V2.2.1".to_owned(),
                        format!(
                            "`{field}`'s range: the value {value}, inside it, was not saved and shown back ({}), so a \
                             refusal outside it would say nothing about the range.",
                            status(&created)
                        ),
                    ));
                    break;
                }
                (0, true) => {}
                (_, true) => stored.push((value, id)),
                (_, false) if refused_as_invalid(&created) => refused += 1,
                (_, false) => out.steps.push(format!(
                    "`{field}` = {value}: answered {}, neither saved nor refused as invalid",
                    status(&created)
                )),
            }
        }
        out.steps.push(format!(
            "`{field}` held to {} to {}: {} of the two values outside it saved",
            limit.min,
            limit.max,
            stored.len()
        ));
        if !stored.is_empty() {
            let values: Vec<String> = stored.iter().map(|(v, _)| v.to_string()).collect();
            out.findings.push(finding_on(
                stored.iter().flat_map(|(_, id)| [id.clone(), format!("{id}-read")]).collect(),
                &INPUT_RANGE,
                "A value outside its declared range is saved",
                Severity::Medium,
                format!(
                    "stackvet.toml says `{field}` is held to {} to {}. The first test user saved a record with `{field}` \
                     set to {}, through {}, and it was stored as sent.",
                    limit.min,
                    limit.max,
                    values.join(" and "),
                    owned.create.path
                ),
            ));
        } else if refused == 2 {
            checked.push(format!("`{field}` ({} to {})", limit.min, limit.max));
        }
    }
    if !checked.is_empty()
        && !out
            .findings
            .iter()
            .any(|f| f.rule_id == INPUT_RANGE.rule_id)
    {
        out.verified.push(
            crate::Verified::new(
                INPUT_RANGE.rule_id,
                INPUT_RANGE.requirement_ids,
                format!(
                    "values one past each end of the declared range refused, and one inside it saved, for {}: the \
                     declared fields only",
                    checked.join(", ")
                ),
            )
            .in_part(),
        );
    }
}
