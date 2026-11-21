//! ConfigVet's first rules (the owner's choice, 20 November 2026; backlog 0687):
//!
//! - [`NODE_ENV`]: an Express app's containers run with `NODE_ENV` unset or not `production`.
//! - [`SESSION_COOKIE_SECURE`] and [`CSRF_COOKIE_SECURE`]: a Django app's settings leave its cookies without `Secure`.
//! - [`CONTAINER_ROOT`]: a container runs as root.
//!
//! Each is asked of every deployment unit and answers with one of the five outcomes. A setting left out is a finding
//! only when the table of defaults holds the default for the version in use; otherwise it is not assessed.

use crate::defaults::{Table, Version};
use crate::dockerfile::{self, Effective};
use crate::outcome::{Assessed, Outcome, Place, Rule};
use crate::unit::{Image, Layer, Unit};
use crate::{Dockerfile, Folder};
use sv_core::Severity;

/// `sv`'s rule for a Dockerfile's final stage setting a development mode (`sv-check`'s `launch::DEV_SERVER`), named
/// here as text so this crate does not build `sv`'s checks.
pub const SV_DEV_SERVER: &str = "config.development-server-started";

pub const NODE_ENV: Rule = Rule {
    id: "cv.express-node-env",
    setting: "NODE_ENV",
    // V13.4.2: debug modes off in production. V16.5.1: a generic message on an unexpected error, no stack trace.
    // Express outside production is its development mode, and its default error handler sends the stack trace.
    requirement_ids: &["V13.4.2", "V16.5.1"],
    cwe: &["CWE-489", "CWE-209"],
    severity: Severity::Medium,
    title: "An Express app runs without NODE_ENV=production",
    impact: "Express runs in development mode, and its default error handler sends the error's stack trace to \
             whoever made the request: file paths, library versions, and sometimes values from the code.",
    fix: "Set NODE_ENV=production where the container is run (the compose service's `environment`, the Kubernetes \
          container's `env`, or the ECS container definition), or with `ENV NODE_ENV=production` in the Dockerfile's \
          final stage.",
    scope: "Express's environment for",
};

pub const SESSION_COOKIE_SECURE: Rule = Rule {
    id: "cv.django-session-cookie-secure",
    setting: "SESSION_COOKIE_SECURE",
    // V3.3.1: cookies carry the Secure attribute (and a __Host- or __Secure- prefix, which this does not read).
    requirement_ids: &["V3.3.1"],
    cwe: &["CWE-614"],
    severity: Severity::Medium,
    title: "Django's session cookie is sent without the Secure attribute",
    impact: "A browser also sends the session cookie over plain HTTP, where anyone on the network path can read it \
             and take over the session.",
    fix: "Set SESSION_COOKIE_SECURE = True in the settings the production app runs with.",
    scope: "The Secure attribute of Django's session cookie, for",
};

pub const CSRF_COOKIE_SECURE: Rule = Rule {
    id: "cv.django-csrf-cookie-secure",
    setting: "CSRF_COOKIE_SECURE",
    requirement_ids: &["V3.3.1"],
    cwe: &["CWE-614"],
    severity: Severity::Low,
    title: "Django's CSRF cookie is sent without the Secure attribute",
    impact: "A browser also sends the CSRF cookie over plain HTTP, where anyone on the network path can read it.",
    fix: "Set CSRF_COOKIE_SECURE = True in the settings the production app runs with.",
    scope: "The Secure attribute of Django's CSRF cookie, for",
};

pub const CONTAINER_ROOT: Rule = Rule {
    id: "cv.container-root",
    setting: "the user the container runs as",
    // No ASVS 5.0 requirement asks this of a container; the finding cites none (the owner's choice, 20 November 2026).
    requirement_ids: &[],
    cwe: &["CWE-250"],
    severity: Severity::Medium,
    title: "A container runs as root",
    impact: "Whoever takes over the app's process holds root inside the container, which makes breaking out of it, \
             or changing what the image holds, much easier.",
    fix: "Add `USER` with an unprivileged user to the Dockerfile's final stage (the official node image has one, \
          `node`), or set `user:` in the compose service, `runAsNonRoot: true` and a non-zero `runAsUser` in the \
          Kubernetes securityContext, or `user` in the ECS container definition.",
    scope: "The user of",
};

