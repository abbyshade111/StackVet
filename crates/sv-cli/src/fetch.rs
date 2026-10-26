//! A public project, fetched from its GitHub or GitLab address for `sv report` to check (backlog 0670, ADR-154).
//!
//! The fence's fourth exception, asked for by name: the person types the project's address, and `sv` downloads that
//! host's own archive of one commit through `curl`, as `sv probe` uses it. HTTPS only, to `github.com`, `gitlab.com`, and
//! the download host GitHub sends to, each redirect checked here before it is followed, and limits on the redirects, on
//! the size downloaded, and on the time. Nothing is sent but the request for the archive.
//!
//! Nothing in the download runs. An archive holds files, not Git: no hooks, no filters, no submodules. `sv` unpacks it
//! itself, into a folder it makes and names after the commit, writing every file as plain data with no permission to
//! run; an entry that is a link, or whose name would land outside that folder, is left out and said, and the unpacking
//! stops at a limit on the files and the bytes.

use anyhow::{Context, Result, bail};
use std::path::{Component, Path, PathBuf};

/// The hosts a project's address may name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host {
    GitHub,
    GitLab,
}

impl Host {
    /// The host's name, as an address writes it.
    pub fn name(self) -> &'static str {
        match self {
            Host::GitHub => "github.com",
            Host::GitLab => "gitlab.com",
        }
    }
}

/// A project's address, read: the host, the project's path on it, and the branch, tag, or commit, when the address
/// names one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address {
    pub host: Host,
    /// `owner/repo` on GitHub; `group/subgroup/project` on GitLab.
    pub path: String,
    /// `None` for the project's default branch.
    pub reference: Option<String>,
}

/// The hosts a download may be sent to: the two a project's address may name, and the one GitHub sends its archives
/// from. A redirect to any other is refused before it is followed.
pub const DOWNLOAD_HOSTS: &[&str] = &["github.com", "codeload.github.com", "gitlab.com"];

/// The most redirects one download follows. GitHub sends an archive's address to its download host once.
pub const MOST_REDIRECTS: usize = 3;

/// The most one download may be, in bytes: far more than the source of almost any project.
pub const MOST_DOWNLOAD_BYTES: u64 = 500 * 1024 * 1024;

/// The most time one request may take, in seconds.
pub const MOST_SECONDS: u32 = 600;

/// How much an archive may unpack to.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub files: usize,
    pub bytes: u64,
}

/// The limits a fetch unpacks to: an archive past them is a bomb or not a project's source.
pub const LIMITS: Limits = Limits {
    files: 50_000,
    bytes: 2 * 1024 * 1024 * 1024,
};

/// The folder, under the one `sv report` was started in, that fetched projects are kept in.
pub const FETCHED_FOLDER: &str = "stackvet-fetched";

/// Whether `sv report` was given an address rather than a folder: anything with `://` in it. A folder's name never has
/// it, so an address that is not one `sv` can fetch is refused rather than read as a folder that does not exist.
pub fn looks_like_address(arg: &str) -> bool {
    arg.contains("://")
}

/// What may name a project, a group, or an owner: letters, digits, `.`, `_`, and `-`, never `.` or `..` alone.
fn plain_name(part: &str) -> bool {
    !part.is_empty()
        && part != "."
        && part != ".."
        && part
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

/// What may name a branch, a tag, or a commit: plain names separated by `/`, not starting with `-`, and with no `..`.
fn plain_reference(reference: &str) -> bool {
    !reference.starts_with('-')
        && !reference.contains("..")
        && reference.split('/').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '+'))
        })
}

