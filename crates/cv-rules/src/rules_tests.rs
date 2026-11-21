//! Each rule's outcomes on small folders written at run time: the witnesses that go red when a rule is turned off.

use crate::defaults::Table;
use crate::outcome::{Assessed, Outcome};
use crate::rules::{
    CONTAINER_ROOT, CSRF_COOKIE_SECURE, NODE_ENV, SESSION_COOKIE_SECURE, SV_DEV_SERVER,
};
use crate::{Checked, check};
use std::path::PathBuf;

fn folder(test: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("cv-rules-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for (path, text) in files {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    root
}

fn table() -> Table {
    let table =
        Table::load().expect("the table of defaults loads from the repository's data folder");
    assert!(
        table.defaults.len() >= 6,
        "the setup: the table is the real one"
    );
    table
}

fn run(test: &str, files: &[(&str, &str)]) -> Checked {
    let root = folder(test, files);
    let checked = check(&root, &table());
    let _ = std::fs::remove_dir_all(&root);
    checked
}

fn of<'a>(checked: &'a Checked, rule: &str) -> Vec<&'a Assessed> {
    checked
        .assessed
        .iter()
        .filter(|a| a.rule_id == rule)
        .collect()
}

const EXPRESS_4: (&str, &str) = ("package.json", r#"{"dependencies":{"express":"^4.21.2"}}"#);

#[test]
fn express_on_the_node_image_with_no_node_env_is_a_finding() {
    let checked = run(
        "node-env-missing",
        &[
            EXPRESS_4,
            (
                "Dockerfile",
                "FROM node:22-slim\nUSER node\nCMD [\"node\", \"app.js\"]\n",
            ),
        ],
    );
    let found = of(&checked, NODE_ENV.id);
    assert_eq!(found.len(), 1, "{found:?}");
    let Outcome::MissingDefaultUnsafe { defaults } = &found[0].outcome else {
        panic!("{:?}", found[0].outcome);
    };
    let ids: Vec<&str> = defaults.iter().map(|d| d.id.as_str()).collect();
    assert_eq!(ids, ["express.env", "docker-node.node-env"]);
    let finding = checked
        .evidence
        .findings
        .iter()
        .find(|f| f.rule_id == NODE_ENV.id)
        .expect("a finding");
    assert_eq!(finding.requirement_ids, ["V13.4.2", "V16.5.1"]);
    assert_eq!(finding.location.file, "Dockerfile");
    assert!(
        finding.description.contains("expressjs.com"),
        "{}",
        finding.description
    );
}

#[test]
fn node_env_set_in_each_orchestrator_layer_is_read() {
    let plan = serde_json::json!({
        "format_version": "1.2",
        "planned_values": {"root_module": {"child_modules": [{"resources": [{
            "address": "module.app.aws_ecs_task_definition.api",
            "type": "aws_ecs_task_definition",
            "values": {"container_definitions": serde_json::to_string(&serde_json::json!([
                {"name": "api", "image": "acme/api:3", "environment": [{"name": "NODE_ENV", "value": "staging"}]}
            ])).unwrap()}
        }]}]}}
    })
    .to_string();
    let checked = run(
        "node-env-layers",
        &[
            EXPRESS_4,
            (
                "compose.yaml",
                "services:\n  web:\n    image: node:24\n    environment:\n      NODE_ENV: production\n",
            ),
            (
                "deploy/k8s.yaml",
                "apiVersion: apps/v1\nkind: Deployment\nmetadata:\n  name: api\nspec:\n  template:\n    spec:\n      \
                 containers:\n        - name: api\n          image: acme/api:3\n          env:\n            - name: \
                 NODE_ENV\n              value: development\n",
            ),
            ("plan.json", &plan),
        ],
    );
    let found = of(&checked, NODE_ENV.id);
    let outcomes: Vec<(&str, &str)> = found
        .iter()
        .map(|a| (a.unit.as_str(), a.outcome.name()))
        .collect();
    assert_eq!(
        outcomes,
        [
            ("compose service `web` in compose.yaml", "set, safe"),
            (
                "container `api` of Deployment `api` in deploy/k8s.yaml",
                "set, unsafe"
            ),
            (
                "ECS container `api` of `module.app.aws_ecs_task_definition.api` in plan.json",
                "set, unsafe"
            ),
        ]
    );
    assert_eq!(
        checked
            .evidence
            .findings
            .iter()
            .filter(|f| f.rule_id == NODE_ENV.id)
            .count(),
        2
    );
    assert_eq!(
        checked
            .evidence
            .verified
            .iter()
            .filter(|v| v.check_id == NODE_ENV.id)
            .count(),
        1
    );
    assert!(
        checked.evidence.verified.iter().all(|v| v.in_part),
        "a file's word is never whole"
    );
}

#[test]
fn a_dockerfile_setting_development_is_left_to_sv() {
    let checked = run(
        "node-env-svs",
        &[
            EXPRESS_4,
            ("Dockerfile", "FROM node:22\nENV NODE_ENV=development\n"),
        ],
    );
    let found = of(&checked, NODE_ENV.id);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].outcome.name(), "set, unsafe");
    assert_eq!(found[0].reported_by, Some(SV_DEV_SERVER));
    assert!(
        checked
            .evidence
            .findings
            .iter()
            .all(|f| f.rule_id != NODE_ENV.id),
        "one rule, one home: sv's launch check reports this"
    );
}

