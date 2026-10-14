//! The bill of materials written as CycloneDX. Moved out of `sbom.rs` unchanged on 9 October 2026
//! (architecture assessment, item 11).

use super::*;

#[derive(Serialize)]
pub(super) struct CycloneComponent {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,
    #[serde(rename = "bom-ref")]
    pub(super) bom_ref: String,
    pub(super) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) purl: Option<String>,
    pub(super) properties: Vec<Property>,
    /// The parts of this one listed under it: a language's own modules the app imports (backlog 0402).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) components: Vec<CycloneComponent>,
}

/// What the bill of materials describes: the app itself (CycloneDX `metadata.component`).
#[derive(Serialize)]
pub(super) struct Subject {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,
    #[serde(rename = "bom-ref")]
    pub(super) bom_ref: String,
    pub(super) name: String,
}

#[derive(Serialize)]
pub(super) struct Property {
    pub(super) name: String,
    pub(super) value: String,
}

#[derive(Serialize)]
pub(super) struct Metadata {
    pub(super) tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) component: Option<Subject>,
    pub(super) properties: Vec<Property>,
}

#[derive(Serialize)]
pub(super) struct Tool {
    pub(super) vendor: &'static str,
    pub(super) name: &'static str,
}

#[derive(Serialize)]
pub struct CycloneDx {
    #[serde(rename = "bomFormat")]
    pub(super) bom_format: &'static str,
    #[serde(rename = "specVersion")]
    pub(super) spec_version: &'static str,
    pub(super) version: u32,
    pub(super) metadata: Metadata,
    pub(super) components: Vec<CycloneComponent>,
}

/// Renders the bill of materials as CycloneDX 1.5 JSON.
///
/// The completeness caveats go in `metadata.properties` rather than only in the terminal, because the
/// document is the thing that gets sent to somebody else, and a caveat that stays behind is not a caveat.
pub fn to_cyclonedx(sbom: &Sbom) -> CycloneDx {
    to_cyclonedx_with(sbom, &RunsOn::default())
}

/// `to_cyclonedx`, with what the app runs on beside its packages (backlog 0402): the app as the document's subject,
/// the container image `stackvet.toml` starts it in, and the language with the modules of its own library the code
/// imports. Each says where `sv` got it. None is a package, so `securevibe:complete` is about the packages alone.
pub fn to_cyclonedx_with(sbom: &Sbom, runs_on: &RunsOn) -> CycloneDx {
    let mut properties = vec![Property {
        name: "securevibe:complete".into(),
        value: sbom.is_complete().to_string(),
    }];
    if sbom.declared_count() > 0 {
        properties.push(Property {
            name: "securevibe:declared-versions".into(),
            value: format!(
                "{} component(s) carry the version that was asked for, not the version installed",
                sbom.declared_count()
            ),
        });
    }
    for (eco, why) in &sbom.unread {
        properties.push(Property {
            name: format!("securevibe:unread:{eco}"),
            value: why.clone(),
        });
    }
    for passed in &sbom.passed_over {
        properties.push(Property {
            name: format!("securevibe:lockfile-passed-over:{}", passed.project),
            value: passed.explain(),
        });
    }
    for disagreement in &sbom.disagreements {
        if disagreement.differs() {
            properties.push(Property {
                name: format!("securevibe:manifest-disagrees:{}", disagreement.project),
                value: disagreement.explain(),
            });
        }
        if disagreement.comparison.not_all_compared() {
            properties.push(Property {
                name: format!("securevibe:manifest-not-compared:{}", disagreement.project),
                value: disagreement.explain_not_compared(),
            });
        }
    }

    CycloneDx {
        bom_format: "CycloneDX",
        spec_version: "1.5",
        version: 1,
        metadata: Metadata {
            tools: vec![Tool {
                vendor: "StackVet",
                name: "sv",
            }],
            component: runs_on.app.as_ref().map(|name| Subject {
                kind: "application",
                bom_ref: format!("app:{name}"),
                name: name.clone(),
            }),
            properties,
        },
        components: sbom
            .components
            .iter()
            .map(|c| CycloneComponent {
                kind: "library",
                bom_ref: c.purl(),
                name: c.name.clone(),
                version: Some(c.version.clone()),
                purl: Some(c.purl()),
                properties: vec![Property {
                    name: "securevibe:version-source".into(),
                    value: match c.source {
                        VersionSource::Locked => "locked: this is what is installed".into(),
                        VersionSource::Declared => {
                            "declared: this is what was asked for, and may be a range".to_string()
                        }
                    },
                }],
                components: Vec::new(),
            })
            .chain(image_component(runs_on))
            .chain(language_component(runs_on))
            .collect(),
    }
}

