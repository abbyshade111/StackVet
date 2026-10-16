//! A member's own page, saved and fetched where it is served (backlog 0535, ADR-126): a `creates` entry marked
//! `page = true` saves a page holding a marked script, and its `read` address is fetched as the first user.
//!
//! It is judged as an uploaded page is (`served_upload_checks`): served as an attachment, with a `sandbox` policy,
//! or typed as something other than a page, the browser does not run its script as part of this app, and V3.2.1 is
//! credited in part; served as a page of this app with the script in it as it was saved, it is a finding. A `read`
//! address on a host of its own is not fetched: the run reaches only the app, and whether that host is another
//! origin is not something the run can see.

use super::*;

/// The mark the saved page carries, once as text and once inside its script.
pub(super) const PAGE_MARK: &str = "sv-probe-page-7c1d";

/// The page saved: the mark as text, to show the page was kept, and a script holding it, to show whether the
/// script was kept as it was written.
pub(super) fn saved_page() -> String {
    format!("<p>{PAGE_MARK}</p>{}", page_script())
}

/// The script inside the saved page, as it appears in an answer that kept it.
pub(super) fn page_script() -> String {
    format!("<script>window.svProbe=\"{PAGE_MARK}\"</script>")
}

/// Whether a `read` address names a host of its own, rather than a path on the app.
fn on_a_host_of_its_own(read: &str) -> bool {
    let lower = read.to_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("//")
}

/// Each `creates` entry marked `page = true`, saved by the first user and fetched where it is read back.
pub(super) fn saved_page_checks(
    http: &mut dyn Http,
    users: &UsersSection,
    a: &SignedIn,
    out: &mut Outcome,
) {
    const IDS: &str = "V3.2.1";
    for (i, create) in users.creates.iter().enumerate().filter(|(_, c)| c.page) {
        let n = i + 1;
        if let Some(read) = create.read.as_deref().filter(|r| on_a_host_of_its_own(r)) {
            out.not_assessed.push((
                IDS.to_owned(),
                format!(
                    "The page saved through {} is read at {read}, a host of its own, which this run does not \
                     reach. A page served from another origin is one of the controls V3.2.1 names; check by hand \
                     that the host differs from the app's own and that the page is served from it.",
                    create.path
                ),
            ));
            continue;
        }
        let page = saved_page();
        let mut session = a.session.clone();
        let (saved, _) = send_template(
            http,
            &format!("page-{n}-save"),
            &create.request(),
            &Values {
                marker: &page,
                ..Default::default()
            },
            &mut session,
            &users.private,
        );
        out.steps.push(format!(
            "saved a page holding a marked script through {} ({})",
            create.path,
            status(&saved)
        ));
        let Some(path) = saved.as_ref().filter(|_| accepted(&saved)).and_then(|r| {
            super::admin::record_path_with(create.read.as_deref(), create.id_field.as_deref(), r)
        }) else {
            out.not_assessed.push((
                IDS.to_owned(),
                format!(
                    "The page saved through {} was not seen saved ({}), or its answer did not say where it is \
                     read, so how it is served was not seen. Check the entry in `creates` in stackvet.toml, and \
                     its `read`.",
                    create.path,
                    status(&saved)
                ),
            ));
            continue;
        };
        let fetch_id = format!("page-{n}-fetch");
        let fetched = http.send(&get(&fetch_id, &path, &session));
        // A crash, a refusal, or an answer without the page is not how the page is served (ADR-021).
        let Some(fetched) = fetched
            .as_ref()
            .filter(|r| (200..300).contains(&r.status) && r.body.contains(PAGE_MARK))
        else {
            out.steps.push(format!(
                "fetched the saved page from {path} ({}): the page was not in the answer",
                status(&fetched)
            ));
            out.not_assessed.push((
                IDS.to_owned(),
                format!(
                    "The page saved through {} was not found at {path} ({}), so how it is served was not seen.",
                    create.path,
                    status(&fetched)
                ),
            ));
            continue;
        };
        let header = |name: &str| fetched.header(name).unwrap_or_default().to_lowercase();
        let (disposition, content_type, csp) = (
            header("content-disposition"),
            header("content-type"),
            header("content-security-policy"),
        );
        let attachment = disposition.contains("attachment");
        let sandboxed = csp.contains("sandbox");
        // No type is not "something other than a page": a browser sniffs it and renders the HTML (backlog 0306).
        let typed = !content_type.trim().is_empty();
        let not_html = typed
            && !content_type.contains("text/html")
            && !content_type.contains("application/xhtml+xml");
        let how = if attachment {
            Some("as an attachment")
        } else if sandboxed {
            Some("with a sandbox policy")
        } else if not_html {
            Some("as something other than a page")
        } else {
            None
        };
        let script_kept = fetched.body.contains(&page_script());
        out.steps.push(format!(
            "fetched the saved page from {path}: {}, its script {}",
            how.unwrap_or("served as a page"),
            if script_kept {
                "as it was saved"
            } else {
                "taken out or escaped"
            }
        ));
        match how {
            Some(how) => out.verified.push(
                crate::Verified::new(
                    PAGE_RENDERED.rule_id,
                    PAGE_RENDERED.requirement_ids,
                    format!("a page saved through {} and fetched from {path}: served {how}", create.path),
                )
                // One page, at the one address `read` names; anywhere else the app shows it was not tried.
                .in_part(),
            ),
            None if script_kept => out.findings.push(finding_on(
                vec![format!("page-{n}-save"), fetch_id],
                &PAGE_RENDERED,
                "A member's saved page is served for the browser to run as part of this app",
                Severity::High,
                format!(
                    "A page saved through {} came back from {path} {} with its script as it was saved, with no \
                     `Content-Disposition: attachment` and no sandbox policy, so a browser runs that script as \
                     part of this app.",
                    create.path,
                    if typed {
                        format!("as `{content_type}`")
                    } else {
                        "with no `Content-Type`, which a browser sniffs as a page".to_owned()
                    }
                ),
            )),
            None => out.not_assessed.push((
                IDS.to_owned(),
                format!(
                    "The page saved through {} came back from {path} as a page of this app, but with its script \
                     taken out or escaped, so whether a script in a member's page runs was not seen. Escaping \
                     is checked under output encoding, not here.",
                    create.path
                ),
            )),
        }
    }
}