/// Reads a project's address: `https://github.com/OWNER/REPO` or `https://gitlab.com/GROUP/.../PROJECT`, each with
/// `/tree/BRANCH` (on GitLab `/-/tree/BRANCH`) or `/commit/SHA` where it names one. Anything else is refused, with
/// what was expected, before anything is sent.
pub fn read_address(text: &str) -> Result<Address> {
    const EXPECTED: &str = "a project's address on GitHub or GitLab, such as https://github.com/OWNER/PROJECT or \
                            https://gitlab.com/GROUP/PROJECT";
    let text = text.trim();
    let Some(rest) = text.strip_prefix("https://") else {
        if text.starts_with("http://") {
            bail!("{text}: only an https:// address is fetched; write it as {EXPECTED}");
        }
        bail!("{text} is not {EXPECTED}");
    };
    // The page's own part (`?tab=readme`, `#readme`) says nothing about which code.
    let rest = rest.split(['?', '#']).next().unwrap_or_default();
    let rest = rest.trim_end_matches('/');
    let (host, path) = rest.split_once('/').unwrap_or((rest, ""));
    let host = match host.to_ascii_lowercase().as_str() {
        "github.com" | "www.github.com" => Host::GitHub,
        "gitlab.com" | "www.gitlab.com" => Host::GitLab,
        _ => bail!(
            "{text}: sv fetches projects from GitHub and GitLab only (ADR-154); give {EXPECTED}, or check out the \
             project yourself and give sv its folder"
        ),
    };
    let parts: Vec<&str> = path.split('/').collect();
    let (project, after): (&[&str], &[&str]) = match host {
        Host::GitHub => {
            if parts.len() < 2 {
                bail!("{text} names no project: give {EXPECTED}");
            }
            parts.split_at(2)
        }
        // GitLab's own pages under a project start with `-`.
        Host::GitLab => match parts.iter().position(|p| *p == "-") {
            Some(at) => (&parts[..at], &parts[at + 1..]),
            None => (&parts[..], &[][..]),
        },
    };
    if project.len() < 2 {
        bail!("{text} names no project: give {EXPECTED}");
    }
    let mut project: Vec<&str> = project.to_vec();
    if let Some(last) = project.last_mut() {
        *last = last.strip_suffix(".git").unwrap_or(last);
    }
    if !project.iter().all(|p| plain_name(p)) {
        bail!("{text}: its project's name is not one sv reads; give {EXPECTED}");
    }
    let reference = match after {
        [] => None,
        ["tree", reference @ ..] if !reference.is_empty() => Some(reference.join("/")),
        ["commit", sha] => Some((*sha).to_owned()),
        _ => bail!(
            "{text} is a page within a project, not the project: give the project's own address, or its \
             address with /tree/BRANCH"
        ),
    };
    if let Some(reference) = &reference
        && !plain_reference(reference)
    {
        bail!("{text}: {reference} is not a branch, tag, or commit name sv reads");
    }
    Ok(Address {
        host,
        path: project.join("/"),
        reference,
    })
}

impl Address {
    /// The project's address, as `sv` writes it back.
    pub fn shown(&self) -> String {
        let base = format!("https://{}/{}", self.host.name(), self.path);
        match (&self.reference, self.host) {
            (None, _) => base,
            (Some(r), Host::GitHub) => format!("{base}/tree/{r}"),
            (Some(r), Host::GitLab) => format!("{base}/-/tree/{r}"),
        }
    }

    /// Where the host serves its archive of the commit: GitHub's `archive/` address (which sends to its download host),
    /// and GitLab's archive in its public API.
    pub fn archive(&self) -> String {
        match self.host {
            Host::GitHub => format!(
                "https://github.com/{}/archive/{}.zip",
                self.path,
                self.reference.as_deref().unwrap_or("HEAD")
            ),
            Host::GitLab => {
                let mut url = format!(
                    "https://gitlab.com/api/v4/projects/{}/repository/archive.zip",
                    self.path.replace('/', "%2F")
                );
                if let Some(r) = &self.reference {
                    url.push_str("?sha=");
                    url.push_str(&r.replace('/', "%2F"));
                }
                url
            }
        }
    }
}

/// Whether a download may be sent to `url`: HTTPS, to one of `DOWNLOAD_HOSTS`, with no name and password and no port
/// of its own.
pub fn allowed(url: &str) -> Result<()> {
    let Some(rest) = url.strip_prefix("https://") else {
        bail!("{url} is not an https:// address");
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    if host.contains(['@', ':']) || !DOWNLOAD_HOSTS.contains(&host.to_ascii_lowercase().as_str()) {
        bail!(
            "{url} is not on {} (ADR-154), so it was not followed",
            DOWNLOAD_HOSTS.join(", ")
        );
    }
    Ok(())
}

/// What one request was answered with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Answer {
    /// The archive, written to the file asked for.
    Saved,
    /// Somewhere else to ask; not followed until `allowed` says so.
    Redirect(String),
}

