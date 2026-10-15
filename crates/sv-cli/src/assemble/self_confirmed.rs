//! The lines in stackvet.toml where the AI coding tool vouches for its own work (backlog 0543). In the three-tool trial
//! of 14 October 2026 one tool wrote `confirmed = { by = "ai-tool", ... }` on twelve design answers and another wrote
//! `[checked-by-hand] "V2.2.1" = { result = "done", by = "ai-tool" }`. `sv` counted both right, and said nothing the
//! tool would read as "stop": the confirmations sat in one gap among other reasons, the check by hand in no gap at
//! all. This names each such line in a gap of its own, which `stackvet_check` puts before anything else.

/// Whether `by` names the AI coding tool, as `sv_check::confirm::judge` and `sv review` read it.
fn names_the_tool(by: Option<&str>) -> bool {
    by.map(str::trim).is_some_and(|b| {
        b.eq_ignore_ascii_case(sv_check::design::AI_TOOL)
            || b.eq_ignore_ascii_case("AI coding tool")
    })
}

/// What the tool wrote of its own work: the design answers it confirmed, the checks by hand it confirmed, and each
/// line in words, in the file's order.
#[derive(Debug, Default)]
pub(super) struct SelfConfirmed {
    pub design: std::collections::BTreeSet<String>,
    pub hand: std::collections::BTreeSet<String>,
    pub lines: Vec<String>,
}

pub(super) fn of(manifest: &sv_manifest::Manifest) -> SelfConfirmed {
    let mut out = SelfConfirmed::default();
    for (id, answer) in &manifest.design {
        if answer
            .confirmed
            .as_ref()
            .is_some_and(|c| names_the_tool(c.by.as_deref()))
        {
            out.design.insert(id.clone());
            out.lines
                .push(format!("[design] {id}, confirmed by the AI coding tool"));
        }
    }
    for (id, check) in &manifest.checked_by_hand {
        let confirmed_by_tool = check
            .confirmed
            .as_ref()
            .is_some_and(|c| names_the_tool(c.by.as_deref()));
        if confirmed_by_tool {
            out.hand.insert(id.clone());
        }
        // A problem it reports is a finding, and counts as one: only a claim that something holds is named. One a
        // person confirmed is theirs when the confirmation holds, and named with its reason when it does not.
        let made_by_tool = check.result == sv_check::hand::DONE
            && names_the_tool(check.by.as_deref())
            && (check.confirmed.is_none() || confirmed_by_tool);
        match (made_by_tool, confirmed_by_tool) {
            (true, true) => out.lines.push(format!(
                "[checked-by-hand] {id}, checked by hand and confirmed by the AI coding tool"
            )),
            (true, false) => out.lines.push(format!(
                "[checked-by-hand] {id}, checked by hand by the AI coding tool"
            )),
            (false, true) => out.lines.push(format!(
                "[checked-by-hand] {id}, confirmed by the AI coding tool"
            )),
            (false, false) => {}
        }
    }
    out
}

impl SelfConfirmed {
    /// The gap that names each line, or `None` when there is none. Each kind is said only when there is one: a check
    /// by hand the tool writes down is its word, which setup prompt 6 asks for when the person describes one, so it is
    /// never called worthless, only never a check.
    pub(super) fn gap(&self) -> Option<sv_report::Gap> {
        if self.lines.is_empty() {
            return None;
        }
        let n = self.lines.len();
        let mut why = Vec::new();
        if self.lines.iter().any(|l| l.contains("confirmed by")) {
            why.push("A confirmation by the AI coding tool counts for nothing.");
        }
        if self.lines.iter().any(|l| l.contains("checked by hand")) {
            why.push(
                "A check by hand written with `by = \"ai-tool\"` counts only as the tool's word, never as a check, \
                 even when it passes on what the person saw.",
            );
        }
        Some(sv_report::Gap {
            what: format!(
                "{n} line{} in stackvet.toml that only your AI coding tool vouches for",
                if n == 1 { "" } else { "s" }
            ),
            why: format!(
                "{} Only `sv review`, run by the person in their own terminal, records a confirmation or a check \
                 as theirs: {}.",
                why.join(" "),
                self.lines.join("; ")
            ),
            reason: sv_report::GapReason::PersonOnly,
            requirements: Vec::new(),
        })
    }
}

#[cfg(test)]
#[path = "self_confirmed_tests.rs"]
mod tests;