/// Every rule, with what it settled.
pub fn run(folder: &Folder, table: &Table) -> Vec<(&'static Rule, Vec<Assessed>)> {
    vec![
        (&NODE_ENV, node_env(folder, table)),
        (
            &SESSION_COOKIE_SECURE,
            django_setting(folder, table, &SESSION_COOKIE_SECURE),
        ),
        (
            &CSRF_COOKIE_SECURE,
            django_setting(folder, table, &CSRF_COOKIE_SECURE),
        ),
        (&CONTAINER_ROOT, container_root(folder, table)),
    ]
}

/// What a rule is asked about: a Dockerfile on its own, or a unit another layer describes.
enum Target<'a> {
    Docker(&'a Dockerfile),
    Unit(&'a Unit),
}

impl Target<'_> {
    fn describe(&self) -> String {
        match self {
            Target::Docker(d) => format!("the image built from {}", d.file),
            Target::Unit(u) => u.describe(),
        }
    }

    fn place(&self) -> Place {
        match self {
            Target::Docker(d) => Place {
                file: d.file.clone(),
                line: d.stages.last().map_or(1, |s| s.from_line),
                layer: Layer::Dockerfile,
            },
            Target::Unit(u) => Place {
                file: u.file.clone(),
                line: u.line,
                layer: u.layer,
            },
        }
    }
}

/// Every unit, and every Dockerfile that no compose service builds (a service built from one speaks for it).
fn targets(folder: &Folder) -> Vec<Target<'_>> {
    let built: Vec<&str> = folder
        .units
        .iter()
        .filter_map(|u| u.build.as_ref().map(|b| b.dockerfile.as_str()))
        .collect();
    let mut out: Vec<Target> = folder
        .dockerfiles
        .iter()
        .filter(|d| !built.contains(&d.file.as_str()))
        .map(Target::Docker)
        .collect();
    out.extend(folder.units.iter().map(Target::Unit));
    out
}

/// What a unit's container starts from.
enum Start<'a> {
    Built(&'a Dockerfile, Effective),
    BuildUnread(String),
    Image(Image),
    Nothing,
}

fn start<'a>(folder: &'a Folder, target: &Target<'a>) -> Start<'a> {
    match target {
        Target::Docker(d) => Start::Built(d, dockerfile::effective(&d.stages)),
        Target::Unit(u) => match (&u.build, &u.image) {
            (Some(build), _) => match folder.dockerfile(&build.dockerfile) {
                Some(d) => Start::Built(d, dockerfile::effective(&d.stages)),
                None => Start::BuildUnread(build.dockerfile.clone()),
            },
            (None, Some(image)) => Start::Image(Image::parse(image)),
            (None, None) => Start::Nothing,
        },
    }
}

fn unknown(why: impl Into<String>) -> Outcome {
    Outcome::Unknown { why: why.into() }
}

fn assessed(
    rule: &Rule,
    target: &Target,
    outcome: Outcome,
    reported_by: Option<&'static str>,
) -> Assessed {
    Assessed {
        rule_id: rule.id,
        setting: rule.setting,
        unit: target.describe(),
        unit_at: target.place(),
        outcome,
        reported_by,
    }
}

// ---- NODE_ENV -------------------------------------------------------------------------------------------------

/// Whether any `package.json` depends on Express, and its major version when every one that does agrees on it.
fn express(folder: &Folder) -> Option<Option<u32>> {
    let mut majors = Vec::new();
    for (_, text) in &folder.package_jsons {
        let Ok(json) = serde_json::from_str::<serde_json::Value>(text) else {
            continue;
        };
        if let Some(spec) = json
            .get("dependencies")
            .and_then(|d| d.get("express"))
            .and_then(serde_json::Value::as_str)
        {
            majors.push(npm_major(spec));
        }
    }
    let first = *majors.first()?;
    Some(if majors.iter().all(|m| *m == first) {
        first
    } else {
        None
    })
}

