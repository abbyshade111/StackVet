//! The four readers and the version readers, each on the forms its files are written in.

use crate::defaults::{Table, Version};
use crate::rules::{django_spec, npm_major, pip_releases};
use crate::unit::{Image, join};
use crate::{compose, dockerfile, kubernetes, terraform};

#[test]
fn a_dockerfile_s_stages_env_and_user() {
    let text = "# build\nFROM --platform=$BUILDPLATFORM node:22 AS Build\nENV A=1 B=\"two words\" \\\n    C=3\n\
                ENV OLD style value\nUSER node\nFROM build\nenv A=4\n";
    let stages = dockerfile::parse(text).unwrap();
    assert_eq!(stages.len(), 2);
    assert_eq!(stages[0].from, "node:22");
    assert_eq!(stages[0].alias.as_deref(), Some("build"));
    let names: Vec<(&str, &str, usize)> = stages[0]
        .env
        .iter()
        .map(|s| (s.name.as_str(), s.value.as_str(), s.line))
        .collect();
    assert_eq!(
        names,
        [
            ("A", "1", 3),
            ("B", "two words", 3),
            ("C", "3", 3),
            ("OLD", "style value", 5)
        ]
    );
    let eff = dockerfile::effective(&stages);
    assert_eq!(eff.base, "node:22");
    assert_eq!(eff.user, Some(("node".to_owned(), 6)));
    let (a, final_stage) = eff.env("A").unwrap();
    assert_eq!((a.value.as_str(), *final_stage), ("4", true));
    assert!(!eff.env("B").unwrap().1, "B is set in the stage built on");
}

#[test]
fn a_dockerfile_with_no_from_is_refused() {
    assert!(dockerfile::parse("RUN true\n").is_err());
    assert!(dockerfile::parse("# nothing\n").is_err());
}

#[test]
fn compose_services_in_both_environment_forms() {
    let text = "services:\n  web:\n    build:\n      context: ./app\n      dockerfile: docker/Dockerfile.prod\n    \
                environment:\n      - NODE_ENV=production\n      - PASSED_THROUGH\n  db:\n    image: postgres:16\n    \
                user: \"999\"\n    env_file:\n      - path: db.env\n";
    let units = compose::read("deploy/compose.yaml", text).unwrap();
    assert_eq!(units.len(), 2);
    let web = &units[0];
    assert_eq!(
        web.build.as_ref().unwrap().dockerfile,
        "deploy/app/docker/Dockerfile.prod"
    );
    assert_eq!(
        web.env("NODE_ENV").unwrap().value.as_deref(),
        Some("production")
    );
    assert_eq!(web.env("PASSED_THROUGH").unwrap().value, None);
    let db = &units[1];
    assert_eq!(db.user, Some(("999".to_owned(), 11)));
    assert_eq!(db.env_elsewhere.as_deref(), Some("db.env"));
    assert!(compose::is_compose("docker-compose.prod.yml"));
    assert!(!compose::is_compose("config.yaml"));
}

#[test]
fn kubernetes_workloads_and_their_containers() {
    let text = "apiVersion: batch/v1\nkind: CronJob\nmetadata:\n  name: nightly\nspec:\n  jobTemplate:\n    spec:\n      \
                template:\n        spec:\n          containers:\n            - name: run\n              image: a:1\n              \
                envFrom:\n                - configMapRef:\n                    name: settings\n---\napiVersion: v1\nkind: \
                ConfigMap\nmetadata:\n  name: settings\n";
    assert!(kubernetes::looks_like(text));
    let units = kubernetes::read("k.yaml", text).unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].name, "container `run` of CronJob `nightly`");
    assert_eq!(units[0].env_elsewhere.as_deref(), Some("envFrom"));
}

#[test]
fn a_kubernetes_file_with_an_alias_is_refused_not_followed() {
    let text = "apiVersion: v1\nkind: Pod\nmetadata: &m\n  name: p\nspec:\n  containers: []\n";
    assert!(kubernetes::read("k.yaml", text).is_err());
}

#[test]
fn a_terraform_plan_s_ecs_containers() {
    let definitions = r#"[{"name":"api","image":"node:22","user":"1000","environment":[{"name":"NODE_ENV","value":"production"}],"secrets":[{"name":"DB_PASSWORD","valueFrom":"arn:aws:ssm:x"}]}]"#;
    let plan = serde_json::json!({
        "format_version": "1.2",
        "planned_values": {"root_module": {"resources": [
            {"address": "aws_s3_bucket.b", "type": "aws_s3_bucket", "values": {}},
            {"address": "aws_ecs_task_definition.api", "type": "aws_ecs_task_definition",
             "values": {"container_definitions": definitions}}
        ]}}
    });
    let text = serde_json::to_string_pretty(&plan).unwrap();
    assert!(terraform::looks_like(&text));
    let units = terraform::read("plan.json", &text).unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].user.as_ref().unwrap().0, "1000");
    assert_eq!(
        units[0].env("NODE_ENV").unwrap().value.as_deref(),
        Some("production")
    );
    assert_eq!(
        units[0].env("DB_PASSWORD").unwrap().value,
        None,
        "a secret's value is never in the plan"
    );
    assert!(units[0].line > 1, "the line of the resource's address");
}

