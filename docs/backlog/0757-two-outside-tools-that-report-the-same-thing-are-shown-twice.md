# Two outside tools that report the same thing are shown twice by cv check, unless cv has a rule of its own for it

**Status:** open

Filed 24 November 2026 by the cato-pipeline session, at the owner's word. That session reads `sv`'s `report.json`
in a continuous-authorization pipeline, and turns it into OSCAL and a plan of action. Seen with `sv` and `cv` built
from 06f5032.

**What was seen.** Over two Dockerfiles with no `HEALTHCHECK`, `cv check` listed four findings for two facts:

    Ensure that HEALTHCHECK instructions have been added to container images (checkov.CKV_DOCKER_2) at Dockerfile line 1
    No HEALTHCHECK defined (trivy.DS-0026) at Dockerfile line 1

and the same pair for the second file. Each tool's line said "0 of them also cv's own". `crates/cv-cli/src/tools.rs`
folds tools' findings together only where `cv` has a rule of its own: its table `AGREES` has one entry,
`cv.container-root`, with hadolint DL3002, Checkov CKV_DOCKER_8 and CKV_K8S_23, and Trivy DS-0002 and KSV-0012.

**What it means.** ADR-157, part 3, chose "a tool's finding that ConfigVet's own rule also made shown as one
finding". Two tools agreeing with each other, where `cv` has no rule, was not decided, and is the commoner case:
`cv` has four rules, and the three tools have hundreds between them. A person reads the same thing twice, and a
pipeline opens two items for one fix.

**To do.** A second table, of rules that two or three tools share with no rule of `cv`'s: one finding, each tool
named in `also_reported_by`, as `sv` does for its own outside tools. Start with the pairs seen over a real app,
each added only after reading both rules' text, as the rule maps were. A pair not in the table stays two findings.
