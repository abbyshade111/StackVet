# Outside services the code names and the run saw, beside the communication question, and an undeclared one reported

**Status:** claimed by claude/backlog-work, 22 October 2026

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V13 Configuration (V13.1.1, V13.2.5)**

- **V13.1.1** (L2), all communication needs documented, outside services included. Notes question exists. Route 1
  and 4 together: the outside addresses the code names (`https://` literals outside tests, SDK packages for outside
  services from the bill of materials: stripe, sendgrid, twilio, openai, `aws-sdk`, `@supabase`), and what the fenced
  run saw the app try to reach (the fence has no gateway, and the run already lists the sites the pages tried to
  reach, 0147). Shown beside the notes question as "StackVet saw the app name these six outside services". Finding-only:
  an address the run saw the app try that neither `[stack] external-apis` nor the notes names (the manifest list is
  structured; the notes are free text, so compare with the manifest only), `probe.undeclared-outside-service` citing
  V13.1.1. Tier: *documented by the owner*; `sv`'s own for the finding. Same direction as partial-checks; the fence
  observation as the source is the addition.
- **V13.2.5** (L2), the server configured with an allowlist of destinations it may reach. No question today. Route 1
  where the folder holds the infrastructure (the `iac` corroborator already finds Terraform, Kubernetes, compose files):
  a Kubernetes `NetworkPolicy` with `egress`, a security group with outbound rules, a compose network marked
  `internal: true`, shown as the pointer; no credit, since the file in the folder is not known to be what runs. Where
  there is none, the honest route is 5: a hand check at the hosting platform ("does your host let the app connect
  anywhere?"), with the reason "the server's outbound rules are set where it runs, not in the folder". The fence
  itself cannot help: it has no gateway by design, so the app reaching nothing shows the fence, not the app. Tier:
  *checked by hand*.

**Checked against `main`, 20 October 2026** (the backlog clean-up): V13.1.1 is all built (813724f0; ADR-123). Only V13.2.5's pointer to outbound rules in infrastructure files is left, which needs a reader for YAML and Terraform; its hand check (0506, 14 October) gives the reason meanwhile, so "No question today" above is out of date.

**Claimed for the part left, 22 October 2026** (the owner's order: 0496 and the YAML and Terraform reader after 0623).
V13.1.1 is built; this claim is V13.2.5's pointer alone.

**Proposed, to be recorded as a Later entry on ADR-022** (which governs the readers that show a question where to look,
as `route_guards.rs` does for V8.1.1):

- **What is read** (`crates/sv-check/src/outbound_rules.rs`, new), outside test code and the folders the static scan
  leaves out:
  - Kubernetes YAML: a `kind: NetworkPolicy` whose `policyTypes` holds `Egress` or that has an `egress:` list, with
    whether the list is empty (nothing may go out) or names destinations, read with the `tree-sitter-yaml` grammar the
    workflow reader already uses.
  - Compose files: a network marked `internal: true`, and which services sit on it.
  - Terraform (`.tf`), read line by line, since no HCL grammar is in the build and adding one is a dependency of its
    own: `egress` blocks in an `aws_security_group`, an `aws_security_group_rule` or
    `aws_vpc_security_group_egress_rule` of type egress, a `google_compute_firewall` with `direction = "EGRESS"`, and
    an `azurerm_network_security_rule` with `direction = "Outbound"`; each with its file and line, and whether it
    allows everywhere (`0.0.0.0/0`, `::/0`, `*`).
- **It finds and does not judge:** no finding, no credit, and nothing that credits or fails V13.2.5 reads it. One fixed
  sentence says it is where to look, not evidence: a file in the folder is not known to be what runs, and the rules
  that count are the ones set where the app is hosted, which the hand check still asks about.
- **Where it is shown:** beside the V13.2.5 hand check in `sv questions`, the MCP `questions` section, and report.html,
  as the route table is; not in compliance.md, `report.json`, or the MCP structured result. With nothing found,
  nothing is shown, and the hand check reads as today.
- **Held by** tests of each kind of file, of what is not an outbound rule (an `ingress` block, an `egress` key
  elsewhere in a YAML file, a commented-out Terraform block), and that V13.2.5 stays not verified with the pointer
  shown; each red with its part undone.