/// One request for one address, its answer written to `to`: `curl` for a real fetch, a stand-in in a test.
pub trait Download {
    fn get(&mut self, url: &str, to: &Path) -> Result<Answer>;
}

/// The program that downloads: `curl`, found on the PATH. Only in a debug build, `SV_CURL_FOR_TEST` names the program to
/// run instead, as it does for `sv probe` (`sv_check::production`), so a test can put a stand-in in its place. A
/// released `sv` has no such switch.
fn curl_program() -> std::ffi::OsString {
    #[cfg(debug_assertions)]
    if let Some(program) = std::env::var_os("SV_CURL_FOR_TEST") {
        return program;
    }
    "curl".into()
}

/// The real download: `curl`, with the flags that make the limits true rather than intended.
pub struct Curl;

impl Curl {
    /// Every flag but the address and the file: `--disable` first, the only place curl reads it, so no `.curlrc` on
    /// this computer adds anything; globbing off; HTTPS only; no redirect followed by curl itself, since each is
    /// checked here first; and the limits.
    pub fn args(to: &Path) -> Vec<String> {
        [
            "--disable",
            "--globoff",
            "--proto",
            "=https",
            "--max-redirs",
            "0",
            "--silent",
            "--show-error",
            "--max-filesize",
            &MOST_DOWNLOAD_BYTES.to_string(),
            "--max-time",
            &MOST_SECONDS.to_string(),
            "--user-agent",
            "sv (StackVet, fetching a public project's archive)",
            "--write-out",
            "%{http_code} %{redirect_url}",
            "--output",
        ]
        .into_iter()
        .map(str::to_owned)
        .chain([to.display().to_string()])
        .collect()
    }
}

impl Download for Curl {
    fn get(&mut self, url: &str, to: &Path) -> Result<Answer> {
        let out = std::process::Command::new(curl_program())
            .args(Self::args(to))
            .arg(url)
            .stdin(std::process::Stdio::null())
            .output()
            .context("curl could not be run, and sv needs it to fetch a project")?;
        if !out.status.success() {
            let said = String::from_utf8_lossy(&out.stderr);
            let said = said.trim().trim_start_matches("curl: ");
            match out.status.code() {
                Some(63) => bail!(
                    "the archive is larger than {} MB, so it was not downloaded",
                    MOST_DOWNLOAD_BYTES / (1024 * 1024)
                ),
                Some(28) => {
                    bail!("the download took longer than {MOST_SECONDS} seconds, so it was stopped")
                }
                _ => bail!("the download did not finish: {said}"),
            }
        }
        let written = String::from_utf8_lossy(&out.stdout);
        let (code, redirect) = written
            .trim()
            .split_once(' ')
            .unwrap_or((written.trim(), ""));
        match code {
            "200" => Ok(Answer::Saved),
            "301" | "302" | "303" | "307" | "308" if !redirect.is_empty() => {
                Ok(Answer::Redirect(redirect.to_owned()))
            }
            "404" => bail!(
                "the host has no such project, branch, or commit, or the project is not public (it answered 404)"
            ),
            other => bail!("the host answered {other}, not the archive"),
        }
    }
}

/// Downloads the archive of `address` to `to`, following each redirect only once `allowed` says it may.
pub fn download(address: &Address, via: &mut dyn Download, to: &Path) -> Result<()> {
    let mut url = address.archive();
    for _ in 0..=MOST_REDIRECTS {
        allowed(&url)?;
        match via.get(&url, to)? {
            Answer::Saved => return Ok(()),
            Answer::Redirect(next) => url = next,
        }
    }
    bail!(
        "the host sent the download on more than {MOST_REDIRECTS} times, so sv stopped following it"
    )
}