#[test]
fn what_the_files_cannot_settle_is_not_assessed() {
    let checked = run(
        "node-env-unknown",
        &[
            EXPRESS_4,
            ("Dockerfile", "FROM node:lts\n"),
            (
                "compose.yaml",
                "services:\n  a:\n    image: node:22\n    env_file: .env.prod\n  b:\n    image: node:24\n    \
                 environment:\n      NODE_ENV: ${NODE_ENV}\n  c:\n    image: node:20\n",
            ),
        ],
    );
    let found = of(&checked, NODE_ENV.id);
    assert_eq!(found.len(), 4, "{found:?}");
    assert!(
        found.iter().all(|a| a.outcome.name() == "not assessed"),
        "{found:?}"
    );
    assert!(
        checked
            .evidence
            .findings
            .iter()
            .all(|f| f.rule_id != NODE_ENV.id)
    );
    assert!(
        checked
            .evidence
            .verified
            .iter()
            .all(|v| v.check_id != NODE_ENV.id)
    );
    assert_eq!(
        checked
            .evidence
            .not_assessed
            .iter()
            .filter(|n| n.rule_id == NODE_ENV.id)
            .count(),
        4
    );
}

#[test]
fn an_express_version_across_majors_leaves_a_missing_node_env_not_assessed() {
    let checked = run(
        "node-env-range",
        &[
            ("package.json", r#"{"dependencies":{"express":">=4"}}"#),
            ("Dockerfile", "FROM node:22\n"),
        ],
    );
    let found = of(&checked, NODE_ENV.id);
    assert_eq!(found[0].outcome.name(), "not assessed");
}

#[test]
fn no_express_means_the_rule_does_not_apply() {
    let checked = run("node-env-none", &[("Dockerfile", "FROM node:22\n")]);
    assert!(of(&checked, NODE_ENV.id).is_empty());
}

#[test]
fn express_with_no_deployment_file_says_so() {
    let checked = run("node-env-nothing", &[EXPRESS_4]);
    let found = of(&checked, NODE_ENV.id);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].outcome.name(), "not assessed");
}

#[test]
fn django_cookies_left_out_are_findings_and_set_true_is_credit_in_part() {
    let checked = run(
        "django",
        &[
            (
                "requirements.txt",
                "Django==5.2.1\ndjango-environ==0.11.2\n",
            ),
            (
                "mysite/settings.py",
                "DEBUG = False\nCSRF_COOKIE_SECURE = True  # behind TLS\n",
            ),
        ],
    );
    let session = of(&checked, SESSION_COOKIE_SECURE.id);
    assert_eq!(session.len(), 1);
    assert_eq!(session[0].outcome.name(), "missing, default unsafe");
    let csrf = of(&checked, CSRF_COOKIE_SECURE.id);
    assert_eq!(csrf[0].outcome.name(), "set, safe");
    let finding = checked
        .evidence
        .findings
        .iter()
        .find(|f| f.rule_id == SESSION_COOKIE_SECURE.id)
        .expect("a finding");
    assert_eq!(finding.requirement_ids, ["V3.3.1"]);
    let credit = checked
        .evidence
        .verified
        .iter()
        .find(|v| v.check_id == CSRF_COOKIE_SECURE.id)
        .expect("a credit");
    assert!(credit.in_part);
}