/// The one major version an npm range allows: `^4.21.2`, `~5.1.0`, `4.x`, `=5.0.1`, `5`. `None` for a range across
/// majors (`>=4`, `*`, `latest`, `4 || 5`) or one that is not a version at all (a git address, a path).
pub fn npm_major(spec: &str) -> Option<u32> {
    let spec = spec.trim();
    if spec.contains("||") || spec.contains(' ') || spec.contains('>') || spec.contains('<') {
        return None;
    }
    let bare = spec.trim_start_matches(['^', '~', '=', 'v']);
    let digits: String = bare.chars().take_while(char::is_ascii_digit).collect();
    let rest = &bare[digits.len()..];
    if digits.is_empty() || !(rest.is_empty() || rest.starts_with('.')) {
        return None;
    }
    digits.parse().ok()
}

fn node_env(folder: &Folder, table: &Table) -> Vec<Assessed> {
    let Some(express_major) = express(folder) else {
        return Vec::new();
    };
    let in_scope: Vec<Target> = targets(folder)
        .into_iter()
        .filter(|t| runs_node(folder, t))
        .collect();
    if in_scope.is_empty() {
        return vec![Assessed {
            rule_id: NODE_ENV.id,
            setting: NODE_ENV.setting,
            unit: "the Express app".to_owned(),
            unit_at: Place {
                file: folder.package_jsons[0].0.clone(),
                line: 1,
                layer: Layer::Dockerfile,
            },
            outcome: unknown(
                "the app depends on Express, and no Dockerfile, compose file, Kubernetes manifest, or Terraform plan \
                 in the folder runs it on the official node image or sets NODE_ENV for it",
            ),
            reported_by: None,
        }];
    }
    in_scope
        .iter()
        .map(|target| {
            let (outcome, by) = node_env_of(folder, table, target, express_major);
            assessed(&NODE_ENV, target, outcome, by)
        })
        .collect()
}

/// Whether a target runs Node.js, as far as the files show: it names `NODE_ENV`, or it starts from the official node
/// image.
fn runs_node(folder: &Folder, target: &Target) -> bool {
    if let Target::Unit(u) = target
        && u.env("NODE_ENV").is_some()
    {
        return true;
    }
    match start(folder, target) {
        Start::Built(_, eff) => {
            eff.env("NODE_ENV").is_some() || Image::parse(&eff.base).is_official("node")
        }
        Start::Image(image) => image.is_official("node"),
        Start::BuildUnread(_) | Start::Nothing => false,
    }
}

fn node_env_of(
    folder: &Folder,
    table: &Table,
    target: &Target,
    express_major: Option<u32>,
) -> (Outcome, Option<&'static str>) {
    if let Target::Unit(u) = target {
        if let Some(var) = u.env("NODE_ENV") {
            let at = Place {
                file: u.file.clone(),
                line: var.line,
                layer: u.layer,
            };
            return (set_to(var.value.as_deref(), "production", at), None);
        }
        if let Some(elsewhere) = &u.env_elsewhere {
            return (
                unknown(format!(
                    "NODE_ENV may be set in `{elsewhere}`, which the file points to and cv does not read"
                )),
                None,
            );
        }
    }
    match start(folder, target) {
        Start::Built(d, eff) => match eff.env("NODE_ENV") {
            Some((setting, final_stage)) => {
                let at = Place {
                    file: d.file.clone(),
                    line: setting.line,
                    layer: Layer::Dockerfile,
                };
                let outcome = set_to(Some(&setting.value), "production", at);
                // `sv check` reports a final stage that sets a development mode; that finding is `sv`'s.
                let svs = *final_stage
                    && matches!(
                        setting.value.to_ascii_lowercase().as_str(),
                        "development" | "local"
                    );
                (outcome, svs.then_some(SV_DEV_SERVER))
            }
            None => (
                missing_node_env(table, &Image::parse(&eff.base), express_major),
                None,
            ),
        },
        Start::BuildUnread(file) => (
            unknown(format!("it is built from {file}, which cv could not read")),
            None,
        ),
        Start::Image(image) if image.is_official("node") => {
            (missing_node_env(table, &image, express_major), None)
        }
        Start::Image(image) => (
            unknown(format!(
                "the image `{}` may set NODE_ENV itself; cv reads files, not images",
                image.repository
            )),
            None,
        ),
        Start::Nothing => (unknown("it names no image and no build"), None),
    }
}

