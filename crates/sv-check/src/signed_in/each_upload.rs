//! Every upload route stackvet.toml names, each run through the same checks (backlog 0232, part D, the owner's choice of
//! 16 October 2026). A finding on any route counts, as each check reports it. A credit stands only when every route gave
//! it, for the same check and requirements; one some routes gave and others did not is withheld, with which said so.
//! With one route nothing changes: the checks run as they always have.

use super::*;
use sv_manifest::UploadSection;

/// Runs `check` on each of `routes`, into `out`, and then keeps each credit the routes gave only when every route it
/// `applies` to gave it. A route it does not apply to (one that says it unpacks no archives, for the archive check) is
/// still run, so what it says is shown, and holds no credit back.
pub(super) fn on_each_upload(
    routes: &[UploadSection],
    applies: impl Fn(&UploadSection) -> bool,
    out: &mut Outcome,
    mut check: impl FnMut(&UploadSection, &mut Outcome),
) {
    let [first, ..] = routes else {
        return;
    };
    if routes.len() == 1 {
        check(first, out);
        return;
    }
    let mut earned: Vec<Vec<crate::Verified>> = Vec::new();
    let mut counted: Vec<&UploadSection> = Vec::new();
    for route in routes {
        out.steps
            .push(format!("the upload route `{}`:", route.path));
        let before = out.verified.len();
        check(route, out);
        let gave: Vec<crate::Verified> = out.verified.drain(before..).collect();
        if applies(route) {
            earned.push(gave);
            counted.push(route);
        }
    }
    let routes = counted;
    let paths: Vec<String> = routes.iter().map(|r| format!("`{}`", r.path)).collect();
    // Each credit, by its check and requirements, once, in the order the routes first gave it.
    let mut keys: Vec<(String, Vec<String>)> = Vec::new();
    for credit in earned.iter().flatten() {
        let key = (credit.check_id.clone(), credit.requirement_ids.clone());
        if !keys.contains(&key) {
            keys.push(key);
        }
    }
    for (check_id, ids) in keys {
        let gave: Vec<Option<&crate::Verified>> = earned
            .iter()
            .map(|e| {
                e.iter()
                    .find(|c| c.check_id == check_id && c.requirement_ids == ids)
            })
            .collect();
        if gave.iter().all(Option::is_some) {
            let credits: Vec<&crate::Verified> = gave.into_iter().flatten().collect();
            // In part when any route's was: the merged credit is a copy of one that was, if one was.
            let mut merged = (*credits.iter().find(|c| c.in_part).unwrap_or(&credits[0])).clone();
            merged.scope = if routes.len() == 1 {
                format!(
                    "{} (on {}, the one upload route it applies to)",
                    merged.scope, paths[0]
                )
            } else {
                format!(
                    "{} (on each of the {} upload routes: {})",
                    merged.scope,
                    routes.len(),
                    paths.join(", ")
                )
            };
            merged.evidence = credits
                .iter()
                .flat_map(|c| c.evidence.iter().cloned())
                .collect();
            out.verified.push(merged);
        } else {
            let missing: Vec<&str> = routes
                .iter()
                .zip(&gave)
                .filter(|(_, g)| g.is_none())
                .map(|(r, _)| r.path.as_str())
                .collect();
            out.not_assessed.push((
                ids.join(", "),
                format!(
                    "`{check_id}` held on some of the upload routes and was not credited on {}, so it is not \
                     credited on any: every route stackvet.toml names must hold.",
                    missing
                        .iter()
                        .map(|p| format!("`{p}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ));
        }
    }
}

#[cfg(test)]
#[path = "each_upload_tests.rs"]
mod tests;
