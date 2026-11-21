# Information: Notes with users

Produced by `sv` 0.2.0 (commit 7031bd26f038) on 2026-11-21 15:48:12 UTC, run 15a3dae229ac.

**This file:** What asks nothing of you: findings in test code, checks that found nothing, your AI coding tool's files, and the text of each requirement, from compliance.md and security.md. **For:** You, when you want the rest. The end of `report.html` lists every file in this folder and who each is for.

This page is one part of the report, cut whole from it: it credits nothing on its own, and the counts stay in `compliance.md`. Like every file in this folder it is sealed with the report, so `sv` can tell whether it is the page it wrote.

## Worth knowing: 1 note

Not things to fix. Each says something about how the app is built that is worth knowing, and asks for nothing unless you decide it matters; if you do, its "What to do" says how.

### [info] 138 commit(s) on the default branch were made by an AI coding tool and not merged in

**Where:** `.git` line 1

*How sure: confirmed.*

*Fingerprint: `4213cb48a86e8e56`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

These commits name an AI coding tool and reached the branch directly: `1fe46225` Run the app's container read-only, without capabilities (#496) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `93bb7454` Claim running the app's container read-only (#493) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `53154bca` Backlog: say when a manifest and its lockfile disagree (#484) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `67a497af` Example app: PyJWT 2.15.1, and a lock that matches its manifest (#482) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `aeb056e1` Backlog: a prompt library from the CSA guide and lessons learned (#476) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `e91c7173` Backlog: record what the sessions agreed about the promotion (#221) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `8ced5fb1` Backlog: v1-paper is tagged and released (#216) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `76156b3c` Backlog: promote sv to the top of the repository, keep v1 for the paper (#210) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `0acfe495` Backlog: what the owner's first build from scratch found, and a zip (#204) (Claude Opus 5.5 &lt;noreply@anthropic.com>); `ce032691` Backlog: packaging sv, a container now and a download later (#198) (Claude Opus 5.5 &lt;noreply@anthropic.com>); and 128 more. This check reads only what a commit says about itself, in its `Assisted-by:` or `Co-Authored-By:` line, so a commit that names no tool is not seen. Whether the hosting service's protection rules held is in its settings, which `sv` does not read.

**Why it matters.** A change that reached the branch without a merge reached it without another person's review.

**What to do.** Ask the AI coding tool to work on a branch and open a pull request, and merge it after a look. Turn on the pull-request rule for the default branch, so a direct push is refused.

## How the app is built with AI (OWASP AISVS Appendix C)

33 requirements of OWASP AISVS Appendix C (AI-Assisted Secure Coding) apply to how this app is built with an AI coding tool, rather than to the app itself, and nothing has checked them. They are listed here rather than in the counts above. 23 are what the rules given to your AI coding tool come from (`sv rules`, or `stackvet_guidance` from inside the tool); the rules are instructions, and following them is not evidence that they are met. 2 are your decisions, and are among your questions. Nothing in `sv` reaches the other 8: most are about a CI pipeline or an organization's AI tooling. 19 more wait on a question nobody has answered.

| requirement | what happens to it | what it asks for |
|---|---|---|
| AC.1.1 | given to your AI coding tool as a rule | Verify that a written workflow says when AI tools may generate, refactor, or review code. The workflow names the approved tools, the prohibited use cases, and the data classifications that are allowed as input. |
| AC.1.2 | given to your AI coding tool as a rule | Verify that the workflow covers every SSDLC phase from design and implementation through code review, testing, deployment, and post-deployment monitoring, and names the security gates that stay mandatory whether AI was involved or not. |
| AC.1.3 | nothing in `sv` reaches it | Verify that the workflow names the adversarial-AI threat scenarios it is built to mitigate. The list should cover prompt injection delivered through PR content, AI-generated supply-chain payloads, autonomous agents approving their own work, fork-PR secret exfiltration, and compromise of the model supply chain. |
| AC.2.1 | given to your AI coding tool as a rule | Verify that every AI tool, whether it is an assistant, a reviewer, an agent, or an MCP server, has a threat model. The threat model covers misuse, model inversion, training-data leakage, prompt injection from untrusted input, insecure output handling, excessive agency, and risk inherited from its dependency chain. |
| AC.2.2 | given to your AI coding tool as a rule | Verify that the evaluation of each tool covers the local components (static and dynamic analysis), the SaaS endpoints (TLS, AuthN/AuthZ, logging, data residency), and the vendor's model supply chain (training-data provenance, fine-tune history, RAG sources). Each of these is reviewed and the review is written down. |
| AC.2.3 | nothing in `sv` reaches it | Verify that each tool goes through adversarial robustness testing before onboarding. The testing is repeated after any material change to the model or to the system prompts. Coverage includes automated prompt-injection probes, jailbreak suites, and indirect-injection corpora delivered through realistic PR and issue surfaces. |
| AC.3.1 | given to your AI coding tool as a rule | Verify that written guidance forbids putting secrets, credentials, PII, or classified data in any prompt sent to an AI tool. The guidance is enforced in pre-commit hooks, IDE integrations, and CI. |
| AC.3.2 | given to your AI coding tool as a rule | Verify that technical controls automatically strip sensitive material from any context window sent to an AI tool. Client-side redaction, approved context filters, and secret scanners with pre-prompt hooks all qualify. |
| AC.3.3 | given to your AI coding tool as a rule | Verify that any externally sourced context being fed to an AI tool is treated as untrusted and screened for prompt injection before it reaches the prompt. Sources to cover: PR descriptions and comments, fork-supplied diffs, issue bodies, commit messages, third-party documentation, web search results, and MCP tool outputs. |
| AC.3.4 | given to your AI coding tool as a rule | Verify that the AI tool enforces an instruction hierarchy, with system and developer messages taking precedence over untrusted repository content. This hierarchy has to hold across multi-turn conversations and tool-augmented workflows. |
| AC.3.5 | given to your AI coding tool as a rule | Verify that input length controls stop untrusted PR or repository content from crowding system instructions or safety directives out of the effective context window. Oversized inputs are rejected outright. Silent truncation is not acceptable. |
| AC.4.1 | your decision, among your questions | Verify that AI-generated code always goes through code review by a qualified human engineer. The reviewer must not be the same identity that asked for the AI generation in the first place (separation of duties). And the AI agent itself does not count as the human reviewer. |
| AC.4.2 | given to your AI coding tool as a rule | Verify that automated security testing runs on every pull request containing AI-generated code: SAST, IAST, DAST, secret scanning, IaC scanning, and SCA. Where the scanner supports them, AI-attribution-aware rules are turned on. |
| AC.4.3 | given to your AI coding tool as a rule | Verify that pull requests containing AI-generated code are blocked from merging when an automated scan surfaces a critical security finding, defined as CVSS >= 9.0 or the equivalent threshold in the organization's vulnerability severity policy. Bypassing the block requires a written exception approved by an authorized human. |
| AC.4.4 | given to your AI coding tool as a rule | Verify that security-critical files require an elevated review threshold when AI generated or modified them: two-person review, security-team sign-off, or stricter. Security-critical files here include authentication, authorization, and cryptography code; IAM policy; CI/CD workflow definitions; deployment manifests; and sandbox or network policy artifacts. |
| AC.5.1 | nothing in `sv` reaches it | Verify that prompt-and-response pairs are logged with stable correlation identifiers, so that an investigator can later replay the whole chain: prompt → response → commit → build → deployment. |
| AC.6.3 | your decision, among your questions | Verify that scheduled red-team exercises target the AI tooling itself. The exercises include direct and indirect prompt-injection probes delivered through realistic PR, issue, and comment surfaces, jailbreak corpora, and supply-chain payload generation. Findings are remediated under tracked severity SLAs. |
| AC.7.1 | given to your AI coding tool as a rule | Verify that AI-generated or AI-modified artifacts are clearly labeled and tracked as such. Artifact classes in scope include infrastructure-as-code (Terraform, CloudFormation, Pulumi, Bicep), CI/CD workflow files (GitHub Actions, GitLab CI, Jenkinsfile, Argo Workflows, Tekton), container and orchestration manifests (Dockerfile, Kubernetes, Helm), and security policy artifacts (IAM, OPA/Rego, NetworkPolicy, admission controllers). |
| AC.7.2 | given to your AI coding tool as a rule | Verify that AI-generated infrastructure and pipeline configurations require human review and approval before they run in any environment beyond a hermetic sandbox. |
| AC.8.1 | given to your AI coding tool as a rule | Verify that autonomous agents cannot approve, merge, sign, or deploy artifacts that they themselves generated, and that this constraint is enforced by the source-control system, the CI system, and the artifact registry. Policy alone does not satisfy this control. |
| AC.8.2 | given to your AI coding tool as a rule | Verify that AI systems run with scoped, non-human identities (service accounts, workload identities, OIDC-issued ephemeral tokens), and that those identities cannot be used to promote their own generated artifacts across environments. |
| AC.10.1 | given to your AI coding tool as a rule | Verify that AI-generated artifacts carry the required origin and generation fields: model identity and version, tool or agent identity, generation context, prompt hash, human involvement, session identifiers, and correlation IDs. |
| AC.10.2 | nothing in `sv` reaches it | Verify that origin and generation metadata is checked for completeness and consistency: no missing or ambiguous fields, values normalized to a single representation, and a signature chain that validates back to a trusted root. |
| AC.11.1 | given to your AI coding tool as a rule | Verify that AI review and assistant bots treat every piece of PR-supplied content (diff, title, description, comments, file contents, commit messages, linked external URLs) as untrusted input, and apply the AISVS C2.1 prompt-injection defenses: instruction-hierarchy enforcement, content sanitization, and indirect-injection detection. |
| AC.11.2 | given to your AI coding tool as a rule | Verify that AI review and assistant bot system prompts and policy configurations are integrity-checked at load time (signed, hash-pinned), and that nothing in the repository, in branch contents, in PR-sourced environment variables, or in any other user-controllable input can modify them. |
| AC.11.3 | given to your AI coding tool as a rule | Verify that AI review and assistant bots emit only structured, schema-validated output (JSON with an allow-list of fields and actions). Any free-form output is treated as untrusted and never executed as a command, a query, a shell snippet, or a workflow step. |
| AC.11.4 | nothing in `sv` reaches it | Verify that AI review and assistant bots run in network-isolated, least-privilege sandboxes: a dedicated namespace, default-deny egress with an allow-list to approved APIs only, no mounted repository secrets, and ephemeral credentials only. |
| AC.11.5 | given to your AI coding tool as a rule | Verify that any privileged action a bot can take (approving a PR, merging, labeling, dismissing reviews, posting comments outside its sandbox, invoking external tools) goes through a separate, audited authorization path. That path is adjudicated by a policy engine, not by the LLM. |
| AC.11.6 | nothing in `sv` reaches it | Verify that AI review and assistant bots log all prompts (including externally sourced context), tool calls, and outputs to tamper-evident storage. Egress patterns (URLs, IPs, DNS, payload sizes) are continuously monitored for exfiltration indicators, with alerting tuned for webhook, paste-site, and bin-service destinations. |
| AC.14.1 | given to your AI coding tool as a rule | Verify that an incident-response playbook exists for AI-in-pipeline compromise. At minimum it covers: revoking AI-agent credentials, rotating every secret that touched the compromised workflow run, quarantining the compromised artifacts, notifying downstream consumers, notifying regulators where applicable, and preserving prompts, responses, and audit logs for forensics. |
| AC.14.2 | given to your AI coding tool as a rule | Verify that any secret that touched a workflow run associated with a suspicious PR, a prompt-injection event, or an AI-agent anomaly is automatically rotated, and that downstream issuers (cloud IAM, package registries, signing-key custodians) are notified of the rotation. |
| AC.14.3 | nothing in `sv` reaches it | Verify that AI agent identities (keys, tokens, OIDC trust grants) can be rapidly revoked and quarantined, with a target time-to-revoke that is written down and tested at least once a year. |
| AC.14.4 | nothing in `sv` reaches it | Verify that build provenance and AI BOM records are used during incident response to identify every downstream artifact produced under the suspect AI agent or the compromised pipeline run, so that recall, rebuild, or quarantine can be targeted. |

## What the AI coding tool did that is worth a look

Information only: nothing here changes a requirement's status, counts toward anything, or is a finding. Each line is something the AI coding tool did that a person may want to look at, with when and where it was seen. Many of them are right to do; a line says what happened, not that it was wrong.

Read: the last 20 commits, as there is no earlier report made at a known commit, of which 20 were made by an AI coding tool, by a trailer in its message that names one; the 0 that name none are not read as the tool's.

Not looked for: the commands the tool ran and the files it wrote without committing them, since the hook record was not read for this run (`sv report --hook-record` reads it, once its hook is installed).

Not looked for: findings that went away, since this folder held no earlier report that sv can show it wrote.

| Kind | What was seen | Where |
|---|---|---|
| Tests skipped or removed | a test was marked to be skipped in tools/session_token.py (xit() | commit c0f03aea "Backlog 0692: sessions push and open pull requests as the se", by Claude Opus 5.5 &lt;noreply@anthropic.com> |
| Tests skipped or removed | a line the test crates/sv-cli/tests/website.rs expects was changed in the same commit as 1 file of code | commit 7031bd26 "stackvet.dev: fix what the whole-site review found, the page", by Claude Opus 5.5 &lt;noreply@anthropic.com> |
| A guard edited | a marker that tells a check to look away (noqa) was added in tools/session_token.py | commit c0f03aea "Backlog 0692: sessions push and open pull requests as the se", by Claude Opus 5.5 &lt;noreply@anthropic.com> |
| A guard edited | a marker that tells a check to look away (nosec) was added in docs/backlog/0693-configvet-part-3-cv-runs-checkov-trivy-and-hadolint-as.md | commit 6f1a9b40 "File and claim backlog 0693: ConfigVet part 3, the outside t", by Claude Opus 5.5 &lt;noreply@anthropic.com> |
| A guard edited | a marker that tells a check to look away (nosec) was added in docs/backlog/done/0693-configvet-part-3-cv-runs-checkov-trivy-and-hadolint-as.md | commit 9ca59c36 "ConfigVet part 3: cv runs hadolint, Checkov, and Trivy (back", by Claude Opus 5.5 &lt;noreply@anthropic.com> |
| A guard edited | .github/workflows/release.yml was changed | commit cc3f49db "The release builds, checks, signs, and attaches cv's own Win", by Claude Opus 5.5 &lt;noreply@anthropic.com> |

## Checks that ran and found nothing, against nothing in the tables above

These were satisfied. They appear here rather than beside a requirement because what they look at is not something this app is being assessed on.

| check | what it covered | why it is here |
|---|---|---|
| config.mcp-server-unpinned | 3 files: no MCP server started with npx, uvx, or docker from a package or image named in them | it names no requirement in any loaded framework |
| config.certificate-checks-off | 3 files: none sets `NODE_TLS_REJECT_UNAUTHORIZED` or `PYTHONHTTPSVERIFY` to `0` or switches certificate checking off for the whole app in code; the same setting made on the server itself is in no file | it names no requirement in any loaded framework |
| config.rich-text-without-sanitizer | 0 packages: none is a rich-text editor `sv` knows | it names no requirement in any loaded framework |
| config.retired-grant-enabled | no sign-in server library `sv` knows (django-oauth-toolkit, Doorkeeper, fosite, node-oauth2-server, league/oauth2-server, Laravel Passport) is used in the app's files | it names no requirement in any loaded framework |
| config.model-file-can-run-code | 0 file(s) named like a model file, none holding a pickle; 2 other file(s) that are not code, none opening as a pickle; a model downloaded when the app runs is not seen | it names no requirement in any loaded framework |

<details>
<summary><strong>Requirements that do not apply, and why</strong> — reference: why each was ruled out</summary>

An exclusion resting on the manifest's word is weaker than one resting on what the code actually contains. The last column says which this is.

| requirement | why not | rests on |
|---|---|---|
| V1.2.6 | No LDAP client library is used, so LDAP injection is not possible. | what the app's code contains (ldap) |
| V1.2.7 | No XPath library is used. | what the app's code contains (xpath) |
| V1.2.8 | No LaTeX processing library is used. | what the app's code contains (latex) |
| V1.3.8 | JNDI is a Java technology and this app does not run on the JVM. | what the app's code contains (jndi) |
| V1.3.9 | No memcache client library is used. | what the app's code contains (memcache) |
| V1.4.1 | This app's languages are memory-safe and it contains no unmanaged (C/C++) code. | what the app's code contains (unmanaged-code) |
| V1.4.2 | This app's languages are memory-safe and it contains no unmanaged (C/C++) code. | what the app's code contains (unmanaged-code) |
| V1.4.3 | This app's languages are memory-safe and it contains no unmanaged (C/C++) code. | what the app's code contains (unmanaged-code) |
| V1.5.1 | No XML parser is used, so XML external entity (XXE) attacks are not possible. | what the app's code contains (xml) |
| V12.1.3 | The app asks for no client certificate, so there is no certificate identity to validate before it is used. | what the app's code contains (mtls) |
| V3.5.5 | This app's pages do not use the browser postMessage feature. | what the app's code contains (postmessage) |
| V4.3.1 | No GraphQL library is used. | what the app's code contains (graphql) |
| V4.3.2 | No GraphQL library is used. | what the app's code contains (graphql) |
| V4.4.1 | No WebSocket library is used; every request is a normal web request. | what the app's code contains (websockets) |
| V4.4.2 | No WebSocket library is used; every request is a normal web request. | what the app's code contains (websockets) |
| V4.4.3 | No WebSocket library is used; every request is a normal web request. | what the app's code contains (websockets) |
| V4.4.4 | No WebSocket library is used; every request is a normal web request. | what the app's code contains (websockets) |
| V6.8.3 | No SAML library is used, so there are no SAML assertions to replay. | what the app's code contains (saml) |


</details>

## What the app says about itself

| about | stackvet.toml | who said it | the code | what that means |
|---|---|---|---|---|
| auth | yes | not recorded, so read as your AI coding tool's | nothing | Taken on the manifest's word. `sv` looked and found nothing, which for this is not the same as finding it absent. |
| oauth | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| authorization-server | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| mcp-server | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| jwt | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| uploads | yes | not recorded, so read as your AI coding tool's | nothing | Taken on the manifest's word. `sv` looked and found nothing, which for this is not the same as finding it absent. |
| payments | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| email | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| public-api | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| scheduler | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| multi-tenant | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| multiple-services | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| webrtc | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| external-apis | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| tls | yes | not recorded, so read as your AI coding tool's | nothing | Taken on the manifest's word. `sv` looked and found nothing, which for this is not the same as finding it absent. |
| internet | yes | not recorded, so read as your AI coding tool's | yes | The manifest and the code agree. |
| ai | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| ai-actions | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| ai-history | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| ai-moderation | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| rag | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| web-search | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| generates-media | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| mcp | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| training | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| level2 | yes | not recorded, so read as your AI coding tool's | nothing | Taken on the manifest's word. `sv` looked and found nothing, which for this is not the same as finding it absent. |
| self-hosted-model | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| multi-agent | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| multimodal-ai | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| out-of-band-auth | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| out-of-band-or-email-code | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| shared-hostname | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| rich-text | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| ci-cd | nothing | nobody | no | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| hosted-scm | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| outside-contributors | nothing | nobody | nothing | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| iac | nothing | nobody | no | Nobody has said. Requirements that turn on this are not assessed rather than excluded. |
| websockets | nothing | nobody | no | The manifest and the code agree. |
| graphql | nothing | nobody | no | The manifest and the code agree. |
| ldap | nothing | nobody | no | The manifest and the code agree. |
| xpath | nothing | nobody | no | The manifest and the code agree. |
| xml | nothing | nobody | no | The manifest and the code agree. |
| mtls | nothing | nobody | no | The manifest and the code agree. |
| latex | nothing | nobody | no | The manifest and the code agree. |
| jndi | nothing | nobody | no | The manifest and the code agree. |
| memcache | nothing | nobody | no | The manifest and the code agree. |
| format-strings | nothing | nobody | yes | The manifest and the code agree. |
| unmanaged-code | nothing | nobody | no | The manifest and the code agree. |
| postmessage | nothing | nobody | no | The manifest and the code agree. |
| saml | nothing | nobody | no | The manifest and the code agree. |
| cryptographic-authenticator | nothing | nobody | no | The manifest and the code agree. |

## Appendix: what each requirement asks for

The words of the standard, for every requirement that applies to this app, in the order of the chapters above.

| requirement | what it asks for |
|---|---|
| V1.2.1 | Verify that output encoding for an HTTP response, HTML document, or XML document is relevant for the context required, such as encoding the relevant characters for HTML elements, HTML attributes, HTML comments, CSS, or HTTP header fields, to avoid changing the message or document structure. |
| V1.2.2 | Verify that when dynamically building URLs, untrusted data is encoded according to its context (e.g., URL encoding or base64url encoding for query or path parameters). Ensure that only safe URL protocols are permitted (e.g., disallow javascript: or data:). |
| V1.2.3 | Verify that output encoding or escaping is used when dynamically building JavaScript content (including JSON), to avoid changing the message or document structure (to avoid JavaScript and JSON injection). |
| V1.2.4 | Verify that data selection or database queries (e.g., SQL, HQL, NoSQL, Cypher) use parameterized queries, ORMs, entity frameworks, or are otherwise protected from SQL Injection and other database injection attacks. This is also relevant when writing stored procedures. |
| V1.2.5 | Verify that the application protects against OS command injection and that operating system calls use parameterized OS queries or use contextual command line output encoding. |
| V1.3.2 | Verify that the application avoids the use of eval() or other dynamic code execution features such as Spring Expression Language (SpEL). Where there is no alternative, any user input being included must be sanitized before being executed. |
| V1.1.1 | Verify that input is decoded or unescaped into a canonical form only once, it is only decoded when encoded data in that form is expected, and that this is done before processing the input further, for example it is not performed after input validation or sanitization. |
| V1.1.2 | Verify that the application performs output encoding and escaping either as a final step before being used by the interpreter for which it is intended or by the interpreter itself. |
| V1.2.9 | Verify that the application escapes special characters in regular expressions (typically using a backslash) to prevent them from being misinterpreted as metacharacters. |
| V1.3.3 | Verify that data being passed to a potentially dangerous context is sanitized beforehand to enforce safety measures, such as only allowing characters which are safe for this context and trimming input which is too long. |
| V1.3.4 | Verify that user-supplied Scalable Vector Graphics (SVG) scriptable content is validated or sanitized to contain only tags and attributes (such as draw graphics) that are safe for the application, e.g., do not contain scripts and foreignObject. |
| V1.3.6 | Verify that the application protects against Server-side Request Forgery (SSRF) attacks, by validating untrusted data against an allowlist of protocols, domains, paths and ports and sanitizing potentially dangerous characters before using the data to call another service. |
| V1.3.7 | Verify that the application protects against template injection attacks by not allowing templates to be built based on untrusted input. Where there is no alternative, any untrusted input being included dynamically during template creation must be sanitized or strictly validated. |
| V1.3.10 | Verify that format strings which might resolve in an unexpected or malicious way when used are sanitized before being processed. |
| V1.5.2 | Verify that deserialization of untrusted data enforces safe input handling, such as using an allowlist of object types or restricting client-defined object types, to prevent deserialization attacks. Deserialization mechanisms that are explicitly defined as insecure must not be used with untrusted input. |
| V2.1.1 | Verify that the application's documentation defines input validation rules for how to check the validity of data items against an expected structure. This could be common data formats such as credit card numbers, email addresses, telephone numbers, or it could be an internal data format. |
| V2.2.1 | Verify that input is validated to enforce business or functional expectations for that input. This should either use positive validation against an allow list of values, patterns, and ranges, or be based on comparing the input to an expected structure and logical limits according to predefined rules. For L1, this can focus on input which is used to make specific business or security decisions. For L2 and up, this should apply to all input. |
| V2.2.2 | Verify that the application is designed to enforce input validation at a trusted service layer. While client-side validation improves usability and should be encouraged, it must not be relied upon as a security control. |
| V2.3.1 | Verify that the application will only process business logic flows for the same user in the expected sequential step order and without skipping steps. |
| V2.1.2 | Verify that the application's documentation defines how to validate the logical and contextual consistency of combined data items, such as checking that suburb and ZIP code match. |
| V2.1.3 | Verify that expectations for business logic limits and validations are documented, including both per-user and globally across the application. |
| V2.2.3 | Verify that the application ensures that combinations of related data items are reasonable according to the pre-defined rules. |
| V2.3.2 | Verify that business logic limits are implemented per the application's documentation to avoid business logic flaws being exploited. |
| V2.3.3 | Verify that transactions are being used at the business logic level such that either a business logic operation succeeds in its entirety or it is rolled back to the previous correct state. |
| V2.3.4 | Verify that business logic level locking mechanisms are used to ensure that limited quantity resources (such as theater seats or delivery slots) cannot be double-booked by manipulating the application's logic. |
| V2.4.1 | Verify that anti-automation controls are in place to protect against excessive calls to application functions that could lead to data exfiltration, garbage-data creation, quota exhaustion, rate-limit breaches, denial-of-service, or overuse of costly resources. |
| V3.2.1 | Verify that security controls are in place to prevent browsers from rendering content or functionality in HTTP responses in an incorrect context (e.g., when an API, a user-uploaded file or other resource is requested directly). Possible controls could include: not serving the content unless HTTP request header fields (such as Sec-Fetch-\\*) indicate it is the correct context, using the sandbox directive of the Content-Security-Policy header field or using the attachment disposition type in the Content-Disposition header field. |
| V3.2.2 | Verify that content intended to be displayed as text, rather than rendered as HTML, is handled using safe rendering functions (such as createTextNode or textContent) to prevent unintended execution of content such as HTML or JavaScript. |
| V3.3.1 | Verify that cookies have the 'Secure' attribute set, and if the '\\__Host-' prefix is not used for the cookie name, the '__Secure-' prefix must be used for the cookie name. |
| V3.4.1 | Verify that a Strict-Transport-Security header field is included on all responses to enforce an HTTP Strict Transport Security (HSTS) policy. A maximum age of at least 1 year must be defined, and for L2 and up, the policy must apply to all subdomains as well. |
| V3.4.2 | Verify that the Cross-Origin Resource Sharing (CORS) Access-Control-Allow-Origin header field is a fixed value by the application, or if the Origin HTTP request header field value is used, it is validated against an allowlist of trusted origins. When 'Access-Control-Allow-Origin: *' needs to be used, verify that the response does not include any sensitive information. |
| V3.5.1 | Verify that, if the application does not rely on the CORS preflight mechanism to prevent disallowed cross-origin requests to use sensitive functionality, these requests are validated to ensure they originate from the application itself. This may be done by using and validating anti-forgery tokens or requiring extra HTTP header fields that are not CORS-safelisted request-header fields. This is to defend against browser-based request forgery attacks, commonly known as cross-site request forgery (CSRF). |
| V3.5.2 | Verify that, if the application relies on the CORS preflight mechanism to prevent disallowed cross-origin use of sensitive functionality, it is not possible to call the functionality with a request which does not trigger a CORS-preflight request. This may require checking the values of the 'Origin' and 'Content-Type' request header fields or using an extra header field that is not a CORS-safelisted header-field. |
| V3.5.3 | Verify that HTTP requests to sensitive functionality use appropriate HTTP methods such as POST, PUT, PATCH, or DELETE, and not methods defined by the HTTP specification as "safe" such as HEAD, OPTIONS, or GET. Alternatively, strict validation of the Sec-Fetch-* request header fields can be used to ensure that the request did not originate from an inappropriate cross-origin call, a navigation request, or a resource load (such as an image source) where this is not expected. |
| V3.3.2 | Verify that each cookie's 'SameSite' attribute value is set according to the purpose of the cookie, to limit exposure to user interface redress attacks and browser-based request forgery attacks, commonly known as cross-site request forgery (CSRF). |
| V3.3.3 | Verify that cookies have the '__Host-' prefix for the cookie name unless they are explicitly designed to be shared with other hosts. |
| V3.3.4 | Verify that if the value of a cookie is not meant to be accessible to client-side scripts (such as a session token), the cookie must have the 'HttpOnly' attribute set and the same value (e. g. session token) must only be transferred to the client via the 'Set-Cookie' header field. |
| V3.4.3 | Verify that HTTP responses include a Content-Security-Policy response header field which defines directives to ensure the browser only loads and executes trusted content or resources, in order to limit execution of malicious JavaScript. As a minimum, a global policy must be used which includes the directives object-src 'none' and base-uri 'none' and defines either an allowlist or uses nonces or hashes. For an L3 application, a per-response policy with nonces or hashes must be defined. |
| V3.4.4 | Verify that all HTTP responses contain an 'X-Content-Type-Options: nosniff' header field. This instructs browsers not to use content sniffing and MIME type guessing for the given response, and to require the response's Content-Type header field value to match the destination resource. For example, the response to a request for a style is only accepted if the response's Content-Type is 'text/css'. This also enables the use of the Cross-Origin Read Blocking (CORB) functionality by the browser. |
| V3.4.5 | Verify that the application sets a referrer policy to prevent leakage of technically sensitive data to third-party services via the 'Referer' HTTP request header field. This can be done using the Referrer-Policy HTTP response header field or via HTML element attributes. Sensitive data could include path and query data in the URL, and for internal non-public applications also the hostname. |
| V3.4.6 | Verify that the web application uses the frame-ancestors directive of the Content-Security-Policy header field for every HTTP response to ensure that it cannot be embedded by default and that embedding of specific resources is allowed only when necessary. Note that the X-Frame-Options header field, although supported by browsers, is obsolete and may not be relied upon. |
| V3.7.1 | Verify that the application only uses client-side technologies which are still supported and considered secure. Examples of technologies which do not meet this requirement include NSAPI plugins, Flash, Shockwave, ActiveX, Silverlight, NACL, or client-side Java applets. |
| V3.7.2 | Verify that the application will only automatically redirect the user to a different hostname or domain (which is not controlled by the application) where the destination appears on an allowlist. |
| V4.1.1 | Verify that every HTTP response with a message body contains a Content-Type header field that matches the actual content of the response, including the charset parameter to specify safe character encoding (e.g., UTF-8, ISO-8859-1) according to IANA Media Types, such as "text/", "/+xml" and "/xml". |
| V4.1.2 | Verify that only user-facing endpoints (intended for manual web-browser access) automatically redirect from HTTP to HTTPS, while other services or endpoints do not implement transparent redirects. This is to avoid a situation where a client is erroneously sending unencrypted HTTP requests, but since the requests are being automatically redirected to HTTPS, the leakage of sensitive data goes undiscovered. |
| V4.1.3 | Verify that any HTTP header field used by the application and set by an intermediary layer, such as a load balancer, a web proxy, or a backend-for-frontend service, cannot be overridden by the end-user. Example headers might include X-Real-IP, X-Forwarded-*, or X-User-ID. |
| V4.2.1 | Verify that all application components (including load balancers, firewalls, and application servers) determine boundaries of incoming HTTP messages using the appropriate mechanism for the HTTP version to prevent HTTP request smuggling. In HTTP/1.x, if a Transfer-Encoding header field is present, the Content-Length header must be ignored per RFC 2616. When using HTTP/2 or HTTP/3, if a Content-Length header field is present, the receiver must ensure that it is consistent with the length of the DATA frames. |
| V5.2.1 | Verify that the application will only accept files of a size which it can process without causing a loss of performance or a denial of service attack. |
| V5.2.2 | Verify that when the application accepts a file, either on its own or within an archive such as a zip file, it checks if the file extension matches an expected file extension and validates that the contents correspond to the type represented by the extension. This includes, but is not limited to, checking the initial 'magic bytes', performing image re-writing, and using specialized libraries for file content validation. For L1, this can focus just on files which are used to make specific business or security decisions. For L2 and up, this must apply to all files being accepted. |
| V5.3.1 | Verify that files uploaded or generated by untrusted input and stored in a public folder, are not executed as server-side program code when accessed directly with an HTTP request. |
| V5.3.2 | Verify that when the application creates file paths for file operations, instead of user-submitted filenames, it uses internally generated or trusted data, or if user-submitted filenames or file metadata must be used, strict validation and sanitization must be applied. This is to protect against path traversal, local or remote file inclusion (LFI, RFI), and server-side request forgery (SSRF) attacks. |
| V5.1.1 | Verify that the documentation defines the permitted file types, expected file extensions, and maximum size (including unpacked size) for each upload feature. Additionally, ensure that the documentation specifies how files are made safe for end-users to download and process, such as how the application behaves when a malicious file is detected. |
| V5.2.3 | Verify that the application checks compressed files (e.g., zip, gz, docx, odt) against maximum allowed uncompressed size and against maximum number of files before uncompressing the file. |
| V5.4.1 | Verify that the application validates or ignores user-submitted filenames, including in a JSON, JSONP, or URL parameter and specifies a filename in the Content-Disposition header field in the response. |
| V5.4.2 | Verify that file names served (e.g., in HTTP response header fields or email attachments) are encoded or sanitized (e.g., following RFC 6266) to preserve document structure and prevent injection attacks. |
| V5.4.3 | Verify that files obtained from untrusted sources are scanned by antivirus scanners to prevent serving of known malicious content. |
| V6.1.1 | Verify that application documentation defines how controls such as rate limiting, anti-automation, and adaptive response, are used to defend against attacks such as credential stuffing and password brute force. The documentation must make clear how these controls are configured and prevent malicious account lockout. |
| V6.2.1 | Verify that user set passwords are at least 8 characters in length although a minimum of 15 characters is strongly recommended. |
| V6.2.2 | Verify that users can change their password. |
| V6.2.3 | Verify that password change functionality requires the user's current and new password. |
| V6.2.4 | Verify that passwords submitted during account registration or password change are checked against an available set of, at least, the top 3000 passwords which match the application's password policy, e.g. minimum length. |
| V6.2.5 | Verify that passwords of any composition can be used, without rules limiting the type of characters permitted. There must be no requirement for a minimum number of upper or lower case characters, numbers, or special characters. |
| V6.2.6 | Verify that password input fields use type=password to mask the entry. Applications may allow the user to temporarily view the entire masked password, or the last typed character of the password. |
| V6.2.7 | Verify that "paste" functionality, browser password helpers, and external password managers are permitted. |
| V6.2.8 | Verify that the application verifies the user's password exactly as received from the user, without any modifications such as truncation or case transformation. |
| V6.3.1 | Verify that controls to prevent attacks such as credential stuffing and password brute force are implemented according to the application's security documentation. |
| V6.3.2 | Verify that default user accounts (e.g., "root", "admin", or "sa") are not present in the application or are disabled. |
| V6.4.1 | Verify that system generated initial passwords or activation codes are securely randomly generated, follow the existing password policy, and expire after a short period of time or after they are initially used. These initial secrets must not be permitted to become the long term password. |
| V6.4.2 | Verify that password hints or knowledge-based authentication (so-called "secret questions") are not present. |
| V6.1.2 | Verify that a list of context-specific words is documented in order to prevent their use in passwords. The list could include permutations of organization names, product names, system identifiers, project codenames, department or role names, and similar. |
| V6.1.3 | Verify that, if the application includes multiple authentication pathways, these are all documented together with the security controls and authentication strength which must be consistently enforced across them. |
| V6.2.9 | Verify that passwords of at least 64 characters are permitted. |
| V6.2.10 | Verify that a user's password stays valid until it is discovered to be compromised or the user rotates it. The application must not require periodic credential rotation. |
| V6.2.11 | Verify that the documented list of context specific words is used to prevent easy to guess passwords being created. |
| V6.2.12 | Verify that passwords submitted during account registration or password changes are checked against a set of breached passwords. |
| V6.3.3 | Verify that either a multi-factor authentication mechanism or a combination of single-factor authentication mechanisms, must be used in order to access the application. For L3, one of the factors must be a hardware-based authentication mechanism which provides compromise and impersonation resistance against phishing attacks while verifying the intent to authenticate by requiring a user-initiated action (such as a button press on a FIDO hardware key or a mobile phone). Relaxing any of the considerations in this requirement requires a fully documented rationale and a comprehensive set of mitigating controls. |
| V6.3.4 | Verify that, if the application includes multiple authentication pathways, there are no undocumented pathways and that security controls and authentication strength are enforced consistently. |
| V6.4.3 | Verify that a secure process for resetting a forgotten password is implemented, that does not bypass any enabled multi-factor authentication mechanisms. |
| V6.4.4 | Verify that if a multi-factor authentication factor is lost, evidence of identity proofing is performed at the same level as during enrollment. |
| V6.5.1 | Verify that lookup secrets, out-of-band authentication requests or codes, and time-based one-time passwords (TOTPs) are only successfully usable once. |
| V6.5.2 | Verify that, when being stored in the application's backend, lookup secrets with less than 112 bits of entropy (19 random alphanumeric characters or 34 random digits) are hashed with an approved password storage hashing algorithm that incorporates a 32-bit random salt. A standard hash function can be used if the secret has 112 bits of entropy or more. |
| V6.5.3 | Verify that lookup secrets, out-of-band authentication code, and time-based one-time password seeds, are generated using a Cryptographically Secure Pseudorandom Number Generator (CSPRNG) to avoid predictable values. |
| V6.5.4 | Verify that lookup secrets and out-of-band authentication codes have a minimum of 20 bits of entropy (typically 4 random alphanumeric characters or 6 random digits is sufficient). |
| V6.5.5 | Verify that out-of-band authentication requests, codes, or tokens, as well as time-based one-time passwords (TOTPs) have a defined lifetime. Out of band requests must have a maximum lifetime of 10 minutes and for TOTP a maximum lifetime of 30 seconds. |
| V6.6.3 | Verify that a code based out-of-band authentication mechanism is protected against brute force attacks by using rate limiting. Consider also using a code with at least 64 bits of entropy. |
| V7.2.1 | Verify that the application performs all session token verification using a trusted, backend service. |
| V7.2.2 | Verify that the application uses either self-contained or reference tokens that are dynamically generated for session management, i.e. not using static API secrets and keys. |
| V7.2.3 | Verify that if reference tokens are used to represent user sessions, they are unique and generated using a cryptographically secure pseudo-random number generator (CSPRNG) and possess at least 128 bits of entropy. |
| V7.2.4 | Verify that the application generates a new session token on user authentication, including re-authentication, and terminates the current session token. |
| V7.4.1 | Verify that when session termination is triggered (such as logout or expiration), the application disallows any further use of the session. For reference tokens or stateful sessions, this means invalidating the session data at the application backend. Applications using self-contained tokens will need a solution such as maintaining a list of terminated tokens, disallowing tokens produced before a per-user date and time or rotating a per-user signing key. |
| V7.4.2 | Verify that the application terminates all active sessions when a user account is disabled or deleted (such as an employee leaving the company). |
| V7.1.1 | Verify that the user's session inactivity timeout and absolute maximum session lifetime are documented, are appropriate in combination with other controls, and that the documentation includes justification for any deviations from NIST SP 800-63B re-authentication requirements. |
| V7.1.2 | Verify that the documentation defines how many concurrent (parallel) sessions are allowed for one account as well as the intended behaviors and actions to be taken when the maximum number of active sessions is reached. |
| V7.3.1 | Verify that there is an inactivity timeout such that re-authentication is enforced according to risk analysis and documented security decisions. |
| V7.3.2 | Verify that there is an absolute maximum session lifetime such that re-authentication is enforced according to risk analysis and documented security decisions. |
| V7.4.3 | Verify that the application gives the option to terminate all other active sessions after a successful change or removal of any authentication factor (including password change via reset or recovery and, if present, an MFA settings update). |
| V7.4.4 | Verify that all pages that require authentication have easy and visible access to logout functionality. |
| V7.4.5 | Verify that application administrators are able to terminate active sessions for an individual user or for all users. |
| V7.5.1 | Verify that the application requires full re-authentication before allowing modifications to sensitive account attributes which may affect authentication such as email address, phone number, MFA configuration, or other information used in account recovery. |
| V7.5.2 | Verify that users are able to view and (having authenticated again with at least one factor) terminate any or all currently active sessions. |
| V8.1.1 | Verify that authorization documentation defines rules for restricting function-level and data-specific access based on consumer permissions and resource attributes. |
| V8.2.1 | Verify that the application ensures that function-level access is restricted to consumers with explicit permissions. |
| V8.2.2 | Verify that the application ensures that data-specific access is restricted to consumers with explicit permissions to specific data items to mitigate insecure direct object reference (IDOR) and broken object level authorization (BOLA). |
| V8.3.1 | Verify that the application enforces authorization rules at a trusted service layer and doesn't rely on controls that an untrusted consumer could manipulate, such as client-side JavaScript. |
| V8.1.2 | Verify that authorization documentation defines rules for field-level access restrictions (both read and write) based on consumer permissions and resource attributes. Note that these rules might depend on other attribute values of the relevant data object, such as state or status. |
| V8.2.3 | Verify that the application ensures that field-level access is restricted to consumers with explicit permissions to specific fields to mitigate broken object property level authorization (BOPLA). |
| V11.3.1 | Verify that insecure block modes (e.g., ECB) and weak padding schemes (e.g., PKCS#1 v1.5) are not used. |
| V11.3.2 | Verify that only approved ciphers and modes such as AES with GCM are used. |
| V11.4.1 | Verify that only approved hash functions are used for general cryptographic use cases, including digital signatures, HMAC, KDF, and random bit generation. Disallowed hash functions, such as MD5, must not be used for any cryptographic purpose. |
| V11.1.1 | Verify that there is a documented policy for management of cryptographic keys and a cryptographic key lifecycle that follows a key management standard such as NIST SP 800-57. This should include ensuring that keys are not overshared (for example, with more than two entities for shared secrets and more than one entity for private keys). |
| V11.1.2 | Verify that a cryptographic inventory is performed, maintained, regularly updated, and includes all cryptographic keys, algorithms, and certificates used by the application. It must also document where keys can and cannot be used in the system, and the types of data that can and cannot be protected using the keys. |
| V11.2.1 | Verify that industry-validated implementations (including libraries and hardware-accelerated implementations) are used for cryptographic operations. |
| V11.2.2 | Verify that the application is designed with crypto agility such that random number, authenticated encryption, MAC, or hashing algorithms, key lengths, rounds, ciphers and modes can be reconfigured, upgraded, or swapped at any time, to protect against cryptographic breaks. Similarly, it must also be possible to replace keys and passwords and re-encrypt data. This will allow for seamless upgrades to post-quantum cryptography (PQC), once high-assurance implementations of approved PQC schemes or standards are widely available. |
| V11.2.3 | Verify that all cryptographic primitives utilize a minimum of 128-bits of security based on the algorithm, key size, and configuration. For example, a 256-bit ECC key provides roughly 128 bits of security where RSA requires a 3072-bit key to achieve 128 bits of security. |
| V11.3.3 | Verify that encrypted data is protected against unauthorized modification preferably by using an approved authenticated encryption method or by combining an approved encryption method with an approved MAC algorithm. |
| V11.4.2 | Verify that passwords are stored using an approved, computationally intensive, key derivation function (also known as a "password hashing function"), with parameter settings configured based on current guidance. The settings should balance security and performance to make brute-force attacks sufficiently challenging for the required level of security. |
| V11.4.3 | Verify that hash functions used in digital signatures, as part of data authentication or data integrity are collision resistant and have appropriate bit-lengths. If collision resistance is required, the output length must be at least 256 bits. If only resistance to second pre-image attacks is required, the output length must be at least 128 bits. |
| V11.4.4 | Verify that the application uses approved key derivation functions with key stretching parameters when deriving secret keys from passwords. The parameters in use must balance security and performance to prevent brute-force attacks from compromising the resulting cryptographic key. |
| V11.5.1 | Verify that all random numbers and strings which are intended to be non-guessable must be generated using a cryptographically secure pseudo-random number generator (CSPRNG) and have at least 128 bits of entropy. Note that UUIDs do not respect this condition. |
| V11.6.1 | Verify that only approved cryptographic algorithms and modes of operation are used for key generation and seeding, and digital signature generation and verification. Key generation algorithms must not generate insecure keys vulnerable to known attacks, for example, RSA keys which are vulnerable to Fermat factorization. |
| V12.1.1 | Verify that only the latest recommended versions of the TLS protocol are enabled, such as TLS 1.2 and TLS 1.3. The latest version of the TLS protocol must be the preferred option. |
| V12.2.1 | Verify that TLS is used for all connectivity between a client and external facing, HTTP-based services, and does not fall back to insecure or unencrypted communications. |
| V12.2.2 | Verify that external facing services use publicly trusted TLS certificates. |
| V12.1.2 | Verify that only recommended cipher suites are enabled, with the strongest cipher suites set as preferred. L3 applications must only support cipher suites which provide forward secrecy. |
| V13.4.1 | Verify that the application is deployed either without any source control metadata, including the .git or .svn folders, or in a way that these folders are inaccessible both externally and to the application itself. |
| V13.1.1 | Verify that all communication needs for the application are documented. This must include external services which the application relies upon and cases where an end user might be able to provide an external location to which the application will then connect. |
| V13.2.3 | Verify that if a credential has to be used for service authentication, the credential being used by the consumer is not a default credential (e.g., root/root or admin/admin). |
| V13.3.1 | Verify that a secrets management solution, such as a key vault, is used to securely create, store, control access to, and destroy backend secrets. These could include passwords, key material, integrations with databases and third-party systems, keys and seeds for time-based tokens, other internal secrets, and API keys. Secrets must not be included in application source code or included in build artifacts. For an L3 application, this must involve a hardware-backed solution such as an HSM. |
| V13.3.2 | Verify that access to secret assets adheres to the principle of least privilege. |
| V13.4.2 | Verify that debug modes are disabled for all components in production environments to prevent exposure of debugging features and information leakage. |
| V13.4.3 | Verify that web servers do not expose directory listings to clients unless explicitly intended. |
| V13.4.4 | Verify that using the HTTP TRACE method is not supported in production environments, to avoid potential information leakage. |
| V13.4.5 | Verify that documentation (such as for internal APIs) and monitoring endpoints are not exposed unless explicitly intended. |
| V14.2.1 | Verify that sensitive data is only sent to the server in the HTTP message body or header fields, and that the URL and query string do not contain sensitive information, such as an API key or session token. |
| V14.3.1 | Verify that authenticated data is cleared from client storage, such as the browser DOM, after the client or session is terminated. The 'Clear-Site-Data' HTTP response header field may be able to help with this but the client-side should also be able to clear up if the server connection is not available when the session is terminated. |
| V14.1.1 | Verify that all sensitive data created and processed by the application has been identified and classified into protection levels. This includes data that is only encoded and therefore easily decoded, such as Base64 strings or the plaintext payload inside a JWT. Protection levels need to take into account any data protection and privacy regulations and standards which the application is required to comply with. |
| V14.1.2 | Verify that all sensitive data protection levels have a documented set of protection requirements. This must include (but not be limited to) requirements related to general encryption, integrity verification, retention, how the data is to be logged, access controls around sensitive data in logs, database-level encryption, privacy and privacy-enhancing technologies to be used, and other confidentiality requirements. |
| V14.2.2 | Verify that the application prevents sensitive data from being cached in server components, such as load balancers and application caches, or ensures that the data is securely purged after use. |
| V14.2.3 | Verify that defined sensitive data is not sent to untrusted parties (e.g., user trackers) to prevent unwanted collection of data outside of the application's control. |
| V14.2.4 | Verify that controls around sensitive data related to encryption, integrity verification, retention, how the data is to be logged, access controls around sensitive data in logs, privacy and privacy-enhancing technologies, are implemented as defined in the documentation for the specific data's protection level. |
| V14.3.2 | Verify that the application sets sufficient anti-caching HTTP response header fields (i.e., Cache-Control: no-store) so that sensitive data is not cached in browsers. |
| V14.3.3 | Verify that data stored in browser storage (such as localStorage, sessionStorage, IndexedDB, or cookies) does not contain sensitive data, with the exception of session tokens. |
| V15.1.1 | Verify that application documentation defines risk based remediation time frames for 3rd party component versions with vulnerabilities and for updating libraries in general, to minimize the risk from these components. |
| V15.2.1 | Verify that the application only contains components which have not breached the documented update and remediation time frames. |
| V15.3.1 | Verify that the application only returns the required subset of fields from a data object. For example, it should not return an entire data object, as some individual fields should not be accessible to users. |
| V15.1.2 | Verify that an inventory catalog, such as software bill of materials (SBOM), is maintained of all third-party libraries in use, including verifying that components come from pre-defined, trusted, and continually maintained repositories. |
| V15.1.3 | Verify that the application documentation identifies functionality which is time-consuming or resource-demanding. This must include how to prevent a loss of availability due to overusing this functionality and how to avoid a situation where building a response takes longer than the consumer's timeout. Potential defenses may include asynchronous processing, using queues, and limiting parallel processes per user and per application. |
| V15.2.2 | Verify that the application has implemented defenses against loss of availability due to functionality which is time-consuming or resource-demanding, based on the documented security decisions and strategies for this. |
| V15.2.3 | Verify that the production environment only includes functionality that is required for the application to function, and does not expose extraneous functionality such as test code, sample snippets, and development functionality. |
| V15.3.3 | Verify that the application has countermeasures to protect against mass assignment attacks by limiting allowed fields per controller and action, e.g., it is not possible to insert or update a field value when it was not intended to be part of that action. |
| V15.3.4 | Verify that all proxying and middleware components transfer the user's original IP address correctly using trusted data fields that cannot be manipulated by the end user, and the application and web server use this correct value for logging and security decisions such as rate limiting, taking into account that even the original IP address may not be reliable due to dynamic IPs, VPNs, or corporate firewalls. |
| V15.3.5 | Verify that the application explicitly ensures that variables are of the correct type and performs strict equality and comparator operations. This is to avoid type juggling or type confusion vulnerabilities caused by the application code making an assumption about a variable type. |
| V15.3.6 | Verify that JavaScript code is written in a way that prevents prototype pollution, for example, by using Set() or Map() instead of object literals. |
| V15.3.7 | Verify that the application has defenses against HTTP parameter pollution attacks, particularly if the application framework makes no distinction about the source of request parameters (query string, body parameters, cookies, or header fields). |
| V16.1.1 | Verify that an inventory exists documenting the logging performed at each layer of the application's technology stack, what events are being logged, log formats, where that logging is stored, how it is used, how access to it is controlled, and for how long logs are kept. |
| V16.2.1 | Verify that each log entry includes necessary metadata (such as when, where, who, what) that would allow for a detailed investigation of the timeline when an event happens. |
| V16.2.2 | Verify that time sources for all logging components are synchronized, and that timestamps in security event metadata use UTC or include an explicit time zone offset. UTC is recommended to ensure consistency across distributed systems and to prevent confusion during daylight saving time transitions. |
| V16.2.3 | Verify that the application only stores or broadcasts logs to the files and services that are documented in the log inventory. |
| V16.2.4 | Verify that logs can be read and correlated by the log processor that is in use, preferably by using a common logging format. |
| V16.2.5 | Verify that when logging sensitive data, the application enforces logging based on the data's protection level. For example, it may not be allowed to log certain data, such as credentials or payment details. Other data, such as session tokens, may only be logged by being hashed or masked, either in full or partially. |
| V16.3.1 | Verify that all authentication operations are logged, including successful and unsuccessful attempts. Additional metadata, such as the type of authentication or factors used, should also be collected. |
| V16.3.2 | Verify that failed authorization attempts are logged. For L3, this must include logging all authorization decisions, including logging when sensitive data is accessed (without logging the sensitive data itself). |
| V16.3.3 | Verify that the application logs the security events that are defined in the documentation and also logs attempts to bypass the security controls, such as input validation, business logic, and anti-automation. |
| V16.3.4 | Verify that the application logs unexpected errors and security control failures such as backend TLS failures. |
| V16.4.1 | Verify that all logging components appropriately encode data to prevent log injection. |
| V16.4.2 | Verify that logs are protected from unauthorized access and cannot be modified. |
| V16.4.3 | Verify that logs are securely transmitted to a logically separate system for analysis, detection, alerting, and escalation. The aim is to ensure that if the application is breached, the logs are not compromised. |
| V16.5.1 | Verify that a generic message is returned to the consumer when an unexpected or security-sensitive error occurs, ensuring no exposure of sensitive internal system data such as stack traces, queries, secret keys, and tokens. |
| V16.5.3 | Verify that the application fails gracefully and securely, including when an exception occurs, preventing fail-open conditions such as processing a transaction despite errors resulting from validation logic. |
| SBD-AC-01 | All communications use TLS/mTLS; service identity is verified (e.g., mesh-issued certs). |
| SBD-AC-02 | Central IdP (OIDC/OAuth2) is used; MFA enforced on privileged/admin paths; tokens are short-lived. |
| SBD-AC-03 | RBAC/ABAC governs APIs and publish/consume on messaging; least privilege is applied. |
| SBD-AC-05 | Secrets are stored in a secret manager; keys/certs rotate automatically; no secrets in code/logs. |
| SBD-AC-06 | Applicable regulatory controls (e.g., privacy/financial) are identified with evidence of design alignment. |
| SBD-AS-07 | Startup resilience: services handle missing dependencies; autoscaling defined with minimum replicas. |
| SBD-DM-02 | Encryption in transit (TLS/mTLS) and at rest with managed keys and rotation is designed in. |
| SBD-DM-01 | Data are classified with named owners; controls are proportional to classification. |
| SBD-DM-05 | Retention/deletion policies exist per data class; minimization is applied to collection/storage. |
| SBD-MT-02 | Metrics/SLOs and dashboards cover service health and event-flow anomalies; alerts are actionable. |
| SBD-MT-03 | ASVS-aligned security tests and negative tests from threat-modeling are planned in the test strategy. |
| SBD-MT-04 | SbD controls are verifiable in test (isolation, mTLS, authZ denials, rate-limit behavior). |
| SBD-MT-05 | OpenAPI/AsyncAPI and event catalogs are published; ADRs/runbooks are current. |
| SBD-MT-06 | A documented Incident Response plan (roles, comms, evidence handling) exists and is rehearsed. |
| SBD-MT-07 | Audit log retention ≥12 months with tamper-evidence and a searchable hot window is defined. |
| SBD-MT-01 | Structured centralized logs with correlation/trace IDs; administrative access is logged. |
| SBD-RR-06 | Timeouts on all calls; health probes; explicit failover/multi-AZ strategies for critical paths. |
| SBD-RR-08 | Caching/CDN/back-pressure patterns are applied where appropriate; performance guardrails are defined. |
| SBD-RR-01 | Retries use exponential backoff + jitter; default exception handling and safe error models exist. |
| SBD-RR-05 | All mutating endpoints enforce an Idempotency-Key; concurrency controls (locks/optimistic concurrency) are in place. |
| SBD-RR-07 | Quotas and autoscaling are defined; rate-limits enforced at edges; minimum capacity documented. |
| C7.2.1 | Verify that the system assesses the reliability of generated answers using a confidence estimation method. |
| C7.2.2 | Verify that the application automatically blocks answers or switches to a fallback message if the confidence score drops below a defined threshold. |
| C12.2.2 | Verify that behavioral anomaly detection identifies unusual conversation patterns, excessive retry attempts, or probing behaviors. |
| C12.2.3 | Verify that custom rules detect AI-specific threat patterns for coordinated jailbreak attempts, prompt injection, and system prompt extraction attempts. |
| C12.2.4 | Verify that extraction-alert events include offending query metadata to support investigation. |

