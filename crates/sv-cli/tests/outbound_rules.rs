//! The outbound rules a folder's infrastructure files set, shown beside V13.2.5 (backlog 0496): in `sv questions`, as
//! where to look and never as evidence, and nothing of them in report.json.

use std::path::PathBuf;
use std::process::Command;

const TERRAFORM: &str = "\
resource \"aws_security_group\" \"web\" {
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = \"-1\"
    cidr_blocks = [\"0.0.0.0/0\"]
  }
}
";

const POLICY: &str = "\
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: web-egress
spec:
  podSelector: {}
  policyTypes: [Egress]
  egress:
    - to:
        - ipBlock:
            cidr: 10.0.0.0/8
";

fn app(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sv-outbound-rules-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("infra")).unwrap();
    std::fs::create_dir_all(dir.join("k8s")).unwrap();
    std::fs::write(
        dir.join("app.py"),
        "from flask import Flask\napp = Flask(__name__)\n",
    )
    .unwrap();
    std::fs::write(dir.join("infra/main.tf"), TERRAFORM).unwrap();
    std::fs::write(dir.join("k8s/policy.yaml"), POLICY).unwrap();
    std::fs::write(
        dir.join("stackvet.toml"),
        "manifest-version = 1\n\n[app]\nname = \"Notes\"\naudience = \"customers\"\n\
         deployment = \"local-only\"\n\n[stack]\nlanguages = [\"python\"]\n\n\
         [capabilities]\nauth = true\nemail = true\n",
    )
    .unwrap();
    dir
}

fn sv(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_sv"))
        .args(args)
        .output()
        .expect("sv runs");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn the_questions_show_the_rules_beside_v13_2_5() {
    let dir = app("questions");
    let (ok, out, err) = sv(&["questions", dir.to_str().unwrap()]);
    std::fs::remove_dir_all(&dir).ok();
    assert!(ok, "{err}");
    let start = out
        .find(" - V13.2.5:")
        .unwrap_or_else(|| panic!("V13.2.5 is asked:\n{out}"));
    let question = &out[start..];
    let question = &question[..question[1..]
        .find("\n - ")
        .map_or(question.len(), |e| e + 1)];
    assert!(
        question.contains("The outbound rules StackVet found"),
        "{question}"
    );
    assert!(question.contains("not evidence"), "{question}");
    assert!(
        question.contains("an egress rule of the AWS security group aws_security_group.web (infra/main.tf, line 2): it lets the app reach everywhere"),
        "{question}"
    );
    assert!(
        question.contains("a Kubernetes NetworkPolicy, web-egress,"),
        "{question}"
    );
    // Shown beside V13.2.5 alone.
    assert_eq!(
        out.matches("The outbound rules StackVet found").count(),
        1,
        "{out}"
    );
}

#[test]
fn report_json_holds_none_of_them_and_v13_2_5_stays_unverified() {
    let dir = app("json");
    let out_dir = dir.join("report");
    let (ok, _, err) = sv(&[
        "report",
        dir.to_str().unwrap(),
        "--no-tools",
        "--out",
        out_dir.to_str().unwrap(),
    ]);
    let json = std::fs::read_to_string(out_dir.join("report.json")).unwrap_or_default();
    let compliance = std::fs::read_to_string(out_dir.join("compliance.md")).unwrap_or_default();
    std::fs::remove_dir_all(&dir).ok();
    assert!(ok, "{err}");
    // The setup: the report was written and asks V13.2.5 among its questions.
    assert!(
        json.contains("\"questions_for_you\""),
        "report.json was not written"
    );
    assert!(
        json.contains("The server can reach only the places you listed"),
        "V13.2.5 is not asked"
    );
    assert!(
        !json.contains("web-egress") && !json.contains("aws_security_group.web"),
        "the rules are in report.json"
    );
    // Where to look, never credit: the requirement is as verified as it was without them.
    let status = sv_check::test_support::status_of(&compliance, "V13.2.5");
    assert!(status.starts_with("not verified"), "V13.2.5: {status}");
}
