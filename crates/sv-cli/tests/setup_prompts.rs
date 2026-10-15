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
        at("setup-step-5"),
        at("setup-step-6"),
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
/// offers to make one, `my-app` or a name the person chooses, asks first, and says how to open it.
#[test]
fn the_prompts_make_sure_the_app_gets_a_folder_of_its_own() {
    let looking = flat(&block("setup-step-1"));
    assert!(
        looking.contains("whether it looks like a folder made for an app"),
        "{looking}"
    );
    let ready = flat(&block("setup-step-2"));
    for said in [
        // Backlog 0558: the name is the person's to choose.
        "offer to make one in my home folder, called `my-app` or any name I choose: ask me which",
        "Ask before you make it",
        "how to open that folder in the app I'm using now",
        // In the trial, opening the folder in Antigravity started a new chat that knew nothing of the first.
        "Opening it may start a new chat with you",
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

/// Every prompt says what has to happen, never which button or menu does it in a given app: an app's menus differ
/// from one to the next and change over time, and the AI tool knows its own (the owner, 13 October 2026, after the
/// trial in Antigravity was given a menu that app does not have). Where the person has to do something in the app,
/// the prompt asks for the app's own words, and for the tool to say so when it is not sure.
#[test]
fn no_prompt_names_an_apps_buttons_or_menus() {
    let names = [
        "setup-step-1",
        "setup-step-2",
        "setup-prompt",
        "setup-windows",
        "setup-step-4",
        "setup-step-5",
        "setup-step-6",
    ];
    // The setup: every prompt is read, or a renamed marker would leave one out unseen.
    assert_eq!(names.len(), 7);
    for name in names {
        let text = flat(&block(name));
        assert!(!text.is_empty(), "{name}");
        for app_specific in [
            "click **",
            "menu bar",
            "Open Folder",
            " → ",
            "Docker Desktop asks",
        ] {
            assert!(
                !text.contains(app_specific),
                "{name} says {app_specific:?}: {text}"
            );
        }
    }
    let folder = flat(&block("setup-step-2"));
    assert!(
        folder.contains("in that app's own words for its menus and buttons"),
        "{folder}"
    );
    for name in ["setup-step-2", "setup-prompt", "setup-windows"] {
        let text = flat(&block(name));
        assert!(
            text.contains("If you are not sure how this app does it, say so"),
            "{name}: {text}"
        );
    }
    for name in ["setup-prompt", "setup-windows"] {
        let text = flat(&block(name));
        assert!(
            text.contains("so you load the new settings, in this app's own words"),
            "{name}: {text}"
        );
    }
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
        text.contains("StackVet needs Docker to run Linux containers"),
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
        "`stackvet connect`",
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
fn the_windows_prompts_connect_command_prints_stackvets_server_in_no_apps_format() {
    let text = block("setup-windows");
    let dir = {
        let dir: PathBuf =
            std::env::temp_dir().join(format!("sv-setup-prompts-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).unwrap();
        dir.canonical().unwrap()
    };
    assert!(text.contains("`stackvet connect`."), "{text}");
    // `stackvet` is the same program as `sv`, installed under a second name (ADR-108, Later).
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .current_dir(&dir)
        .args(["connect"])
        .env_remove("SV_CONNECT_IN_CONTAINER")
        .output()
        .expect("sv runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{said}");
    let server: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("the server alone on standard output");
    assert_eq!(server["name"], "stackvet", "{server}");
    assert_eq!(server["transport"], "stdio", "{server}");
    let command = server["command"].as_str().expect("a command");
    assert!(
        !command.contains("docker"),
        "the installed copy, not Docker: {command}"
    );
    for app in [
        "`claude`",
        "`vscode`",
        "`cursor`",
        ".mcp.json",
        ".vscode/",
        ".cursor/",
        "connect TOOL",
    ] {
        assert!(!text.contains(app), "{app} in the Windows prompt");
    }
}

/// What the trial in Antigravity (13 October 2026) showed prompt 3 needs, whichever app it is pasted into: the app's
/// own documentation, not a guess; a settings file kept for the whole account named, and asked about before it is
/// read or changed; what is already in it kept; and the file it wrote said, since a new chat may not remember it.
#[test]
fn prompt_three_has_the_tool_add_stackvet_its_own_way_and_keep_what_is_there() {
    for name in ["setup-prompt", "setup-windows"] {
        let text = flat(&block(name));
        for said in [
            "from your own app's documentation, how this app is given a local MCP server",
            "If you are not sure, say so and stop",
            "one file for my whole account",
            "ask me before you read it or change it",
            "Read the file once",
            "exactly as printed",
            "keeping all of it",
            "Write it only after I say yes",
            "paste prompt 4, \"Check that it's connected\", from the setup page",
            "which settings file you wrote",
        ] {
            assert!(text.contains(said), "{name}: {said}");
        }
    }
    let check = flat(&block("setup-step-4"));
    assert!(
        check.contains("don't search my computer for others"),
        "{check}"
    );
    assert!(
        check.contains("rather than telling me to start Docker"),
        "{check}"
    );
}

/// The build prompt on the setup page is the guide's, word for word, so the two cannot drift apart.
#[test]
fn the_setup_pages_build_prompt_is_the_guides() {
    let quoted = |text: &str| -> String {
        text.lines()
            .filter(|l| l.starts_with('>'))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let ours = flat(&quoted(&block("setup-step-5")));
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let guide = std::fs::read_to_string(root.join("docs/GETTING-STARTED.md")).unwrap();
    let start = guide
        .find("## 4. Start the build with this prompt")
        .expect("the guide's step 4");
    let theirs: String = quoted(
        &guide[start..]
            .lines()
            .skip_while(|l| !l.starts_with('>'))
            .take_while(|l| l.starts_with('>'))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    // The setup: the guide's prompt was found, and is the build prompt.
    assert!(theirs.contains("Call `stackvet_spec`"), "{theirs}");
    assert_eq!(ours, flat(&theirs));
}

#[test]
fn the_last_prompt_asks_stackvet_itself_whether_it_is_ready() {
    let text = flat(&block("setup-step-4"));
    assert!(text.contains("`stackvet_status`"), "{text}");
}

/// Prompt 6 walks the person through the full report at the terminal (backlog 0398), which StackVet's MCP server
/// makes neither part of (ADR-066): every `sv` command it names is one `sv` takes, the guide section it sends the tool
/// to is there, and it holds what the trial in Antigravity showed (13 October 2026): StackVet's age checked against
/// GitHub, both copies kept current, no container stopped or reached into, and Docker never handed to StackVet.
#[test]
fn prompt_six_walks_the_person_through_the_full_report() {
    let text = flat(&block("setup-step-6"));
    for command in ["report", "doctor", "--version"] {
        let out = Command::new(env!("CARGO_BIN_EXE_sv"))
            .args(if command.starts_with('-') {
                vec![command]
            } else {
                vec![command, "--help"]
            })
            .output()
            .expect("sv runs");
        assert!(
            out.status.success(),
            "`sv {command}` is not a command sv takes"
        );
    }
    for said in [
        "`sv --version`",
        "`stackvet --version`",
        "compare the commit `sv --version` prints with the latest on StackVet's GitHub page",
        "StackVet never checks for itself",
        "`brew upgrade --fetch-HEAD sv`",
        "`scoop update sv`",
        "`docker pull ghcr.io/abbyshade111/stackvet-sv`",
        "Don't stop or restart any container",
        "`sv doctor`",
        "`sv report . --run`",
        "`stackvet report . --run`",
        "ask me to allow it, or tell me the command to type myself",
        "Never give StackVet's container access to Docker, and never run commands inside a container",
        "Never tell me the app is secure",
    ] {
        assert!(text.contains(said), "{said}: {text}");
    }
    for never in ["docker.sock", "docker exec", "--tools"] {
        assert!(!text.contains(never), "{never} in prompt 6");
    }
    // The section it points to is there, by that name, on the page it names (backlog 0440 moved it there).
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let install = std::fs::read_to_string(root.join("docs/INSTALL.md")).unwrap();
    assert!(text.contains("(`docs/INSTALL.md`, the"), "{text}");
    assert!(
        text.contains("\"Installing StackVet on your computer, for `--run`\""),
        "{text}"
    );
    assert!(install.contains("\n## Installing StackVet on your computer, for `--run`\n"));
    // And the page says answers count as the person's own only through `sv review`, typed by them.
    let page = page();
    assert!(
        page.contains("until you record them with `sv review`"),
        "the page"
    );
}