fn property(name: &str, value: impl Into<String>) -> Property {
    Property {
        name: name.to_owned(),
        value: value.into(),
    }
}

/// The container image `stackvet.toml` starts the app in, by its name and tag, with its digest when `--run` read it.
fn image_component(runs_on: &RunsOn) -> Option<CycloneComponent> {
    let image = runs_on.image.as_ref()?;
    let (name, tag) = match image.split('@').next().unwrap_or(image).rsplit_once(':') {
        Some((name, tag)) if !tag.contains('/') => (name.to_owned(), Some(tag.to_owned())),
        _ => (image.split('@').next().unwrap_or(image).to_owned(), None),
    };
    let mut properties = vec![property(
        "securevibe:source",
        "stackvet.toml, `[stack.run]` image: the image sv starts the app in",
    )];
    match &runs_on.image_digest {
        Some(digest) => properties.push(property(
            "securevibe:digest",
            format!(
                "{digest}, read from this computer's Docker when sv report --run started the app"
            ),
        )),
        None => properties.push(property(
            "securevibe:digest",
            "not read: the app was not started here, so which build of this tag it is is not known",
        )),
    }
    Some(CycloneComponent {
        kind: "container",
        bom_ref: format!("image:{image}"),
        name,
        version: tag,
        purl: None,
        properties,
        components: Vec::new(),
    })
}

/// The language the app's code is written in, with each module of its own library the code imports listed under it.
fn language_component(runs_on: &RunsOn) -> Option<CycloneComponent> {
    let language = runs_on.language.as_ref()?;
    let modules = language
        .modules
        .iter()
        .map(|(module, files)| CycloneComponent {
            kind: "library",
            bom_ref: format!("{}/{module}", language.name),
            name: module.clone(),
            version: None,
            purl: None,
            properties: vec![
                property(
                    "securevibe:part-of",
                    format!(
                        "{}'s own library, not a separate package: a flaw in it is fixed in a newer {}",
                        language.name, language.name
                    ),
                ),
                property(
                    "securevibe:imported-by",
                    files.iter().cloned().collect::<Vec<_>>().join(", "),
                ),
            ],
            components: Vec::new(),
        })
        .collect();
    Some(CycloneComponent {
        kind: "platform",
        bom_ref: language.name.to_owned(),
        name: language.name.to_owned(),
        version: language.version.clone(),
        purl: None,
        properties: vec![
            property("securevibe:version-source", language.version_from.clone()),
            property(
                "securevibe:modules",
                match &language.modules_not_read {
                    Some(why) => why.clone(),
                    None => format!(
                        "the modules of {}'s own library the app's code imports, read from its import lines: {}",
                        language.name,
                        if language.modules.is_empty() {
                            "none".to_owned()
                        } else {
                            format!("{}, listed under this component", language.modules.len())
                        }
                    ),
                },
            ),
        ],
        components: modules,
    })
}

/// A finding when the bill of materials cannot be trusted as a complete list.
/// The requirement a complete bill of materials is evidence about: an inventory catalog of every
/// third-party library in use. Named once so the check and `sv coverage` cannot disagree.
pub const INVENTORY_REQUIREMENT: &str = "V15.1.2";

/// What the bill of materials may claim to be, when it is complete enough to claim anything.
///
/// The mirror of `incompleteness_finding`: exactly one of the two speaks, and which one is decided
/// by `is_complete`, so a document cannot be reported as both incomplete and a good inventory. An
/// empty list is not a complete inventory either — an app with no dependencies `sv` could find is
/// far more often an app whose manifests were not read than an app with no dependencies.
/// "1 package", "2 packages".
pub(crate) fn count(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}
