//! The outbound rules a folder's infrastructure files set (backlog 0496): each kind read, with whether it lets the app
//! reach everywhere, and what is not an outbound rule left alone.

use super::*;

#[test]
fn a_network_policy_with_egress_is_read_with_whether_it_allows_everywhere() {
    let text = "\
apiVersion: v1
kind: Service
metadata:
  name: web
spec:
  egress: not-a-policy
---
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
---
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: deny-all-out
spec:
  podSelector: {}
  policyTypes:
    - Egress
---
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: anything-goes
spec:
  podSelector: {}
  egress:
    - {}
---
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: ingress-only
spec:
  podSelector: {}
  policyTypes: [Ingress]
";
    let rules = yaml("k8s/policies.yaml", text);
    // A Service's `egress` key and a policy for incoming traffic alone are not outbound rules.
    assert_eq!(rules.len(), 3, "{rules:#?}");
    assert_eq!(rules[0].line, 9);
    assert!(
        rules[0].what.contains("web-egress") && rules[0].what.contains("(1 rule)"),
        "{}",
        rules[0].what
    );
    assert!(!rules[0].open_to_all);
    assert!(
        rules[1].what.contains("deny-all-out") && rules[1].what.contains("connect nowhere"),
        "{}",
        rules[1].what
    );
    assert!(!rules[1].open_to_all);
    assert!(rules[2].what.contains("anything-goes"), "{}", rules[2].what);
    assert!(
        rules[2].open_to_all,
        "an egress rule with no `to` allows every destination"
    );
}

#[test]
fn a_compose_network_marked_internal_is_read_with_the_services_on_it() {
    let text = "\
services:
  app:
    networks: [backend]
  db:
    networks:
      backend: {}
  proxy:
    networks: [frontend, backend]
networks:
  backend:
    internal: true
  frontend: {}
";
    let rules = yaml("docker-compose.yml", text);
    assert_eq!(rules.len(), 1, "{rules:#?}");
    assert_eq!(rules[0].line, 11);
    assert!(
        rules[0].what.contains("backend, marked internal"),
        "{}",
        rules[0].what
    );
    assert!(
        rules[0].what.contains("app, db, proxy are on it"),
        "{}",
        rules[0].what
    );
    // The same file under another name is not read as compose.
    assert!(yaml("config/settings.yml", text).is_empty());
}

#[test]
fn terraform_outbound_rules_are_read_and_inbound_and_commented_ones_are_not() {
    let text = r#"
resource "aws_security_group" "web" {
  name = "web"
  ingress {
    from_port   = 443
    to_port     = 443
    cidr_blocks = ["0.0.0.0/0"]
  }
  egress {
    from_port   = 443
    to_port     = 443
    cidr_blocks = ["10.0.0.0/8"]
  }
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_security_group_rule" "in" {
  type        = "ingress"
  cidr_blocks = ["0.0.0.0/0"]
}

resource "aws_security_group_rule" "out" {
  type        = "egress"
  cidr_blocks = ["192.168.0.0/16"]
}

resource "google_compute_firewall" "deny_out" {
  direction          = "EGRESS"
  destination_ranges = ["0.0.0.0/0"]
  deny { protocol = "all" }
}

resource "azurerm_network_security_rule" "out" {
  direction                  = "Outbound"
  access                     = "Allow"
  destination_address_prefix = "*"
}

resource "azurerm_network_security_rule" "shut" {
  direction                  = "Outbound"
  access                     = "Deny"
  destination_address_prefix = "*"
}

# resource "aws_vpc_security_group_egress_rule" "old" {
#   cidr_ipv4 = "0.0.0.0/0"
# }

/*
resource "aws_vpc_security_group_egress_rule" "older" {
}
*/

resource "aws_vpc_security_group_egress_rule" "api" {
  cidr_ipv4 = "203.0.113.0/24"
}
"#;
    let rules = terraform("infra/main.tf", text);
    let said: Vec<(usize, &str, bool)> = rules
        .iter()
        .map(|r| (r.line, r.what.as_str(), r.open_to_all))
        .collect();
    assert_eq!(rules.len(), 7, "{said:#?}");
    assert_eq!(
        (rules[0].line, rules[0].open_to_all),
        (9, false),
        "{said:#?}"
    );
    assert_eq!(
        (rules[1].line, rules[1].open_to_all),
        (14, true),
        "{said:#?}"
    );
    assert!(
        rules[0].what.contains("aws_security_group.web"),
        "{}",
        rules[0].what
    );
    assert!(
        rules[2].what.contains("aws_security_group_rule.out") && !rules[2].open_to_all,
        "{said:#?}"
    );
    // A rule that refuses everywhere shuts the app in: it is never one that lets it reach everywhere.
    assert!(
        rules[3]
            .what
            .contains("google_compute_firewall.deny_out, refusing outgoing"),
        "{said:#?}"
    );
    assert!(!rules[3].open_to_all, "{said:#?}");
    assert!(
        rules[4].what.contains("azurerm_network_security_rule.out") && rules[4].open_to_all,
        "{said:#?}"
    );
    assert!(
        rules[5]
            .what
            .contains("azurerm_network_security_rule.shut, refusing")
            && !rules[5].open_to_all,
        "{said:#?}"
    );
    assert!(
        rules[6]
            .what
            .contains("aws_vpc_security_group_egress_rule.api")
            && !rules[6].open_to_all,
        "{said:#?}"
    );
    // Neither the inbound rule nor the commented-out ones are read.
    assert!(
        !rules.iter().any(|r| r.what.contains(".in,")
            || r.what.contains(".old")
            || r.what.contains(".older"))
    );
}

#[test]
fn a_folder_s_rules_are_read_in_order_and_test_code_is_left_out() {
    let dir = std::env::temp_dir().join(format!("sv-outbound-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(dir.join("infra")).unwrap();
    std::fs::create_dir_all(dir.join("tests")).unwrap();
    std::fs::write(
        dir.join("infra/main.tf"),
        "resource \"aws_vpc_security_group_egress_rule\" \"api\" {\n  cidr_ipv4 = \"203.0.113.0/24\"\n}\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("tests/fixture.tf"),
        "resource \"aws_vpc_security_group_egress_rule\" \"t\" {\n}\n",
    )
    .unwrap();
    std::fs::write(dir.join("app.py"), "print('hi')\n").unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    let found = read(&listing);
    std::fs::remove_dir_all(&dir).ok();
    // The setup: the walk found both files.
    assert!(
        listing
            .files
            .iter()
            .any(|e| e.relative == "tests/fixture.tf"),
        "{:?}",
        listing.files
    );
    assert_eq!(found.rules.len(), 1, "{found:#?}");
    assert_eq!(found.rules[0].file, "infra/main.tf");
    assert_eq!(found.more, 0);
}