#[test]
fn django_set_false_or_from_an_expression() {
    let checked = run(
        "django-set",
        &[
            (
                "pyproject.toml",
                "[project]\ndependencies = [\"Django>=4.2,<5.0\"]\n",
            ),
            (
                "app/settings/production.py",
                "SESSION_COOKIE_SECURE = False\nCSRF_COOKIE_SECURE = env.bool(\"CSRF_SECURE\")\n",
            ),
        ],
    );
    assert_eq!(
        of(&checked, SESSION_COOKIE_SECURE.id)[0].outcome.name(),
        "set, unsafe"
    );
    assert_eq!(
        of(&checked, CSRF_COOKIE_SECURE.id)[0].outcome.name(),
        "not assessed"
    );
}

#[test]
fn a_django_version_the_table_has_not_read_is_not_assessed() {
    for (test, spec) in [("django-6", "Django==6.0"), ("django-open", "Django>=4.2")] {
        let checked = run(
            test,
            &[
                ("requirements.txt", spec),
                ("settings.py", "DEBUG = False\n"),
            ],
        );
        assert_eq!(
            of(&checked, SESSION_COOKIE_SECURE.id)[0].outcome.name(),
            "not assessed",
            "{spec}"
        );
    }
}

#[test]
fn a_container_with_no_user_on_the_node_image_runs_as_root() {
    let checked = run(
        "root-missing",
        &[("Dockerfile", "FROM node:24-bookworm-slim\nCMD [\"node\"]\n")],
    );
    let found = of(&checked, CONTAINER_ROOT.id);
    assert_eq!(found.len(), 1);
    let Outcome::MissingDefaultUnsafe { defaults } = &found[0].outcome else {
        panic!("{:?}", found[0].outcome);
    };
    assert_eq!(
        defaults.iter().map(|d| d.id.as_str()).collect::<Vec<_>>(),
        ["docker-node.user", "docker.user"]
    );
    let finding = checked
        .evidence
        .findings
        .iter()
        .find(|f| f.rule_id == CONTAINER_ROOT.id)
        .expect("a finding");
    assert!(
        finding.requirement_ids.is_empty(),
        "no ASVS requirement is cited"
    );
}

#[test]
fn the_user_is_read_in_each_layer() {
    let checked = run(
        "root-layers",
        &[
            (
                "api/Dockerfile",
                "FROM node:22 AS base\nUSER node\nFROM base\n",
            ),
            (
                "compose.yml",
                "services:\n  api:\n    build: ./api\n  admin:\n    image: acme/admin:1\n    user: \"0:0\"\n",
            ),
            (
                "k8s.yaml",
                "apiVersion: v1\nkind: Pod\nmetadata:\n  name: p\nspec:\n  securityContext:\n    runAsUser: 1000\n  \
                 containers:\n    - name: a\n      image: x\n    - name: b\n      image: y\n      securityContext:\n        \
                 runAsUser: 0\n    - name: c\n      image: z\n",
            ),
        ],
    );
    let found = of(&checked, CONTAINER_ROOT.id);
    let outcomes: Vec<(&str, &str)> = found
        .iter()
        .map(|a| (a.unit.as_str(), a.outcome.name()))
        .collect();
    assert_eq!(
        outcomes,
        [
            ("compose service `api` in compose.yml", "set, safe"),
            ("compose service `admin` in compose.yml", "set, unsafe"),
            ("container `a` of Pod `p` in k8s.yaml", "set, safe"),
            ("container `b` of Pod `p` in k8s.yaml", "set, unsafe"),
            ("container `c` of Pod `p` in k8s.yaml", "set, safe"),
        ],
        "the Dockerfile is spoken for by the service built from it; a container's own runAsUser outranks the pod's"
    );
}

#[test]
fn kubernetes_with_no_user_is_not_assessed() {
    let checked = run(
        "root-k8s-unknown",
        &[(
            "pod.yaml",
            "apiVersion: v1\nkind: Pod\nmetadata:\n  name: p\nspec:\n  containers:\n    - name: a\n      image: node:22\n",
        )],
    );
    assert_eq!(
        of(&checked, CONTAINER_ROOT.id)[0].outcome.name(),
        "not assessed"
    );
}

#[test]
fn a_file_that_cannot_be_read_is_named() {
    let checked = run(
        "unread",
        &[
            ("compose.yaml", "services: [\n"),
            ("Dockerfile", "RUN echo before any FROM\n"),
        ],
    );
    let files: Vec<&str> = checked.unread.iter().map(|u| u.file.as_str()).collect();
    assert_eq!(files, ["Dockerfile", "compose.yaml"]);
    assert!(checked.read.is_empty());
}