/// What an archive unpacked to.
#[derive(Debug, Default)]
pub struct Unpacked {
    /// The commit, from the archive itself: `None` when it does not say.
    pub commit: Option<String>,
    /// How many files were written.
    pub files: usize,
    /// Each entry not written, and why: a link, or a name that would land outside the folder.
    pub left_out: Vec<String>,
}

/// A commit's full name: 40 hexadecimal characters (or 64, for a SHA-256 repository).
fn commit_name(text: &str) -> bool {
    matches!(text.len(), 40 | 64) && text.chars().all(|c| c.is_ascii_hexdigit())
}

/// The parts of an entry's name, when it is a plain relative path: no root, no drive, no `..`, no `\`, no `:`, and no
/// NUL. `None` for any other.
fn plain_parts(name: &str) -> Option<Vec<&str>> {
    if name.contains(['\\', ':', '\0']) {
        return None;
    }
    let mut parts = Vec::new();
    for part in Path::new(name).components() {
        match part {
            Component::Normal(part) => parts.push(part.to_str()?),
            Component::CurDir => {}
            _ => return None,
        }
    }
    (!parts.is_empty()).then_some(parts)
}

/// Unpacks the archive at `zip` into `into` (which must exist and be empty), dropping the one folder every entry is
/// in when there is one, as both hosts' archives have. Every file is written new, as plain data with no permission to
/// run, however the archive marks it.
pub fn unpack(zip: &Path, into: &Path, limits: Limits) -> Result<Unpacked> {
    use std::io::{Read, Write};
    let file = std::fs::File::open(zip).with_context(|| format!("opening {}", zip.display()))?;
    let mut archive = zip::ZipArchive::new(std::io::BufReader::new(file))
        .context("the download is not a zip archive")?;
    let comment = String::from_utf8_lossy(archive.comment()).trim().to_owned();
    // A name that is not text is kept as nothing, which `plain_parts` leaves out.
    let names: Vec<String> = (0..archive.len())
        .map(|i| {
            archive
                .name_for_index(i)
                .and_then(|n| n.ok())
                .map(|n| n.into_owned())
                .unwrap_or_default()
        })
        .collect();
    // The one folder every entry is in: `PROJECT-BRANCH/` on GitHub, `PROJECT-BRANCH-COMMIT/` on GitLab.
    let top = names
        .first()
        .and_then(|n| n.split('/').next())
        .filter(|top| {
            !top.is_empty()
                && names.iter().all(|n| {
                    n.strip_prefix(top)
                        .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
                })
        })
        .map(str::to_owned);
    let mut unpacked = Unpacked {
        commit: commit_name(&comment).then(|| comment.to_lowercase()),
        ..Unpacked::default()
    };
    if unpacked.commit.is_none() {
        unpacked.commit = top
            .as_deref()
            .and_then(|t| t.rsplit('-').next())
            .filter(|c| commit_name(c))
            .map(str::to_lowercase);
    }
    let mut bytes = 0u64;
    for (i, name) in names.iter().enumerate() {
        let shown = sv_report::visible(name);
        let Some(parts) = plain_parts(name) else {
            unpacked.left_out.push(format!(
                "{shown}: its name would place it outside the folder"
            ));
            continue;
        };
        let parts = match &top {
            Some(top) if parts.first() == Some(&top.as_str()) => &parts[1..],
            _ => &parts[..],
        };
        if parts.is_empty() {
            continue;
        }
        let mut entry = archive.by_index(i)?;
        if entry.is_symlink() {
            unpacked
                .left_out
                .push(format!("{shown}: a link, which sv does not follow or make"));
            continue;
        }
        if entry.encrypted() {
            unpacked.left_out.push(format!("{shown}: encrypted"));
            continue;
        }
        let target: PathBuf = parts
            .iter()
            .fold(into.to_path_buf(), |p, part| p.join(part));
        if entry.is_dir() {
            std::fs::create_dir_all(&target)
                .with_context(|| format!("making {}", target.display()))?;
            continue;
        }
        if unpacked.files >= limits.files {
            bail!(
                "the archive holds more than {} files, more than sv unpacks",
                limits.files
            );
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("making {}", parent.display()))?;
        }
        // New, never over a file already written: two names that are one on this computer (`A` and `a` on a Mac) keep
        // the first, and the second is said.
        let mut out = match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&target)
        {
            Ok(out) => out,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                unpacked.left_out.push(format!(
                    "{shown}: another entry has the same name on this computer"
                ));
                continue;
            }
            Err(e) => return Err(e).with_context(|| format!("writing {}", target.display())),
        };
        // Counted as it is read, not from the size the archive claims.
        let room = limits.bytes - bytes;
        let mut data = Vec::new();
        (&mut entry)
            .take(room + 1)
            .read_to_end(&mut data)
            .with_context(|| format!("reading {shown} from the archive"))?;
        if data.len() as u64 > room {
            bail!(
                "the archive unpacks to more than {} MB, more than sv unpacks",
                limits.bytes / (1024 * 1024)
            );
        }
        bytes += data.len() as u64;
        out.write_all(&data)
            .with_context(|| format!("writing {}", target.display()))?;
        unpacked.files += 1;
    }
    Ok(unpacked)
}

