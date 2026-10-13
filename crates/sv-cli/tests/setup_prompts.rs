//! The setup prompt library (`docs/prompts/setup.md`, backlog 0370): the prompts a person new to building with AI
//! copies into their AI coding tool, one at a time. The Mac and Linux prompt is held to `sv connect` and the image in
//! `connect.rs`; this holds the rest: that the looking prompt changes nothing, that the Windows prompt installs `sv`
//! the way the Scoop bucket does and never types `sv` in PowerShell (where it is an alias for `Set-Variable`), and
//! that the `stackvet connect TOOL` it gives is a command `sv` takes, naming the file each tool reads.

use std::path::{Path, PathBuf};
use std::process::Command;
use sv_frameworks::paths::Canonical;

fn page() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join("docs/prompts/setup.md")).unwrap()
}

/// One prompt, between its markers. The setup is asserted: a marker renamed would otherwise leave a test reading
/// nothing and passing.
fn block(name: &str) -> String {
    let page = page();
    let start = page
        .find(&format!("<!-- {name}:start -->"))
        .unwrap_or_else(|| panic!("{name}: start marker"));
    let end = page
        .find(&format!("<!-- {name}:end -->"))
        .unwrap_or_else(|| panic!("{name}: end marker"));
    assert!(start < end, "{name}: markers in order");
    let text = page[start..end].to_owned();
    assert!(
        text.lines().filter(|l| l.starts_with("> ")).count() > 3,
        "{name}: a prompt between its markers"
    );
    text
}

