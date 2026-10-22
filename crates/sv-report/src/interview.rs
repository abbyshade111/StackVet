//! The questions only a person can answer, written for the AI coding tool to ask them.
//!
//! The owner is not a programmer, and three lists of questions sat in the report waiting for them:
//! the design questions, the security notes, and the checks to make by hand. The tool that wrote the
//! app is sitting in the same conversation and knows the code, so the owner asked (26 September
//! 2026) for the tool to be given the questions and to interview them: one question at a time, with
//! what the code says offered as a tip, and the answers recorded where `sv` reads them.
//!
//! Who answered is the part that has to stay honest. An answer the owner gave the tool in the chat is
//! written as theirs (`by = "owner"`), and counts as theirs, *attested by the owner*, only once they
//! record it with `sv review`, which lists every such answer and records them together (backlog
//! 0618). When the owner asks the tool to answer, it answers from the code and records
//! `by = "ai-tool"`, reported one tier lower as *stated by the AI coding tool*. A security notes
//! answer is always recorded as the tool's (`Written by: AI coding tool`, through
//! `stackvet_record_answer`), since `sv` cannot tell whose words it holds: in `sv review` the owner
//! makes it theirs (`mine`) or confirms it as the tool's reading. A section without the line counts as
//! the tool's.
//!
//! Nothing here credits anything. This writes instructions; what the answers are worth is decided
//! where they are read (`sv_check::design`, `sv_check::notes`).

use crate::{Report, Status};
use sv_check::human::{Item, Route};

/// How the tool is told to ask. Kept apart from the questions so a test can hold each rule to it.
pub const HOW_TO_ASK: &str = "\
How to ask them. These are for the person you are building this app with, not for you to answer:
- Ask one question at a time, in the words of its \"Ask\" line, and wait for the answer. Never paste
  the whole list.
- Before each question, look at the code you wrote and offer what you found as a tip. Quote the exact
  lines the tip rests on, with the file and line number, and say plainly that it is your reading of
  the code, not a check: \"config.py, line 12: `PERMANENT_SESSION_LIFETIME = 1800`. As I read it, a
  session ends after 30 minutes without use; nothing has tested that. Is that what you want?\" With
  no lines to quote, say so rather than describe code you have not shown. The \"where to look\" line
  under each question says where the answer is usually found, and \"In the code\", when a question
  has it, shows lines StackVet found by their names or their shape: start from those, and say when they do not do
  what their names suggest.
- Record the answer as the person gave it, as described in each part below. \"Not sure\" is a good
  answer, and is recorded as not-sure. A \"no\" is useful too: the report lists it as something to fix.
  Before there is code, a decision made now is recorded as planned, naming the file it will be in.
- Write by = \"owner\" only for an answer the person gave you themselves. It counts as theirs only
  once they run `sv review` in their own terminal, which shows them every answer given as theirs and
  records them together; never run it for them. If they ask you to answer, say what the code does,
  answer from that, and write by = \"ai-tool\". The report shows that as \"stated by the AI coding
  tool\", which is weaker than the person's own word, and says so.
