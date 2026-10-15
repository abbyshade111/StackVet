//! Backlog 0525: a query given by a function in the same file whose one return is fixed text is that text. In the
//! three-tool prompt trial of 14 October 2026 the VS Code app ran `connection.executescript(_database_schema())`, where
//! `_database_schema` returned one triple-quoted schema, and `ast.sql-built-by-hand` called it a query built by hand,
//! the app's only high finding. Every way the function stops giving fixed text is still reported.

use super::*;
use std::path::PathBuf;

fn built_by_hand(source: &str) -> Vec<usize> {
    let rules = AstRules::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"),
    )
    .expect("rules load");
    scan_file(&rules, "python", "app.py", source)
        .into_iter()
        .filter(|f| f.rule_id == "ast.sql-built-by-hand")
        .map(|f| f.location.line)
        .collect()
}

/// The VS Code app's shape: the schema function, and the call that runs it.
const SCHEMA: &str = "def _database_schema():\n    \
                      return \"\"\"\n    CREATE TABLE notes (id INTEGER PRIMARY KEY, body TEXT);\n    \"\"\"\n\n";
const RUN: &str = "def init_db(connection):\n    connection.executescript(_database_schema())\n";

#[test]
fn a_schema_given_by_a_function_that_returns_fixed_text_is_not_flagged() {
    assert_eq!(
        built_by_hand(&format!("{SCHEMA}{RUN}")),
        Vec::<usize>::new()
    );
    // Its return naming a module constant is fixed text too.
    let constant = "SCHEMA = \"CREATE TABLE t (id INTEGER)\"\ndef schema():\n    return SCHEMA\n\n";
    assert_eq!(
        built_by_hand(&format!(
            "{constant}def f(c):\n    c.executescript(schema())\n"
        )),
        Vec::<usize>::new()
    );
}

#[test]
fn every_way_the_function_stops_giving_fixed_text_is_still_reported() {
    let run_at = |source: &str| built_by_hand(source);
    // The control, so the cases below are known to reach the call: a query joined with a value.
    let joined = "def f(c, name):\n    c.execute(\"SELECT * FROM t WHERE n = '\" + name + \"'\")\n";
    assert_eq!(run_at(joined), vec![2]);
    for (why, function) in [
        (
            "its return joins in what it is given",
            "def _database_schema(name):\n    return \"SELECT * FROM t WHERE n = '\" + name + \"'\"\n\n",
        ),
        (
            "it has a second return",
            "def _database_schema(x):\n    if x:\n        return input()\n    return \"SELECT 1\"\n\n",
        ),
        (
            "its return is an f-string with a value in it",
            "def _database_schema(t):\n    return f\"SELECT * FROM {t}\"\n\n",
        ),
        (
            "it yields",
            "def _database_schema():\n    yield \"SELECT 1\"\n    return \"SELECT 2\"\n\n",
        ),
    ] {
        let source = format!("{function}{RUN}");
        let lines = run_at(&source);
        assert!(!lines.is_empty(), "{why}: not reported\n{source}");
    }
    // Defined twice, or its name bound again, which of them runs cannot be told.
    let twice = format!("{SCHEMA}def _database_schema():\n    return input()\n\n{RUN}");
    assert!(!run_at(&twice).is_empty(), "defined twice: not reported");
    let rebound = format!("{SCHEMA}_database_schema = lambda: input()\n{RUN}");
    assert!(!run_at(&rebound).is_empty(), "rebound: not reported");
    // A method of the same name on another object is not the module's function.
    let method =
        format!("{SCHEMA}def f(c, other):\n    c.executescript(other._database_schema())\n");
    assert!(
        !run_at(&method).is_empty(),
        "another object's method: not reported"
    );
}
