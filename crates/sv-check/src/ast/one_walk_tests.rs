//! Backlog 0353: a file is walked once with every rule's query for its language together, and each rule then
//! judges its own matches. It must find exactly what a walk per rule found.

use super::*;
use std::path::PathBuf;

fn load() -> AstRules {
    AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
        .expect("rules load")
}

/// What reading a file produced, in a form two reads can be compared by.
fn shown(read: &FileRead) -> String {
    let findings: Vec<_> = read
        .findings
        .iter()
        .map(|f| (&f.rule_id, f.location.line, f.confidence, &f.description))
        .collect();
    format!(
        "{findings:?} {:?} {} {}",
        read.parameter_destinations,
        read.broken.len(),
        read.parse_error
    )
}

#[test]
fn one_walk_for_every_rule_finds_what_a_walk_per_rule_finds() {
    let together = load();
    let apart = load();
    for lock in apart.together.values() {
        lock.set(None).ok();
    }
    let files = [
        (
            "python",
            "app.py",
            "import subprocess, pickle, hashlib\n\
             def go(destination):\n    return redirect(destination)\n\
             def run(cmd):\n    subprocess.call(cmd, shell=True)\n    eval(cmd)\n\
             data = pickle.loads(request.data)\nh = hashlib.md5(password.encode())\n",
        ),
        (
            "javascript",
            "server.js",
            "const { exec } = require('child_process');\n\
             app.get('/go', (req, res) => res.redirect(req.query.next));\n\
             app.get('/run', (req, res) => { exec(req.query.cmd); eval(req.body.code); });\n",
        ),
        (
            "typescript",
            "page.tsx",
            "export function Page({ html }: { html: string }) {\n\
             \x20 eval(html);\n\
             \x20 return <div dangerouslySetInnerHTML={{ __html: html }} />;\n}\n",
        ),
    ];
    for (language, relative, source) in files {
        let one = read_file(&together, language, relative, source);
        let each = read_file(&apart, language, relative, source);
        assert_eq!(shown(&one), shown(&each), "{relative}");
        // Something was found, so the comparison above is about something.
        assert!(!one.findings.is_empty(), "{relative}: nothing found");
    }
    // And the one walk was the one taken: each language's queries compiled as one.
    for language in ["python", "javascript", "tsx"] {
        assert!(together.together(language).is_some(), "{language}");
    }
}

#[test]
fn every_pattern_belongs_to_the_rule_it_was_written_in() {
    let rules = load();
    for key in rules.together.keys() {
        let together = rules.together(key).expect("the queries compile as one");
        let apart: usize = rules
            .compiled
            .iter()
            .filter_map(|c| {
                if key == "tsx" {
                    c.tsx.as_ref()
                } else {
                    c.queries.get(key.as_str())
                }
            })
            .map(|q| q.get().expect("compiles").pattern_count())
            .sum();
        assert_eq!(together.query.pattern_count(), apart, "{key}");
        for (at, c) in rules.compiled.iter().enumerate() {
            let own = if key == "tsx" {
                c.tsx.as_ref()
            } else {
                c.queries.get(key.as_str())
            };
            let owned = together
                .rule_of_pattern
                .iter()
                .filter(|r| **r == at)
                .count();
            assert_eq!(
                owned,
                own.map_or(0, |q| q.get().expect("compiles").pattern_count()),
                "{key}: {}",
                c.rule.id
            );
        }
    }
}
