//! The outbound rules a folder's infrastructure files set, for V13.2.5's hand check (backlog 0496; ADR-022, Later).
//!
//! V13.2.5 asks that the server reach only the places it was allowed to. That is set where the app is hosted, which no
//! file in the folder can show, so the requirement is a hand check. Where the folder holds the infrastructure too, the
//! rules written there are where to look: a Kubernetes `NetworkPolicy` that limits where pods may connect, a compose
//! network marked `internal`, and the outbound rules of a cloud firewall in Terraform.
//!
//! # It finds, and does not judge
//!
//! Like a locator (`crate::locate`), this makes no finding and credits nothing, and nothing that credits or fails
//! V13.2.5 reads it. A file in the folder is not known to be what runs, and a rule written here may be overridden where
//! the app is hosted; so the list is where to look, never evidence, and the hand check still asks about the host.
//!
//! # What it reads
//!
//! - Kubernetes YAML: a `kind: NetworkPolicy` whose `policyTypes` holds `Egress` or whose `spec` has `egress`, read
//!   with the YAML reader the workflow checks use (`crate::yaml`). A file it cannot read is left alone.
//! - Compose files (`docker-compose*.yml`, `compose*.yml`, and their `.yaml`): a network marked `internal: true`, with
//!   the services on it.
//! - Terraform (`.tf`), read line by line, since no grammar for it is in the build: `egress` blocks in an
//!   `aws_security_group`, an `aws_security_group_rule` of type `egress`, an `aws_vpc_security_group_egress_rule`, a
//!   `google_compute_firewall` with `direction = "EGRESS"`, and an `azurerm_network_security_rule` with
//!   `direction = "Outbound"`. Lines that are comments are skipped.
//!
//! Each rule says whether it lets the app reach everywhere (`0.0.0.0/0`, `::/0`, `*`, or a Kubernetes egress rule with
//! no `to`), since a rule that allows everything limits nothing; a rule that refuses (a Google Cloud `deny`, an Azure
//! `access = "Deny"`) never does, whatever it names. Test code is left out.

use crate::yaml::{Value, parse_documents};

/// The most rules a question shows; the rest are counted.
pub const MOST_RULES: usize = 30;

/// What the list is, and what it is not, shown above it.
pub const RULES_SAY: &str = "The outbound rules StackVet found in this folder's infrastructure files: where they are, \
     and whether one lets the app reach everywhere. These are where to look, not evidence: a file in the folder is not \
     known to be what runs, and the rules that count are the ones set where the app is hosted, so the question above \
     is still yours to answer. It reads Kubernetes NetworkPolicies, compose networks marked internal, and the outbound \
     rules of AWS security groups, Google Cloud firewalls, and Azure network security rules in Terraform.";

/// One outbound rule, as a file writes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboundRule {
    pub file: String,
    pub line: usize,
    /// What it is, in plain words.
    pub what: String,
    /// Whether it lets the app reach everywhere.
    pub open_to_all: bool,
}

/// Every outbound rule found in a folder, in file and line order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutboundRules {
    pub rules: Vec<OutboundRule>,
    /// Rules past `MOST_RULES`, counted and not kept.
    pub more: usize,
}

/// The outbound rules in the app's own files, not counting test code, the first `MOST_RULES` kept.
pub fn read(listing: &sv_scan::files::Listing) -> OutboundRules {
    let mut rules = Vec::new();
    for entry in listing.app_files() {
        if listing.is_test_code(&entry.relative) {
            continue;
        }
        let extension = entry.extension.as_deref().unwrap_or_default();
        if !matches!(extension, "yml" | "yaml" | "tf") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&entry.path) else {
            continue;
        };
        if extension == "tf" {
            rules.extend(terraform(&entry.relative, &text));
        } else {
            rules.extend(yaml(&entry.relative, &text));
        }
    }
    rules.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    let more = rules.len().saturating_sub(MOST_RULES);
    rules.truncate(MOST_RULES);
    OutboundRules { rules, more }
}

/// Whether a file's name says it is a compose file.
fn is_compose(file: &str) -> bool {
    let name = file.rsplit('/').next().unwrap_or(file).to_ascii_lowercase();
    name.starts_with("docker-compose")
        || name.starts_with("compose.")
        || name.starts_with("compose-")
}

