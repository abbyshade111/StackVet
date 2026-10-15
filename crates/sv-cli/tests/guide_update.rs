//! "Keeping StackVet up to date" (`docs/GAP-ANALYSIS.md`, 5.3; in `docs/INSTALL.md` since backlog 0440), held to the workflow that
//! publishes the image and to what `sv --version` prints: a guide that names an image nobody publishes,
//! or a tag that does not exist, sends the owner to an update that never comes.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &str) -> String {
    std::fs::read_to_string(repo().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The image the publishing job pushes, from its `IMAGE:` setting.
fn published_image(workflow: &str) -> String {
    let publish = workflow
        .split("\n  publish:\n")
        .nth(1)
        .expect("the workflow has a publish job");
    publish
        .lines()
        .find_map(|l| l.trim().strip_prefix("IMAGE: "))
        .expect("the publish job names its image")
        .trim()
        .to_owned()
}

/// The guide's section on updating, up to the next section.
fn update_section(guide: &str) -> String {
    let start = guide
        .find("\n## Keeping StackVet up to date\n")
        .expect("the guide has a section on keeping StackVet up to date");
    let rest = &guide[start + 1..];
    let end = rest[3..].find("\n## ").map_or(rest.len(), |e| e + 3);
    rest[..end].to_owned()
}

#[test]
fn the_guide_updates_the_image_the_workflow_publishes() {
    let workflow = read(".github/workflows/rust.yml");
    let image = published_image(&workflow);
    assert_eq!(image, "ghcr.io/abbyshade111/stackvet-sv", "the setup");
    let install = read("docs/INSTALL.md");
    let section = update_section(&install);

    assert!(
        section.contains(&format!("docker pull {image}\n")),
        "{section}"
    );
    assert!(
        section.contains(&format!("docker run --rm {image} --version\n")),
        "{section}"
    );
    // The copy built on this computer is updated as the installing section says.
    assert!(
        section.contains("`git pull`, then `sh tools/install.sh`"),
        "{section}"
    );
    assert!(install.contains("then `git pull`, then `sh tools/install.sh` again"));
    // A new version is picked up only when the tool starts the server again.
    assert!(section.contains("restart your AI tool"), "{section}");

    // Every image the guide and the install page name is the one published, so a rename cannot leave either behind.
    let guide = read("docs/GETTING-STARTED.md") + &install;
    let named = guide.matches("ghcr.io/").count();
    assert!(
        named >= 8,
        "the setup: the guide names the image throughout"
    );
    assert_eq!(
        guide
            .matches(&format!("ghcr.io/{}", &image["ghcr.io/".len()..]))
            .count(),
        named,
        "the guide names an image the workflow does not publish"
    );
}

#[test]
fn a_version_can_be_held_to_by_its_commit() {
    // The guide says every image is published under the commit `--version` prints. Both come from the
    // same `github.sha`, or the pinned name would be one nobody pushed. Each platform's image is built
    // once, in the `image` and `image-arm64` jobs, and `publish` pushes those same images, loaded from
    // them (backlog 0343), and one manifest naming both under the commit (backlog 0539).
    let workflow = read(".github/workflows/rust.yml");
    let publish = workflow.split("\n  publish:\n").nth(1).unwrap();
    let job = |name: &str| {
        workflow
            .split(&format!("\n  {name}:\n"))
            .nth(1)
            .and_then(|rest| rest.split("\n  publish:\n").next())
            .and_then(|rest| rest.split("\n  image-arm64:\n").next())
            .unwrap_or_else(|| panic!("the workflow has a {name} job before publish"))
            .to_owned()
    };
    for name in ["image", "image-arm64"] {
        assert!(
            job(name).contains("--build-arg SV_GIT_COMMIT=${{ github.sha }}"),
            "{name}: the image's --version names another commit than its tag"
        );
    }
    // `docker buildx imagetools` joins images already pushed; it builds nothing.
    assert!(
        !publish.lines().any(|l| {
            let l = l.trim();
            (l.starts_with("docker build") && !l.starts_with("docker buildx imagetools"))
                || l.contains("buildx build")
        }),
        "publish builds an image of its own rather than pushing the one that was tested"
    );
    assert!(
        publish.contains("docker load")
            && publish.contains("pattern: sv-image*")
            && publish.contains("docker tag stackvet/sv \"$IMAGE:${{ github.sha }}-$arch\""),
        "publish does not push the images the image jobs built"
    );
    assert!(
        publish.contains("docker buildx imagetools create")
            && publish.contains("-t \"$IMAGE:${{ github.sha }}\""),
        "no image is published under its commit"
    );
    assert!(
        publish.contains("-t \"$IMAGE:latest\""),
        "the newest is not published as latest"
    );
    let install = read("docs/INSTALL.md");
    assert!(
        update_section(&install).contains("followed by the whole commit that `--version` prints")
    );

    // And `--version` prints the whole commit, as the guide says: 40 letters and digits.
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .arg("--version")
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&out.stdout);
    let first = said.lines().next().unwrap_or_default();
    let commit = first
        .strip_prefix(&format!("sv {} (commit ", env!("CARGO_PKG_VERSION")))
        .and_then(|rest| rest.strip_suffix(')'))
        .unwrap_or_else(|| panic!("not the line the guide shows: {first}"));
    assert!(
        commit == "unknown"
            || (commit.len() == 40 && commit.chars().all(|c| c.is_ascii_hexdigit())),
        "{commit}"
    );
}