#[test]
fn images_and_paths() {
    let image = Image::parse("docker.io/library/node:22.11-alpine@sha256:abc");
    assert!(image.is_official("node"));
    assert_eq!(image.major(), Some(22));
    assert_eq!(Image::parse("node:lts").major(), None);
    assert_eq!(Image::parse("node").major(), None);
    assert!(!Image::parse("ghcr.io/acme/node:22").is_official("node"));
    assert!(!Image::parse("registry:5000/node").is_official("node"));
    assert_eq!(
        join("deploy", "../app/./Dockerfile").as_deref(),
        Some("app/Dockerfile")
    );
    assert_eq!(join("", "../outside"), None);
}

#[test]
fn npm_and_pip_versions() {
    for (spec, major) in [
        ("^4.21.2", Some(4)),
        ("~5.1.0", Some(5)),
        ("4.x", Some(4)),
        ("5", Some(5)),
        (">=4", None),
        ("*", None),
        ("4 || 5", None),
        ("github:expressjs/express", None),
    ] {
        assert_eq!(npm_major(spec), major, "{spec}");
    }
    for (line, spec) in [
        ("Django==5.2.1", Some("==5.2.1")),
        ("django[argon2]>=4.2,<5.0 # pinned", Some(">=4.2,<5.0")),
        ("  \"Django>=5.1,<5.2\",", Some(">=5.1,<5.2")),
        ("django = \"==5.1\"", Some("==5.1")),
        ("django = \"5.1.4\"", Some("==5.1.4")),
        ("django-environ==0.11", None),
        ("djangorestframework==3.15", None),
    ] {
        assert_eq!(django_spec(line).as_deref(), spec, "{line}");
    }
    assert_eq!(
        pip_releases("==5.2.1"),
        Some(Version::Releases((5, 2), (5, 3)))
    );
    assert_eq!(
        pip_releases(">=4.2,<5.0"),
        Some(Version::Releases((4, 2), (5, 0)))
    );
    assert_eq!(
        pip_releases("~=5.1"),
        Some(Version::Releases((5, 1), (6, 0)))
    );
    assert_eq!(pip_releases(">=4.2"), None);
}

#[test]
fn the_table_holds_a_default_only_for_the_versions_read() {
    let table = Table::load().unwrap();
    assert!(table.holding("express.env", Version::Major(4)).is_some());
    assert!(table.holding("express.env", Version::Major(3)).is_none());
    assert!(
        table
            .holding("docker-node.node-env", Version::Major(22))
            .is_some()
    );
    assert!(
        table
            .holding("docker-node.node-env", Version::Major(20))
            .is_none()
    );
    let django = "django.SESSION_COOKIE_SECURE";
    assert!(
        table
            .holding(django, Version::Releases((4, 2), (5, 3)))
            .is_some()
    );
    assert!(
        table
            .holding(django, Version::Releases((4, 1), (4, 2)))
            .is_none()
    );
    assert!(
        table
            .holding(django, Version::Releases((5, 2), (6, 0)))
            .is_none()
    );
    assert!(table.holding("docker.user", Version::Any).is_some());
    assert!(table.holding("express.env", Version::Any).is_none());
}

#[test]
fn every_default_names_its_source() {
    let table = Table::load().unwrap();
    let mut ids: Vec<&str> = table.defaults.iter().map(|d| d.id.as_str()).collect();
    for default in &table.defaults {
        assert!(!default.sources.is_empty(), "{}", default.id);
        for source in &default.sources {
            assert!(
                source.url.starts_with("https://"),
                "{}: {}",
                default.id,
                source.url
            );
            assert_eq!(
                source.read.len(),
                10,
                "{}: a date read, YYYY-MM-DD",
                default.id
            );
        }
        assert!(
            default.versions != crate::defaults::Versions::default(),
            "{}: names the versions it was read for",
            default.id
        );
    }
    ids.sort_unstable();
    let count = ids.len();
    ids.dedup();
    assert_eq!(ids.len(), count, "ids are unique");
    for asked in [
        "express.env",
        "docker-node.node-env",
        "docker-node.user",
        "docker.user",
        "django.SESSION_COOKIE_SECURE",
        "django.CSRF_COOKIE_SECURE",
    ] {
        assert!(ids.contains(&asked), "the rules ask for {asked}");
    }
}