/// A project fetched and unpacked.
#[derive(Debug)]
pub struct Fetched {
    /// Where it was unpacked: `stackvet-fetched/HOST/PATH-COMMIT`, under the folder `sv` was started in.
    pub folder: PathBuf,
    pub address: Address,
    pub unpacked: Unpacked,
    /// The project carried no `stackvet.toml`, so `sv` wrote one with nothing answered (`NOBODY_ANSWERED`).
    pub manifest_written: bool,
}

/// The `stackvet.toml` written for a fetched project that carries none: comments only, so every question in it is
/// unanswered and read as nothing (ADR-015). The person checking somebody else's project cannot answer for it.
pub const NOBODY_ANSWERED: &str = "\
# Written by sv for a project fetched from its address (backlog 0670, ADR-154), which carried no stackvet.toml.
# Nobody has answered its questions: the person checking it did not write the project and cannot answer for it, so
# each is unanswered, and what rests on it is said not assessed.
";

/// Fetches `address` into a folder of its own under `base`'s `stackvet-fetched`, named after the commit, and never one
/// already there: a second fetch of the same commit is unpacked beside the first, which may since have been changed.
pub fn fetch(address: &Address, base: &Path, via: &mut dyn Download) -> Result<Fetched> {
    let parent = address.path.split('/').fold(
        base.join(FETCHED_FOLDER).join(address.host.name()),
        |p, part| p.join(part),
    );
    let parent = parent.parent().map(Path::to_path_buf).unwrap_or(parent);
    std::fs::create_dir_all(&parent).with_context(|| format!("making {}", parent.display()))?;
    let pid = std::process::id();
    let zip = parent.join(format!(".download-{pid}.zip"));
    let staging = parent.join(format!(".unpacking-{pid}"));
    let result = (|| {
        download(address, via, &zip)?;
        std::fs::create_dir(&staging).with_context(|| format!("making {}", staging.display()))?;
        unpack(&zip, &staging, LIMITS)
    })();
    // Both are sv's own, made just now; the project is in neither until it is named below.
    let _ = std::fs::remove_file(&zip);
    let unpacked = match result {
        Ok(unpacked) => unpacked,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&staging);
            return Err(e);
        }
    };
    let project = address.path.rsplit('/').next().unwrap_or("project");
    let short = unpacked
        .commit
        .as_deref()
        .map_or_else(|| "commit-not-said".to_owned(), |c| c[..12].to_owned());
    let mut folder = parent.join(format!("{project}-{short}"));
    let mut n = 2;
    while folder.exists() {
        folder = parent.join(format!("{project}-{short}-{n}"));
        n += 1;
    }
    let manifest_written = matches!(sv_manifest::locate(&staging), Ok(None));
    if manifest_written {
        let path = staging.join(sv_frameworks::names::MANIFEST);
        std::fs::write(&path, NOBODY_ANSWERED)
            .with_context(|| format!("writing {}", path.display()))?;
    }
    std::fs::rename(&staging, &folder).with_context(|| format!("naming {}", folder.display()))?;
    Ok(Fetched {
        folder,
        address: address.clone(),
        unpacked,
        manifest_written,
    })
}

#[cfg(test)]
mod tests;