/// The NetworkPolicies and internal compose networks a YAML file holds.
pub fn yaml(file: &str, text: &str) -> Vec<OutboundRule> {
    let Ok(documents) = parse_documents(text) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for doc in &documents {
        if doc.get("kind").and_then(Value::text) == Some("NetworkPolicy") {
            out.extend(network_policy(file, doc));
        }
        if is_compose(file) {
            out.extend(compose_networks(file, doc));
        }
    }
    out
}

fn network_policy(file: &str, doc: &Value) -> Option<OutboundRule> {
    let spec = doc.get("spec")?;
    let types: Vec<&str> = match spec.get("policyTypes") {
        Some(Value::Seq(items)) => items.iter().filter_map(Value::text).collect(),
        _ => Vec::new(),
    };
    let egress = spec.get("egress");
    if egress.is_none() && !types.contains(&"Egress") {
        return None;
    }
    let name = doc
        .get("metadata")
        .and_then(|m| m.get("name"))
        .and_then(Value::text)
        .unwrap_or("with no name");
    let line = doc.get("kind").and_then(Value::line).unwrap_or(1);
    let rules: &[Value] = match egress {
        Some(Value::Seq(items)) => items,
        _ => &[],
    };
    // An egress rule with no `to` allows every destination; so does a block of everywhere.
    let open_to_all = rules.iter().any(|rule| {
        let to = rule.get("to");
        to.is_none()
            || matches!(to, Some(Value::Null))
            || matches!(to, Some(Value::Seq(t)) if t.is_empty())
            || {
                let mut scalars = Vec::new();
                rule.scalars(false, &mut scalars);
                scalars.iter().any(|s| everywhere(s))
            }
    });
    let what = if rules.is_empty() {
        format!("a Kubernetes NetworkPolicy, {name}, that lets the pods it selects connect nowhere")
    } else {
        format!(
            "a Kubernetes NetworkPolicy, {name}, that lists where the pods it selects may connect ({} rule{})",
            rules.len(),
            if rules.len() == 1 { "" } else { "s" }
        )
    };
    Some(OutboundRule {
        file: file.to_owned(),
        line,
        what,
        open_to_all,
    })
}

fn compose_networks(file: &str, doc: &Value) -> Vec<OutboundRule> {
    let Some(Value::Map(networks)) = doc.get("networks") else {
        return Vec::new();
    };
    let services: &[(String, Value)] = match doc.get("services") {
        Some(Value::Map(s)) => s,
        _ => &[],
    };
    networks
        .iter()
        .filter_map(|(name, network)| {
            let internal = network.get("internal")?;
            if !matches!(internal.text(), Some("true" | "True" | "yes")) {
                return None;
            }
            let on: Vec<&str> = services
                .iter()
                .filter(|(_, service)| match service.get("networks") {
                    Some(Value::Seq(items)) => items.iter().any(|n| n.text() == Some(name.as_str())),
                    Some(Value::Map(pairs)) => pairs.iter().any(|(n, _)| n == name),
                    _ => false,
                })
                .map(|(service, _)| service.as_str())
                .collect();
            let who = match on.as_slice() {
                [] => "no service is on it".to_owned(),
                [one] => format!("{one} is on it"),
                many => format!("{} are on it", many.join(", ")),
            };
            Some(OutboundRule {
                file: file.to_owned(),
                line: internal.line().unwrap_or(1),
                what: format!(
                    "a compose network, {name}, marked internal, so what is only on it cannot reach outside; {who}"
                ),
                open_to_all: false,
            })
        })
        .collect()
}

/// Whether a destination stands for everywhere.
fn everywhere(text: &str) -> bool {
    let t = text.trim().trim_matches('"');
    t == "0.0.0.0/0" || t == "::/0" || t == "*" || t == "Internet"
}

