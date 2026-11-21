//! A deployment unit: one container as a compose file, a Kubernetes manifest, or a Terraform plan says to run it.
//! The three readers below each turn their layer into these, so a rule asks one shape the same questions.

use serde::Serialize;

/// The layer a value was read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layer {
    Dockerfile,
    Compose,
    Kubernetes,
    TerraformPlan,
    DjangoSettings,
}

impl Layer {
    pub fn name(self) -> &'static str {
        match self {
            Layer::Dockerfile => "Dockerfile",
            Layer::Compose => "compose file",
            Layer::Kubernetes => "Kubernetes manifest",
            Layer::TerraformPlan => "Terraform plan",
            Layer::DjangoSettings => "Django settings",
        }
    }
}

/// One environment variable a unit sets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvVar {
    pub name: String,
    /// `None` when the value comes from somewhere the file does not hold: a Kubernetes `valueFrom`, an ECS secret, or
    /// a compose entry that names the variable and passes the shell's value through.
    pub value: Option<String>,
    pub line: usize,
}

/// A compose service's `build`: the Dockerfile it is built from, relative to the folder `cv` was given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Build {
    pub dockerfile: String,
    pub line: usize,
}

/// One container to be run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub layer: Layer,
    pub file: String,
    pub line: usize,
    /// What the file calls it: a compose service, a Kubernetes container (with its workload), an ECS container.
    pub name: String,
    pub image: Option<String>,
    pub build: Option<Build>,
    pub env: Vec<EnvVar>,
    /// Variables may also come from a place the file only points to (`env_file`, `envFrom`, `environmentFiles`): the
    /// first such place, as the file names it.
    pub env_elsewhere: Option<String>,
    /// The user to run as, as the file writes it (`user:`, Kubernetes `runAsUser`), with its line.
    pub user: Option<(String, usize)>,
    /// Kubernetes `runAsNonRoot`, with its line.
    pub run_as_non_root: Option<(String, usize)>,
}

impl Unit {
    pub fn new(layer: Layer, file: &str, line: usize, name: String) -> Unit {
        Unit {
            layer,
            file: file.to_owned(),
            line,
            name,
            image: None,
            build: None,
            env: Vec::new(),
            env_elsewhere: None,
            user: None,
            run_as_non_root: None,
        }
    }

    pub fn env(&self, name: &str) -> Option<&EnvVar> {
        self.env.iter().rev().find(|v| v.name == name)
    }

    /// How a report names it: "compose service `web` in compose.yaml".
    pub fn describe(&self) -> String {
        format!("{} in {}", self.name, self.file)
    }
}

/// An image name taken apart as far as the rules need: whether it is the official `node` image, and its major version
/// when the tag starts with one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub repository: String,
    pub tag: Option<String>,
}

impl Image {
    pub fn parse(name: &str) -> Image {
        let without_digest = name.split('@').next().unwrap_or(name);
        // A tag follows the last colon, unless that colon is a registry's port (a slash after it).
        let (repository, tag) = match without_digest.rsplit_once(':') {
            Some((repo, tag)) if !tag.contains('/') => (repo, Some(tag.to_owned())),
            _ => (without_digest, None),
        };
        let repository = repository
            .strip_prefix("docker.io/")
            .or_else(|| repository.strip_prefix("index.docker.io/"))
            .unwrap_or(repository);
        let repository = repository.strip_prefix("library/").unwrap_or(repository);
        Image {
            repository: repository.to_ascii_lowercase(),
            tag,
        }
    }

    /// The Docker Hub official image of that name.
    pub fn is_official(&self, name: &str) -> bool {
        self.repository == name
    }

    /// The major version the tag begins with (`22` of `22.11-alpine`); `None` for `latest`, `lts`, a codename, or no
    /// tag, whose version moves.
    pub fn major(&self) -> Option<u32> {
        let tag = self.tag.as_deref()?;
        let digits: String = tag.chars().take_while(char::is_ascii_digit).collect();
        digits.parse().ok()
    }

    pub fn names_variable(name: &str) -> bool {
        name.contains('$')
    }
}

/// A path relative to the folder, from a path written in a file in `dir` (itself relative to the folder). `None` when
/// it leaves the folder.
pub fn join(dir: &str, written: &str) -> Option<String> {
    let mut parts: Vec<&str> = dir.split('/').filter(|p| !p.is_empty()).collect();
    for part in written.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

/// The folder a relative path is in, `""` for the top.
pub fn dir_of(relative: &str) -> &str {
    relative.rsplit_once('/').map_or("", |(dir, _)| dir)
}