/// A set value: safe when it is `safe` (case aside), unknown when it comes from a variable or another place.
fn set_to(value: Option<&str>, safe: &str, at: Place) -> Outcome {
    match value {
        None => unknown(format!(
            "it is set in {} line {} from a value the file does not hold",
            at.file, at.line
        )),
        Some(v) if Image::names_variable(v) => unknown(format!(
            "it is set in {} line {} from a variable (`{v}`), decided when the file is used",
            at.file, at.line
        )),
        Some(v) if v.eq_ignore_ascii_case(safe) => Outcome::SetSafe {
            value: v.to_owned(),
            at,
        },
        Some(v) => Outcome::SetUnsafe {
            value: v.to_owned(),
            at,
        },
    }
}

fn missing_node_env(table: &Table, base: &Image, express_major: Option<u32>) -> Outcome {
    if !base.is_official("node") {
        return unknown(format!(
            "the base image `{}` may set NODE_ENV itself; cv reads files, not images",
            base.repository
        ));
    }
    let Some(node) = base.major() else {
        return unknown(format!(
            "the node image's tag (`{}`) does not say which Node.js version it is, and the table's defaults are \
             for the image's versions 22 and 24",
            base.tag.as_deref().unwrap_or("latest")
        ));
    };
    let Some(image_default) = table.holding("docker-node.node-env", Version::Major(node)) else {
        return unknown(format!(
            "the table of defaults has not read the node image's version {node}"
        ));
    };
    let Some(express) = express_major else {
        return unknown(
            "the package.json's Express version is not one major version, so which default applies is not known",
        );
    };
    let Some(express_default) = table.holding("express.env", Version::Major(express)) else {
        return unknown(format!(
            "the table of defaults has not read Express {express}"
        ));
    };
    Outcome::MissingDefaultUnsafe {
        defaults: vec![express_default.clone(), image_default.clone()],
    }
}

// ---- Django -----------------------------------------------------------------------------------------------------

/// Whether a Python dependency file names Django, and the releases it allows when every one that does agrees.
fn django(folder: &Folder) -> Option<Option<Version>> {
    let mut found = Vec::new();
    for (_, text) in &folder.python_deps {
        for line in text.lines() {
            if let Some(spec) = django_spec(line) {
                found.push(pip_releases(&spec));
            }
        }
    }
    let first = *found.first()?;
    Some(if found.iter().all(|v| *v == first) {
        first
    } else {
        None
    })
}

/// The version part of a line that asks for Django: `Django==5.2.1`, `django[argon2]>=4.2,<5.0`, a
/// `"Django>=5.1,<5.2"` in `pyproject.toml`, or `django = "==5.1"` in a Pipfile or Poetry table.
pub fn django_spec(line: &str) -> Option<String> {
    let line = line.split('#').next().unwrap_or_default();
    let lower = line.to_ascii_lowercase();
    let at = lower.find("django")?;
    let before = lower[..at].trim();
    if !(before.is_empty()
        || before.ends_with('"')
        || before.ends_with('\'')
        || before.ends_with('['))
    {
        return None;
    }
    let after = &line[at + "django".len()..];
    if after.starts_with(|c: char| c.is_alphanumeric() || c == '-' || c == '_') {
        return None; // `django-environ`, `djangorestframework`
    }
    let after = match after.find(']') {
        Some(end) if after.starts_with('[') => &after[end + 1..],
        _ => after,
    };
    let spec: String = after
        .trim_start_matches([' ', '"', '\'', '='])
        .trim_start()
        .chars()
        .take_while(|c| !matches!(c, '"' | '\'' | ';'))
        .collect();
    // A Poetry or Pipfile value `"==5.1"` lost its `==` to the trim above: put a bare version back as a pin.
    let spec = spec.trim().to_owned();
    if spec.starts_with(|c: char| c.is_ascii_digit()) {
        return Some(format!("=={spec}"));
    }
    Some(spec)
}

