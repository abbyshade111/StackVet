//! Each outside tool says its name as it begins, so a person watching `sv report --tools` at a
//! terminal sees which one is running (backlog 226, part 2, item 15).

use super::*;
use std::sync::Mutex;

const TOOL: &str = r#"#!/bin/sh
if [ "$1" = --version ]; then echo "faketool 1.0"; exit 0; fi
printf '{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"Fake","rules":[]}},"results":[]}]}' > "$2"
exit 0
"#;

fn adapter(id: &str, name: &str, tool: &str) -> Adapter {
    serde_json::from_value(serde_json::json!({
        "id": id,
        "name": name,
        "language": "python",
        "version": { "command": tool, "args": ["--version"] },
        "run": { "command": tool, "args": ["{dir}", "{output}"] },
        "install": "get it",
        "finished_exits": [0],
        "rules": {},
    }))
    .unwrap()
}

#[test]
fn each_tool_is_named_as_it_begins_in_the_order_they_run() {
    let dir = std::env::temp_dir().join(format!("sv-tool-progress-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("app")).unwrap();
    std::fs::write(dir.join("app/app.py"), "print(1)\n").unwrap();
    let tool = dir.join("faketool");
    crate::test_support::executable(&tool, TOOL);
    let tool = tool.display().to_string();
    let rules = SecretRules::load(&sv_frameworks::data::file("secret-rules.json")).unwrap();
    let said = Mutex::new(Vec::new());
    let run = run_all_in(
        &Adapters {
            adapters: vec![
                adapter("one", "First", &tool),
                adapter("two", "Second", &tool),
            ],
        },
        &sv_scan::files::Listing::of(&dir.join("app")),
        &["python".to_owned()],
        &BTreeSet::new(),
        &dir,
        &rules,
        &|name| said.lock().unwrap().push(name.to_owned()),
    );
    std::fs::remove_dir_all(&dir).ok();
    // The setup: both tools really ran, so the names stand for runs and not for skips.
    assert_eq!(run.ran, ["one", "two"], "{run:?}");
    assert_eq!(said.into_inner().unwrap(), ["First", "Second"]);
}

/// A tool that writes its mark, then waits up to ten seconds for the other's, and reports whether it saw it.
fn waiting(mine: &std::path::Path, theirs: &std::path::Path) -> String {
    format!(
        "#!/bin/sh\n\
         if [ \"$1\" = --version ]; then echo \"faketool 1.0\"; exit 0; fi\n\
         : > '{mine}'\n\
         i=0; while [ ! -e '{theirs}' ] && [ $i -lt 100 ]; do sleep 0.1; i=$((i+1)); done\n\
         [ -e '{theirs}' ] || exit 9\n\
         printf '{{\"version\":\"2.1.0\",\"runs\":[{{\"tool\":{{\"driver\":{{\"name\":\"Fake\",\"rules\":[]}}}},\"results\":[]}}]}}' > \"$2\"\n\
         exit 0\n",
        mine = mine.display(),
        theirs = theirs.display()
    )
}

#[test]
fn a_small_tool_runs_beside_a_heavy_one_and_the_results_keep_the_adapters_order() {
    // Each tool waits for the other's mark: run one after the other, the first would wait out its ten
    // seconds and fail, so both finishing cleanly shows they ran at the same time (backlog 0368).
    let dir = std::env::temp_dir().join(format!("sv-tool-beside-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("app")).unwrap();
    std::fs::write(dir.join("app/app.py"), "print(1)\n").unwrap();
    let (small_mark, heavy_mark) = (dir.join("small.mark"), dir.join("heavy.mark"));
    let small = dir.join("small-tool");
    let heavy = dir.join("heavy-tool");
    crate::test_support::executable(&small, &waiting(&small_mark, &heavy_mark));
    crate::test_support::executable(&heavy, &waiting(&heavy_mark, &small_mark));
    let rules = SecretRules::load(&sv_frameworks::data::file("secret-rules.json")).unwrap();
    let said = Mutex::new(Vec::new());
    let run = run_all_in(
        &Adapters {
            adapters: vec![
                adapter("semgrep", "Heavy", &heavy.display().to_string()),
                adapter("bandit", "Small", &small.display().to_string()),
            ],
        },
        &sv_scan::files::Listing::of(&dir.join("app")),
        &["python".to_owned()],
        &BTreeSet::new(),
        &dir,
        &rules,
        &|name| said.lock().unwrap().push(name.to_owned()),
    );
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(run.ran, ["semgrep", "bandit"], "{run:?}");
    let tools: Vec<&str> = run.tools.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(tools, ["semgrep", "bandit"]);
    let mut said = said.into_inner().unwrap();
    said.sort();
    assert_eq!(said, ["Heavy", "Small"]);
}