/// The resource types whose outbound rules this reads, and how each says it is outbound.
enum Kind {
    /// `aws_security_group`: each nested `egress { … }` block is a rule.
    GroupBlocks,
    /// The whole resource is a rule when one of its lines says so (`type = "egress"`), or always (`None`).
    Whole(Option<(&'static str, &'static str)>, &'static str),
}

fn kind_of(resource: &str) -> Option<Kind> {
    Some(match resource {
        "aws_security_group" => Kind::GroupBlocks,
        "aws_security_group_rule" => {
            Kind::Whole(Some(("type", "egress")), "an AWS security group rule")
        }
        "aws_vpc_security_group_egress_rule" => {
            Kind::Whole(None, "an AWS security group egress rule")
        }
        "google_compute_firewall" => Kind::Whole(
            Some(("direction", "EGRESS")),
            "a Google Cloud firewall rule",
        ),
        "azurerm_network_security_rule" => Kind::Whole(
            Some(("direction", "Outbound")),
            "an Azure network security rule",
        ),
        _ => return None,
    })
}

/// `{` minus `}` on a line, outside strings.
fn depth_change(line: &str) -> i64 {
    let mut in_string = false;
    let mut escaped = false;
    let mut change = 0;
    for c in line.chars() {
        match c {
            _ if escaped => escaped = false,
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' if !in_string => change += 1,
            '}' if !in_string => change -= 1,
            _ => {}
        }
    }
    change
}

/// The value a line sets `key` to, unquoted: `type = "egress"` gives `egress`.
fn setting<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line
        .trim()
        .strip_prefix(key)?
        .trim_start()
        .strip_prefix('=')?;
    Some(rest.trim().trim_matches('"'))
}

/// The outbound rules a Terraform file's resources set, read line by line.
pub fn terraform(file: &str, text: &str) -> Vec<OutboundRule> {
    let mut out = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    let mut in_comment = false;
    while i < lines.len() {
        let line = lines[i].trim();
        if in_comment {
            in_comment = !line.contains("*/");
            i += 1;
            continue;
        }
        if line.starts_with("/*") {
            in_comment = !line.contains("*/");
            i += 1;
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        let resource = match words.as_slice() {
            ["resource", kind, name, ..] => Some((kind.trim_matches('"'), name.trim_matches('"'))),
            _ => None,
        };
        let Some((resource, name)) = resource else {
            i += 1;
            continue;
        };
        let Some(kind) = kind_of(resource) else {
            i += 1;
            continue;
        };
        // The resource's body: from its opening line until its braces close.
        let start = i;
        let mut depth = depth_change(lines[i]);
        let mut end = i;
        while depth > 0 && end + 1 < lines.len() {
            end += 1;
            let l = lines[end].trim();
            if !(l.starts_with('#') || l.starts_with("//")) {
                depth += depth_change(lines[end]);
            }
        }
        let body: Vec<(usize, &str)> = (start + 1..=end)
            .map(|n| (n, lines[n].trim()))
            .filter(|(_, l)| !(l.starts_with('#') || l.starts_with("//")))
            .collect();
        match kind {
            Kind::GroupBlocks => {
                let mut k = 0;
                while k < body.len() {
                    let (n, l) = body[k];
                    if l.starts_with("egress") && l.contains('{') {
                        let mut d = depth_change(l);
                        let mut open = false;
                        let mut m = k;
                        while d > 0 && m + 1 < body.len() {
                            m += 1;
                            d += depth_change(body[m].1);
                            open |= body[m].1.split(['"', ',', '[', ']', ' ']).any(everywhere);
                        }
                        out.push(OutboundRule {
                            file: file.to_owned(),
                            line: n + 1,
                            what: format!(
                                "an egress rule of the AWS security group {resource}.{name}"
                            ),
                            open_to_all: open,
                        });
                        k = m + 1;
                    } else {
                        k += 1;
                    }
                }
            }
            Kind::Whole(says, what) => {
                let outbound = match says {
                    None => true,
                    Some((key, wanted)) => body.iter().any(|(_, l)| {
                        setting(l, key).is_some_and(|v| v.eq_ignore_ascii_case(wanted))
                    }),
                };
                if outbound {
                    // A Google Cloud `deny { … }` block, or an Azure `access = "Deny"`, refuses what it names: one to
                    // everywhere shuts the app in, the opposite of letting it reach everywhere.
                    let refuses = body.iter().any(|(_, l)| {
                        (l.starts_with("deny") && l.contains('{'))
                            || setting(l, "access").is_some_and(|v| v.eq_ignore_ascii_case("deny"))
                    });
                    let open = !refuses
                        && body
                            .iter()
                            .any(|(_, l)| l.split(['"', ',', '[', ']', ' ']).any(everywhere));
                    let how = if refuses { "refusing" } else { "for" };
                    out.push(OutboundRule {
                        file: file.to_owned(),
                        line: start + 1,
                        what: format!("{what}, {resource}.{name}, {how} outgoing connections"),
                        open_to_all: open,
                    });
                }
            }
        }
        i = end + 1;
    }
    out
}

#[cfg(test)]
#[path = "outbound_rules_tests.rs"]
mod tests;