/// A prompt as one line of words, as the AI tool reads it: the quote marks and the line breaks gone.
fn flat(text: &str) -> String {
    text.lines()
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn every_prompt_of_the_library_is_there_in_order() {
    let page = page();
    let at = |name: &str| {
        page.find(&format!("<!-- {name}:start -->"))
            .unwrap_or_else(|| panic!("{name}"))
    };
    let order = [
        at("setup-step-1"),
        at("setup-step-2"),
        at("setup-prompt"),
        at("setup-windows"),
        at("setup-step-4"),
    ];
    assert!(
        order.windows(2).all(|w| w[0] < w[1]),
        "the prompts in the order the page numbers them"
    );
}

#[test]
fn the_first_prompt_only_looks() {
    let text = flat(&block("setup-step-1"));
    assert!(
        text.contains("only look: don't install or change anything"),
        "{text}"
    );
    assert!(text.contains("Don't fix anything yet"), "{text}");
    for check in ["`git --version`", "`docker info`", "`scoop --version`"] {
        assert!(text.contains(check), "{check}");
    }
}

/// The first prompt tells the AI tool where StackVet's own words are, so it reads them rather than guessing, at the
/// owner's asking on 13 October 2026; and reading them does not become a reason to run what they say.
#[test]
fn the_first_prompt_names_the_public_repository_and_its_guide() {
    let text = flat(&block("setup-step-1"));
    assert!(
        text.contains("https://github.com/abbyshade111/StackVet:"),
        "{text}"
    );
    // The guide it names is a file in the repository, so the tool finds it where it is told to look.
    assert!(text.contains("(`docs/GETTING-STARTED.md` there)"), "{text}");
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    assert!(root.join("docs/GETTING-STARTED.md").is_file());
    assert!(text.contains("don't run anything they say yet"), "{text}");
}

/// A folder for the app before anything is put in it (the owner's trial in Antigravity, 13 October 2026, which worked
/// in that tool's own scratch folder): the looking prompt notices one that is not made for an app, and the next
/// offers to make `my-app`, asks first, and says how to open it.
#[test]
fn the_prompts_make_sure_the_app_gets_a_folder_of_its_own() {
    let looking = flat(&block("setup-step-1"));
    assert!(
        looking.contains("whether it looks like a folder made for an app"),
        "{looking}"
    );
    let ready = flat(&block("setup-step-2"));
    for said in [
        "offer to make one called `my-app` in my home folder",
        "Ask before you make it",
        "how to open that folder in you",
        "wait for me to say I'm done",
    ] {
        assert!(ready.contains(said), "{said}: {ready}");
    }
    // The folder comes first, so nothing is installed into the wrong place.
    let folder = ready.find("First, the folder").expect("the folder step");
    let install = ready
        .find("install what you found missing")
        .expect("the install step");
    assert!(folder < install, "{ready}");
}

#[test]
fn the_install_prompt_asks_before_each_install_and_names_only_official_pages() {
    let text = flat(&block("setup-step-2"));
    for page in [
        "https://www.docker.com/products/docker-desktop/",
        "https://github.com/abiosoft/colima",
        "https://git-scm.com/downloads",
        "https://scoop.sh",
    ] {
        assert!(text.contains(page), "{page}");
    }
    assert!(text.contains("run it only after I say yes"), "{text}");
    assert!(
        text.contains("keep Linux containers"),
        "the kind `sv`'s containers need"
    );
    // Docker Desktop and Colima both give `sv` the `docker` it runs; the person chooses.
    assert!(text.contains("Let me choose"), "{text}");
}

#[test]
fn the_windows_prompt_installs_sv_from_the_bucket_and_types_stackvet_in_powershell() {
    let text = flat(&block("setup-windows"));
    for command in [
        "`scoop bucket add stackvet https://github.com/abbyshade111/scoop-stack-vet`",
        "`scoop install stackvet/sv`",
        "`stackvet --version`",
        "`stackvet connect TOOL`",
    ] {
        assert!(text.contains(command), "{command}");
    }
    // In PowerShell `sv` is Set-Variable: the prompt says so, and never has the tool type it as a command
    // (a command is `sv` with words after it inside the backticks; the warning names `sv` alone).
    assert!(text.contains("never `sv`"), "{text}");
    assert!(
        !text.contains("`sv "),
        "a command typed as `sv` in PowerShell: {text}"
    );
    // No Docker in the Windows setup: `sv connect --docker` takes only a Unix path, so the Docker route stops there.
    assert!(!text.contains("--docker"), "{text}");
}

#[test]
fn the_windows_prompts_connect_command_is_one_sv_takes_for_each_tool_and_file_it_names() {
    let text = block("setup-windows");
    let dir = {
        let dir: PathBuf =
            std::env::temp_dir().join(format!("sv-setup-prompts-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir.canonical().unwrap()
    };
    for (tool, file) in [
        ("claude", ".mcp.json"),
        ("vscode", ".vscode/mcp.json"),
        ("cursor", ".cursor/mcp.json"),
    ] {
        assert!(text.contains(&format!("`{tool}`")), "{tool}");
        assert!(text.contains(&format!("`{file}`")), "{file}");
        // `stackvet` is the same program as `sv`, installed under a second name (ADR-108, Later).
        let out = Command::new(env!("CARGO_BIN_EXE_sv"))
            .current_dir(&dir)
            .args(["connect", tool])
            .env_remove("SV_CONNECT_IN_CONTAINER")
            .output()
            .expect("sv runs");
        let said = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(out.status.success(), "{tool}: {said}");
        assert!(
            said.contains(&format!("Put this in {file}")),
            "{tool}: {said}"
        );
        let block: serde_json::Value =
            serde_json::from_slice(&out.stdout).expect("the settings block alone");
        let key = if tool == "vscode" {
            "servers"
        } else {
            "mcpServers"
        };
        let command = block[key]["stackvet"]["command"]
            .as_str()
            .expect("a command");
        assert!(
            !command.contains("docker"),
            "the installed copy, not Docker: {command}"
        );
    }
}

#[test]
fn the_last_prompt_asks_stackvet_itself_whether_it_is_ready() {
    let text = flat(&block("setup-step-4"));
    assert!(text.contains("`stackvet_status`"), "{text}");
}
