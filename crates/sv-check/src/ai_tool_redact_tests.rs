//! A key in the AI coding tool's own settings is cut before a note quotes it (backlog 0283): a hook's
//! command, a server's address or arguments, a proxy's address. The notes reach `report.html`,
//! `compliance.md`, `report.json`, and the MCP server's resources, where `sv` never prints a key.

use super::tests::rules;
use super::*;

/// A random token no vendor's shape describes, and a key the scan knows by its shape, both built
/// from pieces so this file holds neither.
fn planted() -> (String, String) {
    let token = ["Qv7rK2", "mX9pL4", "tW8nB3", "zC6"].concat();
    let key = ["sk", "ant", "api03", "Hp4Tz8Wq2Lr6Vn1Kd5Yb0Xm3"].join("-");
    (token, key)
}

#[test]
fn a_key_in_any_quoted_setting_is_cut_to_its_first_four_characters_and_length() {
    let (token, key) = planted();
    let dir = std::env::temp_dir().join(format!("sv-ai-tool-redact-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let settings = serde_json::json!({
        "hooks": { "PostToolUse": [ { "hooks": [
            { "type": "command", "command": format!("curl -H 'Authorization: Bearer {token}' https://hooks.example.com") },
            { "type": "http", "url": format!("https://hooks.example.com/in?api_key={token}") }
        ] } ] },
        "apiKeyHelper": format!("echo {key}"),
        "env": {
            "ANTHROPIC_BASE_URL": format!("https://gateway.example.com/?token={token}"),
            "HTTPS_PROXY": format!("http://builder:{token}@proxy.example.com:8080")
        }
    });
    let servers = serde_json::json!({ "mcpServers": {
        "notes": { "command": "npx", "args": ["notes-mcp@1.2.3", "--token", token] },
        "search": { "url": format!("https://mcp.example.com/sse?api_key={token}") }
    } });
    for (relative, json) in [(".claude/settings.json", settings), (".mcp.json", servers)] {
        let path = dir.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, json.to_string()).unwrap();
    }
    let files = read(&Listing::of(&dir), rules());
    let _ = std::fs::remove_dir_all(&dir);
    // The setup: every place the key was planted gave a note, so none passes by being left out.
    assert_eq!(files.notes.len(), 7, "{:#?}", files.notes);
    let leaked: Vec<&str> = files
        .notes
        .iter()
        .filter(|n| n.what.contains(&token) || n.what.contains(&key))
        .map(|n| n.what.as_str())
        .collect();
    assert!(leaked.is_empty(), "{leaked:#?}");
    for note in &files.notes {
        for secret in [&token, &key] {
            assert!(!note.what.contains(secret.as_str()), "{}", note.what);
        }
        assert!(note.what.contains("[redacted: "), "{}", note.what);
    }
    // What is kept is what `sv` keeps of any key: its first four characters and how long it is.
    let first_four: String = token.chars().take(4).collect();
    assert!(
        files.notes.iter().any(|n| n.what.contains(&first_four)),
        "{:#?}",
        files.notes
    );
}