/// The releases a pip requirement allows, as `[low, high)` in `major.minor`; `None` when either end is open.
pub fn pip_releases(spec: &str) -> Option<Version> {
    let mut low: Option<(u32, u32)> = None;
    let mut high: Option<(u32, u32)> = None;
    for part in spec.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let op_len = part
            .find(|c: char| c.is_ascii_digit())
            .unwrap_or(part.len());
        let (op, version) = part.split_at(op_len);
        let parts: Vec<u32> = version
            .trim()
            .split('.')
            .map_while(|p| p.parse().ok())
            .collect();
        let (major, minor) = match parts.as_slice() {
            [major] => (*major, 0),
            [major, minor, ..] => (*major, *minor),
            [] => return None,
        };
        match op.trim() {
            "==" | "===" | "~" => {
                low = Some((major, minor));
                high = Some((major, minor + 1));
            }
            "~=" if parts.len() >= 3 => {
                low = Some((major, minor));
                high = Some((major, minor + 1));
            }
            "~=" | "^" => {
                low = Some((major, minor));
                high = Some((major + 1, 0));
            }
            ">=" | ">" => low = Some((major, minor)),
            "<" if parts.len() >= 3 && parts[2] > 0 => high = Some((major, minor + 1)),
            "<" => high = Some((major, minor)),
            "<=" => high = Some((major, minor + 1)),
            _ => return None,
        }
    }
    Some(Version::Releases(low?, high?))
}

/// Each assignment of `name` at the start of a line in a settings module: the file, line, and value as written.
fn assignments<'a>(folder: &'a Folder, name: &str) -> Vec<(&'a str, usize, String)> {
    let mut out = Vec::new();
    for (file, text) in &folder.django_settings {
        for (index, line) in text.lines().enumerate() {
            let Some(rest) = line.strip_prefix(name) else {
                continue;
            };
            let rest = rest.trim_start();
            if let Some(value) = rest.strip_prefix('=')
                && !value.starts_with('=')
            {
                let value = value.split('#').next().unwrap_or_default().trim();
                out.push((file.as_str(), index + 1, value.to_owned()));
            }
        }
    }
    out
}

fn django_setting(folder: &Folder, table: &Table, rule: &'static Rule) -> Vec<Assessed> {
    let Some(version) = django(folder) else {
        return Vec::new();
    };
    let (unit, unit_at) = match folder.django_settings.as_slice() {
        [] => ("the Django app".to_owned(), folder.python_deps[0].0.clone()),
        [(one, _)] => (format!("the Django settings in {one}"), one.clone()),
        [(first, _), ..] => (
            format!(
                "the Django settings in {} files",
                folder.django_settings.len()
            ),
            first.clone(),
        ),
    };
    let place = |file: &str, line: usize| Place {
        file: file.to_owned(),
        line,
        layer: Layer::DjangoSettings,
    };
    let found = assignments(folder, rule.setting);
    let outcome = if folder.django_settings.is_empty() {
        unknown("no Django settings module (`settings.py`, or a `settings` folder) was found")
    } else if let Some((file, line, value)) = found.iter().find(|(_, _, v)| v == "False") {
        Outcome::SetUnsafe {
            value: value.clone(),
            at: place(file, *line),
        }
    } else if let Some((file, line, value)) = found.iter().find(|(_, _, v)| v != "True") {
        unknown(format!(
            "it is set in {file} line {line} to `{value}`, whose value is decided when the app starts"
        ))
    } else if let Some((file, line, value)) = found.first() {
        Outcome::SetSafe {
            value: value.clone(),
            at: place(file, *line),
        }
    } else {
        let id = format!("django.{}", rule.setting);
        match version.and_then(|v| table.holding(&id, v)) {
            Some(default) if !default.safe => Outcome::MissingDefaultUnsafe {
                defaults: vec![default.clone()],
            },
            Some(default) => Outcome::MissingDefaultSafe {
                defaults: vec![default.clone()],
            },
            None => unknown(
                "the dependency files do not pin Django to releases the table of defaults has read (4.2 to 5.2), \
                 so the default that applies is not known",
            ),
        }
    };
    vec![Assessed {
        rule_id: rule.id,
        setting: rule.setting,
        unit,
        unit_at: place(&unit_at, 1),
        outcome,
        reported_by: None,
    }]
}