- The person can confirm what you answered, and then it counts as much as their own word. Suggest
  something they can look at or try themselves (a page to open, a thing to try), not a yes-or-no,
  and write it beside your answer as a proposal: confirmed = { by = \"ai-tool\", how = \"what to
  look at, and what it should show\" }. It counts only once the person has done it and recorded it by
  running `sv review` in their own terminal. Never run `sv review` for them, and never write their
  name or a `seal` yourself: you cannot confirm your own answer.
- A finding you think is a false alarm: read the code first, and tell the person what you found. You
  may write it under `[[finding-review]]` with by = \"ai-tool\" as a proposal; it counts for nothing
  until the person has looked and recorded it by running `sv review` in their own terminal. Never
  run it for them or write their name or a `seal` yourself, and never change correct code only to
  make a warning go away.
- Never answer yes to make the report look better. An answer is a record of how the app is, and a
  wrong yes hides the one thing the question exists to find.
- They are in order of what is most at stake: level 1, the baseline every app needs, first. When the
  person has had enough, stop. What is unanswered stays on the list for next time.
- Afterward, check the app again (stackvet_check) so the answers are read.";

/// The questions for one app, as the AI coding tool is given them.
pub fn text(report: &Report) -> String {
    text_with(report, &crate::fence::Fence::none())
}

/// The same, with the app's own text in it, its name and the lines of its code a question is about (backlog 0487),
/// put through `fence`: the questions and how to ask them are `sv`'s own (deep review R9).
pub fn text_with(report: &Report, fence: &crate::fence::Fence) -> String {
    let design: Vec<&Item> = of(report, Route::AnswerInTheManifest);
    let notes: Vec<&Item> = of(report, Route::WriteItDown);
    let by_hand: Vec<&Item> = of(report, Route::GoAndLook);
    let total = design.len() + notes.len() + by_hand.len();

    let mut out = format!(
        "QUESTIONS FOR THE OWNER of {}: {total} to ask.\n",
        fence.wrap(&report.app_name)
    );
    if total == 0 {
        out.push_str(
            "\nThere is nothing to ask: every question that applies to this app has an answer from \
             the owner. That does not make the answers right, and the report still says how much \
             of the app nothing has checked.\n",
        );
        return out;
    }
    out.push('\n');
    out.push_str(HOW_TO_ASK);
    out.push('\n');

    if !design.is_empty() {
        out.push_str(&format!(
            "\n1. HOW THE APP IS BUILT ({}). Record each answer in stackvet.toml, in the `[design]` \
             section, keyed by its id:\n   \"V8.3.1\" = {{ answer = \"yes\", where = \"server/auth.py\", \
             by = \"owner\" }}\n   `answer` is yes, no, not-sure, or planned (decided, not built yet); \
             `where` names the file that does it, or will, and is left out when there is none.\n",
            design.len()
        ));
        for item in &design {
            out.push_str(&question(item, fence));
            if let Some(means) = &item.where_means {
                out.push_str(&format!("   `where` names {means}.\n"));
            }
            if status_of(report, &item.id) == Some(Status::Stated) {
                out.push_str(
                    "   Only you, the AI coding tool, have answered this so far. Tell the person \
                     what you answered and why, and record their answer if they give one. Or \
                     suggest how they can see it for themselves, as a proposal beside your \
                     answer: confirmed = { by = \"ai-tool\", how = \"what to look at, and what it \
                     should show\" }. If they look and agree, they record it by running `sv review` \
                     in their own terminal; never run it for them.\n",
                );
            }
        }
    }

    if !notes.is_empty() {
        out.push_str(&format!(
            "\n{}. WRITTEN DECISIONS ({}). These ask what the rules are, which only the person can \
             decide. Record each answer, in a sentence or two of the person's words (or from the \
             code, if they ask you to answer), with stackvet_record_answer and the question's id. \
             sv marks what it records `Written by: AI coding tool`, which the report counts for less, \
             as stated by the AI coding tool, and asks again: sv cannot tell the person's word from \
             yours, so nothing you record counts as theirs. Show them what you recorded, and tell \
             them to run `sv review` in their own terminal: it asks whether each answer is their own \
             decision (they type `mine`) or your reading they confirm. Never write or change that \
             line for them, or run `sv review` for them. Without the MCP server, write \
             the answer under the question in security-notes.md (`sv notes` makes the file) and \
             start it with `Written by: AI coding tool`; a section without that line counts as \
             yours. Either way, write only under a question with nothing under it, or over an \
             answer marked `Written by: AI coding tool`: any other answer may be the person's own \
             words, so ask them instead of changing it.\n",
            if design.is_empty() { 1 } else { 2 },
            notes.len()
        ));
        for item in &notes {
            out.push_str(&question(item, fence));
        }
    }

    if !by_hand.is_empty() {
        out.push_str(&format!(
            "\n{}. CHECKS TO MAKE BY HAND ({}). Walk the person through each one, and help with the \
             part that is in the code. Then record what happened in stackvet.toml, in the \
             `[checked-by-hand]` section, keyed by its id:\n   \"V12.2.2\" = {{ result = \"done\", on = \
             \"2026-09-26\", by = \"owner\", how = \"Opened the live site; the padlock shows a \
             trusted certificate.\" }}\n   `result` is done, problem, or not-yet; `on` is the day; \
             `how` is one sentence of what was done and seen, in the person's words, and is \
             required. A check you made yourself, reading the code, is by = \"ai-tool\"; to propose \
             that the person make it too, add confirmed = {{ by = \"ai-tool\", how = \"what to look \
             at\" }} beside it, and they record it by running `sv review` in their own terminal. A problem is \
             worth recording: the report lists it as something to fix.\n",
            1 + usize::from(!design.is_empty()) + usize::from(!notes.is_empty()),
            by_hand.len()
        ));
        for item in &by_hand {
            out.push_str(&question(item, fence));
        }
    }
    out
}

fn of(report: &Report, route: Route) -> Vec<&Item> {
    report
        .questions_for_you
        .iter()
        .filter(|i| i.route == route)
        .collect()
}

fn status_of(report: &Report, id: &str) -> Option<Status> {
    report
        .requirements
        .iter()
        .find(|r| r.id == id)
        .map(|r| r.status)
}

fn question(item: &Item, fence: &crate::fence::Fence) -> String {
    // The plain question is what to ask (backlog 0427). The standard's words stay in the structured
    // result's `how` and in the report, rather than doubling every question in a text that is paged.
    let mut out = format!(
        "\n - {}: {}\n   Ask: {}\n",
        item.id,
        item.title,
        one_line(item.plain.as_deref().unwrap_or(&item.how))
    );
    if let Some(look) = &item.where_to_look {
        out.push_str(&format!("   Where to look: {}\n", one_line(look)));
    }
    out.push_str(&rests_on(item, fence));
    out.push_str(&in_the_packages(item, fence));
    if let Some(why) = &item.why_not_checked {
        out.push_str(&format!(
            "   Why StackVet cannot check it: {}\n",
            one_line(why)
        ));
    }
    out.push_str(&routes(item, fence));
    out.push_str(&outbound(item, fence));
    out.push_str(&pathways(item, fence));
    out.push_str(&personal_fields(item, fence));
    out.push_str(&crypto(item, fence));
    if let Some(token) = &item.token_audience {
        out.push_str(&format!(
            "   {}\n",
            token.audience_said(|name| fence.wrap(name))
        ));
    }
    out.push_str(&measured(item));
    out
}

/// The draft cryptographic inventory (backlog 0494), each piece of the app's text through `fence`, with the sentence
/// that says it is where to look and not evidence.
/// The checks that measure parts of a hand check's requirement, with this report's verdict on each (backlog 0610).
/// `sv`'s own words and the report's own verdicts, so nothing here goes through the fence.
fn measured(item: &Item) -> String {
    if item.measured_by.is_empty() {
        return String::new();
    }
    let mut out = format!("   {}\n", sv_check::human::MEASURED_SAY);
    for part in &item.measured_by {
        out.push_str(&format!(
            "     {}, {}: {}\n",
            part.id, part.what, part.status
        ));
    }
    out
}

fn crypto(item: &Item, fence: &crate::fence::Fence) -> String {
    let Some(found) = &item.crypto else {
        return String::new();
    };
    let mut out = format!("   {}\n", sv_check::crypto_inventory::INVENTORY_SAYS);
    for place in found.uses.iter().chain(&found.settings) {
        out.push_str(&format!(
            "     {} ({}, line {})\n",
            fence.wrap(&place.what),
            fence.wrap(&place.file),
            place.line
        ));
    }
    for file in &found.key_files {
        out.push_str(&format!(
            "     a key or certificate file, {} (not opened)\n",
            fence.wrap(file)
        ));
    }
    if !found.libraries.is_empty() {
        out.push_str(&format!(
            "     cryptography libraries among the packages: {}\n",
            found
                .libraries
                .iter()
                .map(|l| fence.wrap(l))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if found.more > 0 {
        out.push_str(&format!("     And {} more, not shown.\n", found.more));
    }
    out
}

/// The fields the app's schema shows that look like personal data (backlog 0497), each piece of the app's text through
/// `fence`, with the sentence that says they are where to look and not evidence.
fn personal_fields(item: &Item, fence: &crate::fence::Fence) -> String {
    let Some(found) = &item.personal_fields else {
        return String::new();
    };
    let mut out = String::new();
    if !found.fields.is_empty() {
        out.push_str(&format!("   {}\n", sv_check::personal_fields::FIELDS_SAY));
    }
    for field in &found.fields {
        out.push_str(&format!(
            "     {} ({}, line {})\n",
            fence.wrap(&field.name()),
            fence.wrap(&field.file),
            field.line
        ));
    }
    if found.more > 0 {
        out.push_str(&format!("     And {} more, not shown.\n", found.more));
    }
    if let Some(token) = &found.token {
        out.push_str(&format!("   {}\n", fence.wrap(&token.said())));
    }
    out
}

/// The ways into the app the code shows (backlog 0491), each piece of the app's text through `fence`, with the sentence
/// that says they are where to look and not evidence.
fn pathways(item: &Item, fence: &crate::fence::Fence) -> String {
    let Some(found) = &item.pathways else {
        return String::new();
    };
    let mut out = format!("   {}\n", sv_check::sign_in_paths::PATHWAYS_SAY);
    for way in &found.ways {
        out.push_str(&format!(
            "     {} ({}, line {})\n",
            fence.wrap(&way.what),
            fence.wrap(&way.file),
            way.line
        ));
    }
    if found.more > 0 {
        out.push_str(&format!("     And {} more, not shown.\n", found.more));
    }
    out
}

/// The outbound rules the folder's infrastructure files set (backlog 0496), each piece of the app's text through
/// `fence`, with the sentence that says they are where to look and not evidence.
fn outbound(item: &Item, fence: &crate::fence::Fence) -> String {
    use sv_check::outbound_rules::RULES_SAY;
    let Some(found) = &item.outbound else {
        return String::new();
    };
    let mut out = format!("   {RULES_SAY}\n");
    for r in &found.rules {
        out.push_str(&format!(
            "     {} ({}, line {}){}\n",
            fence.wrap(&r.what),
            fence.wrap(&r.file),
            r.line,
            if r.open_to_all {
                ": it lets the app reach everywhere"
            } else {
                ""
            }
        ));
    }
    if found.more > 0 {
        out.push_str(&format!("     And {} more, not shown.\n", found.more));
    }
    out
}

/// The app's routes and what is written on each (backlog 0493), each piece of the app's text through `fence`, with
/// the sentence that says they are where to look and not evidence.
fn routes(item: &Item, fence: &crate::fence::Fence) -> String {
    use sv_check::route_guards::{NONE_FOUND, ROUTES_SAY};
    let Some(table) = &item.routes else {
        return String::new();
    };
    let mut out = format!("   {ROUTES_SAY}\n");
    for r in &table.routes {
        let guards = if r.guards.is_empty() {
            NONE_FOUND.to_owned()
        } else {
            r.guards
                .iter()
                .map(|g| fence.wrap(g))
                .collect::<Vec<_>>()
                .join(", ")
        };
        out.push_str(&format!(
            "     {} {} ({}, line {}): {guards}\n",
            r.method,
            fence.wrap(&r.path),
            fence.wrap(&r.file),
            r.line
        ));
    }
    if table.more > 0 {
        out.push_str(&format!(
            "     And {} more {}, not shown.\n",
            table.more,
            if table.more == 1 { "route" } else { "routes" }
        ));
    }
    out
}

/// The packages of the app's a question is about (backlog 0490), each name through `fence`, with what the report says
/// of them and the sentence that they are where to look and not evidence.
fn in_the_packages(item: &Item, fence: &crate::fence::Fence) -> String {
    let Some(found) = &item.in_the_packages else {
        return String::new();
    };
    // None of them (backlog 0609): `says` is the locator's own sentence for that, and nothing was found by its name.
    if found.packages.is_empty() {
        return format!(
            "   In the packages, {}: none. {}\n",
            found.finds,
            one_line(&found.says)
        );
    }
    let packages: Vec<String> = found.packages.iter().map(|p| fence.wrap(p)).collect();
    format!(
        "   In the packages, {}: {}. {} {}\n",
        found.finds,
        packages.join(", "),
        one_line(&found.says),
        sv_check::human::PACKAGES_SAY
    )
}

/// The lines of the app's code a question is about (backlog 0487), each piece of the app's text through `fence`, and
/// the sentence that says they are where to look and not evidence.
fn rests_on(item: &Item, fence: &crate::fence::Fence) -> String {
    let Some(rests_on) = &item.rests_on else {
        return String::new();
    };
    let mut out = format!(
        "   In the code, {}. {}\n",
        rests_on.finds,
        sv_check::human::POINTERS_SAY
    );
    for place in &rests_on.places {
        out.push_str(&format!(
            "     {}, line {}:\n",
            fence.wrap(&place.file),
            place.line
        ));
        for (n, text) in &place.lines {
            out.push_str(&format!("       {n} | {}\n", fence.wrap(text)));
        }
    }
    if rests_on.more > 0 {
        out.push_str(&format!(
            "     And {} more {} like these, not shown.\n",
            rests_on.more,
            if rests_on.more == 1 {
                "place"
            } else {
                "places"
            }
        ));
    }
    out
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::HOW_TO_ASK;

    #[test]
    fn the_tool_is_told_whose_answer_is_whose() {
        // The rules that keep the tiers honest are only as good as the tool being told them.
        let flat = HOW_TO_ASK.split_whitespace().collect::<Vec<_>>().join(" ");
        for rule in [
            "one question at a time",
            "Quote the exact lines the tip rests on, with the file and line number",
            "your reading of the code, not a check",
            // Backlog 0618: the person's own answer, not one of the tool's they confirmed (that is `confirmed`).
            "Write by = \"owner\" only for an answer the person gave you themselves",
            "write by = \"ai-tool\"",
            "Never answer yes to make the report look better",
            "\"Not sure\" is a good answer",
        ] {
            assert!(
                flat.contains(rule),
                "the instructions lost {rule:?}:\n{HOW_TO_ASK}"
            );
        }
    }
}
