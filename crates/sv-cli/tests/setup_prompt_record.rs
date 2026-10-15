//! Backlog 0551, from the three-tool prompt trial of 14 October 2026: in Cursor, prompts 1 to 4 ran in a cloud agent
//! on Linux, whose settings pointed at `/usr/bin/docker`, so the Mac's start of the server failed four times; in
//! Antigravity, prompts 1 to 3 left no record. Prompt 1 now asks for a chat on this computer and has the tool show
//! the system and the folder; prompt 3, in both versions, has it paste the entry it wrote into the chat.

use std::path::Path;

fn page() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join("docs/prompts/setup.md")).unwrap()
}

/// One prompt, between its markers, as one line of words. The setup is asserted: a marker renamed would otherwise
/// leave a test reading nothing.
fn prompt(name: &str) -> String {
    let page = page();
    let start = page
        .find(&format!("<!-- {name}:start -->"))
        .unwrap_or_else(|| panic!("{name}: start marker"));
    let end = page
        .find(&format!("<!-- {name}:end -->"))
        .unwrap_or_else(|| panic!("{name}: end marker"));
    let text = page[start..end]
        .lines()
        .map(|l| l.trim_start_matches('>').trim())
        .collect::<Vec<_>>()
        .join(" ");
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(text.len() > 300, "{name}: a prompt between its markers");
    text
}

#[test]
fn prompt_1_runs_on_this_computer_and_shows_where() {
    let looking = prompt("setup-step-1");
    for said in [
        "This has to happen in a chat that runs on this computer, not in a cloud or background agent: if you are \
         one of those, tell me and stop.",
        "On a Mac or Linux, show me what `uname -a` prints.",
        "Which folder you are working in (show me what `pwd` prints)",
    ] {
        assert!(looking.contains(said), "{said}: {looking}");
    }
}

#[test]
fn prompt_3_pastes_the_entry_it_wrote_in_both_versions() {
    let said = "Paste the `stackvet` entry you wrote into this chat, and nothing else from the file, so the chat \
                keeps a record.";
    for name in ["setup-prompt", "setup-windows"] {
        let text = prompt(name);
        assert!(text.contains(said), "{name}: {text}");
    }
}

/// Backlog 0552: in the Antigravity trial the tool spent 16 minutes connected to a container that served an older
/// app's folder. `stackvet_status` names the folder it serves (backlog 0541), and asked about a folder outside it says
/// so, so prompt 4 has the tool ask about this folder by its full path, and stop an old container only with a yes.
#[test]
fn prompt_4_asks_about_this_folder_and_stops_an_old_container_only_with_a_yes() {
    let check = prompt("setup-step-4");
    for said in [
        "call it with this folder's full path (`pwd` prints it)",
        "If that is not this folder, or it says this folder is not inside the folder the server was started for, a \
         StackVet server started for an earlier app is still running: on a Mac or Linux, show me what `docker ps` \
         lists, stop the old StackVet container only after I say yes, and tell me to restart you.",
    ] {
        assert!(check.contains(said), "{said}: {check}");
    }
}