// ---- Root -------------------------------------------------------------------------------------------------------

fn container_root(folder: &Folder, table: &Table) -> Vec<Assessed> {
    targets(folder)
        .iter()
        .map(|target| {
            assessed(
                &CONTAINER_ROOT,
                target,
                root_of(folder, table, target),
                None,
            )
        })
        .collect()
}

fn root_of(folder: &Folder, table: &Table, target: &Target) -> Outcome {
    if let Target::Unit(u) = target {
        if u.layer == Layer::Kubernetes {
            return kubernetes_user(u);
        }
        if let Some((user, line)) = &u.user {
            return user_named(
                user,
                Place {
                    file: u.file.clone(),
                    line: *line,
                    layer: u.layer,
                },
            );
        }
    }
    match start(folder, target) {
        Start::Built(d, eff) => match &eff.user {
            Some((user, line)) => user_named(
                user,
                Place {
                    file: d.file.clone(),
                    line: *line,
                    layer: Layer::Dockerfile,
                },
            ),
            None => missing_user(table, &Image::parse(&eff.base)),
        },
        Start::BuildUnread(file) => {
            unknown(format!("it is built from {file}, which cv could not read"))
        }
        Start::Image(image) => missing_user(table, &image),
        Start::Nothing => unknown("it names no image and no build"),
    }
}

/// `root`, `0`, or either with a group, is root; a variable is unknown; any other name or number is not root.
fn user_named(user: &str, at: Place) -> Outcome {
    let name = user.split(':').next().unwrap_or(user).trim();
    if Image::names_variable(name) {
        return unknown(format!(
            "the user is set in {} line {} from a variable (`{user}`)",
            at.file, at.line
        ));
    }
    if name == "root" || name == "0" {
        Outcome::SetUnsafe {
            value: user.to_owned(),
            at,
        }
    } else {
        Outcome::SetSafe {
            value: user.to_owned(),
            at,
        }
    }
}

/// Kubernetes: `runAsUser` decides; else `runAsNonRoot: true`, which refuses to start a container whose user is root.
/// With neither, the container runs as its image's user, which cv does not read, and the table holds no Kubernetes
/// default (its reference could not be read for one on 20 November 2026).
fn kubernetes_user(u: &Unit) -> Outcome {
    let at = |line: usize| Place {
        file: u.file.clone(),
        line,
        layer: Layer::Kubernetes,
    };
    if let Some((user, line)) = &u.user {
        return match user.parse::<u64>() {
            Ok(0) => Outcome::SetUnsafe {
                value: format!("runAsUser: {user}"),
                at: at(*line),
            },
            Ok(_) => Outcome::SetSafe {
                value: format!("runAsUser: {user}"),
                at: at(*line),
            },
            Err(_) => unknown(format!(
                "runAsUser is `{user}`, which is not a number cv can read"
            )),
        };
    }
    match &u.run_as_non_root {
        Some((value, line)) if value == "true" => Outcome::SetSafe {
            value: "runAsNonRoot: true".to_owned(),
            at: at(*line),
        },
        _ => unknown(
            "neither runAsUser nor runAsNonRoot: true is set, so the container runs as its image's own user, which \
             cv does not read",
        ),
    }
}

fn missing_user(table: &Table, image: &Image) -> Outcome {
    if !image.is_official("node") {
        return unknown(format!(
            "the image `{}` may set its own USER; cv reads files, not images",
            image.repository
        ));
    }
    let Some(major) = image.major() else {
        return unknown(format!(
            "the node image's tag (`{}`) does not say which Node.js version it is, and the table's defaults are \
             for the image's versions 22 and 24",
            image.tag.as_deref().unwrap_or("latest")
        ));
    };
    match (
        table.holding("docker-node.user", Version::Major(major)),
        table.holding("docker.user", Version::Any),
    ) {
        (Some(image_default), Some(docker_default)) => Outcome::MissingDefaultUnsafe {
            defaults: vec![image_default.clone(), docker_default.clone()],
        },
        _ => unknown(format!(
            "the table of defaults has not read the node image's version {major}"
        )),
    }
}
