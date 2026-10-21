#!/usr/bin/env python3
"""Writes docs/COVERAGE.md: which requirements any check in `sv` can speak to, and what it needs to run;
with it, docs/REQUIREMENTS.md (every requirement and its checks) and data/reach.json.

    python3 tools/coverage.py            # rewrite all three
    python3 tools/coverage.py --check    # fail if any of the three is not what this would write
    python3 tools/coverage.py --credits LOG  # fail if the suite's credits disagree with the lists below
    python3 tools/coverage.py --withheld LOG # list the checks the suite saw credit and never saw withhold

Everything is read from where the checks themselves keep their citations, so the document cannot
claim a check the code does not have:

- `data/ast-rules.json`, `data/secret-rules.json`, `data/adapters.json`: the rules and their
  requirements;
- the checks whose citations are written in Rust (probes, signed-in checks, configuration, the bill
  of materials, advisories), listed in `RUST_CHECKS` below. A requirement id written into `sv`'s
  code as a string of its own that is in neither that table nor `MENTIONS` stops this script, so a
  new hard-coded check cannot be left out;
- `data/knowledge/applicability.json`'s `manualOnly`, and the Secure by Design prefix, for the
  requirements a check can support but never settle;
- `data/sbd-asvs-crosswalk.json`, for the checklist controls with an ASVS counterpart.

It also holds `data/prompts.json` and `data/design-prompts.json` to the same citations: each prompt names the requirements it
targets and the rules whose result shows whether it worked, and every one of those requirements has
to be cited by one of those rules, and every rule has to cite one of them, or one the prompt sets aside with its reason
(`check_prompts`).

A test (`crates/sv-check/tests/coverage_doc.rs`) runs `--check`, so a change to the checks that is not
followed by regenerating this document fails the build.

`--credits` reads the file the test suite writes when `SV_CREDIT_LOG` names it (CI's test job does):
one line for every credit a check gave, with the place in the code that gave it, and one for every
time a credit was marked in part or made whole again (`check_credits`).
"""

import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _shared import write_text  # noqa: E402  (tools/_shared.py)


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
OUT = ROOT / "docs" / "COVERAGE.md"
LIST_OUT = ROOT / "docs" / "REQUIREMENTS.md"
# Which kinds of run can credit each requirement, read by the report's short version to say how many
# requirements only a kind of run that did not happen could have reached.
REACH_OUT = ROOT / "data" / "reach.json"
# The two tables a picture reads best from, drawn from the same counts (backlog 0483).
CHAPTER_SVG = ROOT / "docs" / "coverage-by-chapter.svg"
LEVEL_SVG = ROOT / "docs" / "coverage-by-level.svg"

# What each kind of check needs before it can run at all.
TIERS = [
    ("static", "Reads the code", "nothing: plain `sv check`"),
    ("advisories", "Known vulnerabilities", "a local copy of the OSV database (`--advisories DIR`)"),
    (
        "running",
        "The running app",
        "a container backend and a `run` section (`--run`); for the AI checks, an `ai` section too",
    ),
    (
        "signed-in",
        "Signed in",
        "the above, and a `users` section with test accounts, or an `oidc` section for a sign-in "
        "through another service",
    ),
    ("tools", "Outside tools", "the tool installed; runs with `sv report` unless `--no-tools`"),
    (
        "opt-in",
        "Outside tools you turn on",
        "the tool installed and turned on in stackvet.toml (`[tools] codeql = true`), for a use its license "
        "allows: an open-source app, academic research, or a paid GitHub Advanced Security license",
    ),
    (
        "production",
        "Your own live site",
        "the address your app is served from, typed at the terminal (`sv probe https://…`)",
    ),
]

# The tiers whose checks are outside tools: the ones that run by default, and those the owner turns on (ADR-115).
TOOL_TIERS = ("tools", "opt-in")


def is_tool(tier, check):
    """Whether `check`, under `tier`, is an outside tool: one of `sv`'s own checks can sit in a tool
    tier too, when it credits only after the tools ran (`config.template-engine-escapes`, backlog
    0613), and is still `sv`'s own check, with its own words and its own credit."""
    return tier in TOOL_TIERS and check not in RUST_CHECKS

# Checks whose citations are written in Rust rather than in a data file.
RUST_CHECKS = {
    "config.secrets-file-committed": ("static", ["V13.3.1"]),
    "config.template-engine-escapes": ("tools", ["V1.1.2"]),  # only after the outside tools ran (backlog 0613)
    "config.development-server-started": ("static", ["V15.2.3"]),
    "config.mcp-server-unpinned": ("static", ["C10.1.1"]),
    "config.mcp-transport-unencrypted": ("static", ["C10.3.1"]),
    "config.rich-text-without-sanitizer": ("static", ["V1.3.1"]),
    "config.sign-in-pathway-not-listed": ("static", ["V6.3.4"]),
    "config.session-timeout-differs-from-policy": ("static", ["V7.1.1"]),
    "ast.weak-key-generation": ("static", ["V11.6.1"]),
    "config.client-tech-unsupported": ("static", ["V3.7.1"]),
    "config.retired-grant-enabled": ("static", ["V10.4.4"]),
    "config.model-file-can-run-code": ("static", ["C4.1.2"]),
    "config.certificate-checks-off": ("static", ["V12.3.2", "V12.3.4"]),
    "config.firebase-rules-open": ("static", ["V8.2.2", "V8.2.1"]),
    "config.supabase-table-without-rls": ("static", ["V8.2.2", "V8.2.1"]),
    "config.supabase-policy-allows-all": ("static", ["V8.2.2", "V8.2.1"]),
    "config.secret-under-public-name": ("static", ["V13.3.1", "SBD-AC-05"]),
    "probe.retired-grants-offered": ("running", ["V10.4.4"]),
    "probe.admin-opened-by-address": ("running", ["V8.4.2"]),
    "probe.private-files-served": ("running", ["V13.4.7"]),
    "probe.app-stopped-during-questions": ("running", ["V16.5.4"]),
    "config.gitignore-covers-env": ("static", ["V13.3.1"]),
    "config.versions-pinned": ("static", ["V15.1.2"]),
    "config.workflow-runs-fork-code": ("static", ["AC.12.1"]),
    "config.workflow-checkout-keeps-token": ("static", ["AC.12.2"]),
    "config.workflow-secrets-with-fork-code": ("static", ["AC.12.3"]),
    "config.workflow-hands-out-all-secrets": ("static", ["V13.3.2"]),
    "config.workflow-untrusted-text-in-run": ("static", ["AC.12.1"]),
    "sbom": ("static", ["V15.1.2"]),
    "secrets.credential-assignment": ("static", ["V13.3.1", "SBD-AC-05"]),
    "secrets.password-in-url": ("static", ["V13.3.1", "SBD-AC-05"]),
    "advisories": ("advisories", ["V15.2.1"]),
    "probe.security-headers": ("running", ["V3.4.3", "V3.4.4", "V3.4.5", "V3.4.6"]),
    "probe.cookie-attributes": ("running", ["V3.3.2", "V3.3.4"]),
    "probe.cookie-too-large": ("running", ["V3.3.5"]),
    "probe.cors-any-origin": ("running", ["V3.4.2"]),
    "probe.error-detail-leak": ("running", ["V13.4.2", "V16.5.1"]),
    "probe.trace-enabled": ("running", ["V13.4.4"]),
    "probe.ai-instructions-leaked": ("running", ["C7.3.2"]),
    "probe.ai-output-fetched": ("running", ["C7.3.3", "C9.3.7"]),
    "probe.ai-output-unbounded": ("running", ["C7.1.2"]),
    "probe.ai-injection-unscreened": ("running", ["C2.1.3"]),
    "probe.ai-call-log-incomplete": ("running", ["C12.1.3"]),
    "probe.ai-injection-logged": ("running", ["C12.2.1"]),
    "probe.ai-rate-unlimited": ("running", ["C11.2.2"]),
    "probe.ai-kill-switch-ignored": ("running", ["C9.6.1"]),
    "probe.ai-mcp-output-unvalidated": ("running", ["C10.4.1", "C9.3.2"]),
    "probe.ai-mcp-injection-unscreened": ("running", ["C10.4.2"]),
    "probe.ai-stored-injection-unscreened": ("running", ["C2.1.3"]),
    "probe.ai-reply-html-unencoded": ("running", ["V1.2.1"]),
    "probe.ai-input-truncated": ("running", ["C2.1.4"]),
    "probe.ai-injection-other-languages": ("running", ["C2.2.2"]),
    "probe.ai-hidden-content-passed": ("running", ["C7.3.4"]),
    "probe.ai-flagged-reply-shown": ("running", ["C7.3.1"]),
    "probe.ai-raw-response-exposed": ("running", ["C11.3.2"]),
    "probe.ai-floating-model-sent": ("running", ["C3.2.3"]),
    "probe.ai-service-error-shown": ("running", ["V16.5.1"]),
    "probe.ai-service-failure-handled": ("running", ["V16.5.2"]),
    "probe.ai-service-failure-logged": ("running", ["V16.3.4"]),
    "probe.ai-output-shape-unchecked": ("running", ["C7.1.1"]),
    "probe.ai-agent-unbounded": ("running", ["C9.1.2"]),
    "probe.ai-tool-timeout": ("running", ["C9.1.1"]),
    "probe.ai-hidden-input": ("running", ["C2.1.2"]),
    "probe.ai-input-charset-unrestricted": ("running", ["C2.1.5"]),
    "probe.ai-call-log-session": ("running", ["C12.1.1"]),
    "probe.ai-token-use-attributed": ("running", ["C12.2.5"]),
    "probe.ai-safety-decision-detailed": ("running", ["C12.1.2"]),
    "probe.ai-tool-action-logged": ("running", ["C12.4.2"]),
    "probe.ai-flag-not-gating": ("running", ["C11.4.2"]),
    "probe.ai-tool-reads-others-records": ("signed-in", ["C9.5.3"]),
    "probe.ai-retrieval-ignores-user": ("signed-in", ["C5.2.2", "C8.1.3"]),
    "probe.ai-reply-carries-others-data": ("signed-in", ["C5.2.4"]),
    "probe.mcp-server-origin-unchecked": ("running", ["C10.3.3"]),
    "probe.fetch-goes-anywhere": ("running", ["V1.3.6", "V13.2.4"]),
    "probe.fetch-follows-redirect": ("running", ["V15.3.2"]),
    "probe.mcp-server-token-unchecked": ("running", ["C10.2.1"]),
    "probe.mcp-server-takes-unknown-or-oversized-arguments": ("running", ["C10.4.3"]),
    "probe.mcp-server-takes-wrong-types": ("running", ["C10.4.4"]),
    "probe.mcp-server-no-size-limit": ("running", ["C10.4.5"]),
    "probe.mcp-session-survives-end": ("running", ["C10.2.6"]),
    "probe.unused-method-accepted": ("running", ["V4.1.4"]),
    "probe.jsonp-enabled": ("running", ["V3.5.6"]),
    "probe.docs-or-monitoring-exposed": ("running", ["V13.4.5"]),
    "probe.log-file-served": ("running", ["V16.4.2"]),
    "probe.development-console-open": ("running", ["V15.2.3", "V13.4.2"]),
    "probe.version-disclosed": ("running", ["V13.4.6"]),
    "probe.reflected-unencoded": ("running", ["V1.2.1"]),
    "probe.sql-injection": ("signed-in", ["V1.2.4"]),
    "probe.reflected-json-unescaped": ("running", ["V1.2.3"]),
    "probe.opener-policy-missing": ("running", ["V3.4.8"]),
    "probe.csp-no-report": ("running", ["V3.4.7"]),
    "probe.content-type": ("running", ["V4.1.1"]),
    "probe.source-control-exposed": ("running", ["V13.4.1"]),
    "probe.private-page-anonymous": ("signed-in", ["V8.2.1"]),
    "probe.admin-page-ordinary-user": ("signed-in", ["V8.2.1", "V8.3.1"]),
    "probe.admin-action-ordinary-user": ("signed-in", ["V8.2.1", "V8.3.1"]),
    "probe.role-field-trusted": ("signed-in", ["V8.3.1", "V15.3.3", "V8.2.3"]),
    "probe.email-change-role-trusted": ("signed-in", ["V8.3.1", "V15.3.3", "V8.2.3"]),
    "probe.owner-field-trusted": ("signed-in", ["V15.3.3", "V8.2.3", "V8.2.2"]),
    "probe.stored-unencoded": ("signed-in", ["V1.2.1"]),
    "probe.other-users-data": ("signed-in", ["V8.2.2"]),
    "probe.session-cookie-attributes": ("signed-in", ["V3.3.2", "V3.3.4"]),
    "probe.session-not-renewed": ("signed-in", ["V7.2.4"]),
    "probe.logout-keeps-session": ("signed-in", ["V7.4.1"]),
    "probe.cross-site-request-accepted": ("signed-in", ["V3.5.1"]),
    "probe.csrf-token-shared": ("signed-in", ["V3.5.1"]),
    "probe.short-password-accepted": ("signed-in", ["V6.2.1"]),
    "probe.common-password-accepted": ("signed-in", ["V6.2.4"]),
    "probe.breached-password-accepted": ("signed-in", ["V6.2.12"]),
    "probe.context-word-password-accepted": ("signed-in", ["V6.2.11"]),
    "probe.flow-step-skipped": ("signed-in", ["V2.3.1"]),
    "probe.activation-code-guessable": ("signed-in", ["V6.4.1"]),
    "probe.activation-link-reusable": ("signed-in", ["V6.4.1"]),
    "probe.email-code-long-lived": ("signed-in", ["V6.5.5"]),
    "probe.websocket-without-session": ("signed-in", ["V4.4.4"]),
    "probe.websocket-after-sign-out": ("signed-in", ["V4.4.3"]),
    "probe.totp-reused": ("signed-in", ["V6.5.1"]),
    "probe.totp-old-code-accepted": ("signed-in", ["V6.5.5"]),
    "probe.forwarded-for-trusted": ("signed-in", ["V15.3.4"]),
    "probe.password-composition-rules": ("signed-in", ["V6.2.5"]),
    "probe.default-account": ("signed-in", ["V6.3.2"]),
    "probe.password-in-url": ("signed-in", ["V14.2.1"]),
    "probe.oidc-sign-in-from-another-session": ("signed-in", ["V10.1.2", "V10.2.1", "V7.6.2"]),
    "probe.oidc-nonce-not-checked": ("signed-in", ["V10.5.1"]),
    "probe.oidc-audience-not-checked": ("signed-in", ["V10.5.4"]),
    "probe.oidc-signature-not-checked": ("signed-in", ["V6.8.2"]),
    "probe.oidc-issuer-not-checked": ("signed-in", ["V10.2.2"]),
    "probe.oidc-user-keyed-on-email": ("signed-in", ["V10.5.2"]),
    "probe.oidc-access-token-audience-not-checked": ("signed-in", ["V10.3.1"]),
    "probe.oidc-id-token-taken-as-access-token": ("signed-in", ["V9.2.2"]),
    "probe.oidc-logout-token-not-checked": ("signed-in", ["V10.5.5"]),
    "probe.sign-out-control-hidden": ("signed-in", ["V7.4.4"]),
    "probe.text-rendered-as-markup": ("signed-in", ["V3.2.2"]),
    "probe.storage-kept-after-sign-out": ("signed-in", ["V14.3.1"]),
    "probe.account-details-sent-elsewhere": ("signed-in", ["V14.2.3"]),
    "probe.undeclared-outside-service": ("signed-in", ["V13.1.1"]),
    "probe.bypass-attempt-logged": ("signed-in", ["V16.3.3"]),
    "probe.preflight-skipped": ("signed-in", ["V3.5.2"]),
    "probe.session-id-weak": ("signed-in", ["V7.2.3"]),
    "probe.session-token-static": ("signed-in", ["V7.2.2"]),
    "probe.password-altered": ("signed-in", ["V6.2.8"]),
    "probe.long-password-refused": ("signed-in", ["V6.2.9"]),
    "probe.crashed-on-ordinary-request": ("signed-in", ["V16.5.3"]),
    "probe.sessions-ended-without-password": ("signed-in", ["V7.5.2"]),
    "probe.other-sessions-not-ended": ("signed-in", ["V7.5.2"]),
    "probe.admin-end-sessions-ignored": ("signed-in", ["V7.4.5"]),
    "probe.password-field-unmasked": ("signed-in", ["V6.2.6"]),
    "probe.password-paste-blocked": ("signed-in", ["V6.2.7"]),
    "probe.sign-out-on-get": ("signed-in", ["V3.5.3"]),
    "probe.password-change": ("signed-in", ["V6.2.2"]),
    "probe.password-change-without-current": ("signed-in", ["V6.2.3"]),
    "probe.action-done-twice": ("signed-in", ["V2.3.4"]),
    "probe.create-rate-unlimited": ("signed-in", ["V2.4.1"]),
    "probe.open-redirect": ("signed-in", ["V3.7.2"]),
    "probe.email-change-without-password": ("signed-in", ["V7.5.1"]),
    "probe.phone-change-without-password": ("signed-in", ["V7.5.1"]),
    "probe.two-factor-change-without-password": ("signed-in", ["V7.5.1"]),
    "probe.recovery-change-without-password": ("signed-in", ["V7.5.1"]),
    "probe.password-change-ends-sessions": ("signed-in", ["V7.4.3"]),
    "probe.password-change-notified": ("signed-in", ["V6.3.7"]),
    "probe.identity-header-trusted": ("signed-in", ["V4.1.3"]),
    "probe.uploaded-svg-keeps-script": ("signed-in", ["V1.3.4"]),
    "probe.upload-path-traversal": ("signed-in", ["V5.3.2"]),
    "probe.upload-not-scanned": ("signed-in", ["V5.4.3"]),
    "probe.token-in-browser-storage": ("signed-in", ["V10.1.1"]),
    "probe.password-in-browser-storage": ("signed-in", ["V14.3.3"]),
    "probe.sessions-survive-deletion": ("signed-in", ["V7.4.2"]),
    "probe.password-hints": ("signed-in", ["V6.4.2"]),
    "probe.reset-reusable": ("signed-in", ["V6.4.3"]),
    "probe.mail-header-injected": ("signed-in", ["V1.3.11"]),
    "probe.reset-keeps-old-password": ("signed-in", ["V6.4.3"]),
    "probe.reset-code-guessable": ("signed-in", ["V6.4.3"]),
    "probe.reset-code-in-answer": ("signed-in", ["V6.4.3"]),
    "probe.reset-reveals-account": ("signed-in", ["V6.3.8"]),
    "probe.signin-reveals-account": ("signed-in", ["V6.3.8"]),
    "probe.signup-reveals-account": ("signed-in", ["V6.3.8"]),
    "probe.signup-replaces-account": ("signed-in", ["V6.2.3"]),
    "probe.email-code-reusable": ("signed-in", ["V6.5.1"]),
    "probe.session-idle-timeout": ("signed-in", ["V7.3.1"]),
    "probe.session-lifetime": ("signed-in", ["V7.3.2"]),
    "probe.email-code-unbound": ("signed-in", ["V6.6.2"]),
    "probe.email-code-short": ("signed-in", ["V6.5.4"]),
    "probe.email-code-guessing-unlimited": ("signed-in", ["V6.6.3"]),
    "probe.failed-sign-ins-unlimited": ("signed-in", ["V6.3.1"]),
    "probe.input-length-unlimited": ("signed-in", ["V2.2.1"]),
    "probe.input-out-of-range-stored": ("signed-in", ["V2.2.1"]),
    "probe.certificate-not-trusted": ("production", ["V12.2.2"]),
    "probe.plain-http-served": ("production", ["V12.2.1"]),
    "probe.ocsp-not-stapled": ("production", ["V12.1.4"]),
    "probe.api-redirected-to-https": ("production", ["V4.1.2"]),
    "probe.old-tls-accepted": ("production", ["V12.1.1"]),
    "probe.no-hsts": ("production", ["V3.4.1"]),
    "live.ech-not-offered": ("production", ["V12.1.5"]),
    "live.hsts-not-preloaded": ("production", ["V3.7.4"]),
    "probe.cookie-without-host-prefix": ("production", ["V3.3.3"]),
    "probe.directory-listing": ("running", ["V13.4.3"]),
    "probe.private-page-cached": ("signed-in", ["V14.3.2"]),
    "probe.private-page-shared-cache": ("signed-in", ["V14.2.2"]),
    "probe.private-page-headers": ("signed-in", ["V3.4.3", "V3.4.4", "V3.4.5", "V3.4.6"]),
    "probe.no-sign-out-link": ("signed-in", ["V7.4.4"]),
    "probe.oversized-file-accepted": ("signed-in", ["V5.2.1"]),
    "probe.archive-unchecked": ("signed-in", ["V5.2.3"]),
    "probe.file-contents-unchecked": ("signed-in", ["V5.2.2"]),
    "probe.uploaded-file-executed": ("signed-in", ["V5.3.1"]),
    "probe.uploaded-file-rendered": ("signed-in", ["V3.2.1"]),
    "probe.saved-page-rendered": ("signed-in", ["V3.2.1"]),
    "probe.authentication-logged": ("signed-in", ["V16.3.1"]),
    "probe.authorization-failure-logged": ("signed-in", ["V16.3.2"]),
    "probe.log-line-metadata": ("signed-in", ["V16.2.1"]),
    "probe.log-timestamp-zoned": ("signed-in", ["V16.2.2"]),
    "probe.download-unnamed": ("signed-in", ["V5.4.1"]),
    "probe.download-name-injected": ("signed-in", ["V5.4.2"]),
    "probe.log-common-format": ("signed-in", ["V16.2.4"]),
    "probe.graphql-introspection": ("running", ["V4.3.2"]),
    "probe.graphql-no-amount-limit": ("running", ["V4.3.1"]),
    "probe.websocket-origin-unchecked": ("running", ["V4.4.2"]),
    "probe.validation-only-in-the-browser": ("signed-in", ["V2.2.2"]),
    "probe.session-token-unverified": ("signed-in", ["V7.2.1"]),
    "probe.app-token-signature-not-checked": ("signed-in", ["V9.1.1"]),
    "probe.app-token-alg-none": ("signed-in", ["V9.1.2"]),
    "probe.app-token-expired-accepted": ("signed-in", ["V9.2.1"]),
    "probe.app-token-key-source-followed": ("signed-in", ["V9.1.3"]),
    "probe.app-token-placeholder-key": ("signed-in", ["V9.1.1"]),
    "probe.record-returns-secret-fields": ("signed-in", ["V15.3.1", "V8.2.3"]),
    "probe.clear-site-data": ("signed-in", ["V14.3.1"]),
}

# Checks in RUST_CHECKS that only ever raise their requirement as a finding: a clean run of one
# credits nothing, because what would settle the requirement is not in anything the check reads.
RUST_FINDINGS_ONLY = {
    # One stored pattern screened out, or a note the search did not hand over, is not every way of
    # writing one (gap analysis finding 13(e)).
    "probe.ai-stored-injection-unscreened",
    # One reply on one page is not every place an app writes one (gap analysis finding 13(b)).
    "probe.ai-reply-html-unencoded",
    # Nine guessed addresses cannot show that no log is served (ADR-072).
    "probe.log-file-served",
    # No list of what a page loads is complete, so finding none shows nothing (ADR-070).
    "config.client-tech-unsupported",
    # An `https://` address shows neither that the link is authenticated nor that it is the only one
    # (ADR-068).
    "config.mcp-transport-unencrypted",
    # A few characters kept out do not show an allow-list (ADR-065).
    "probe.ai-input-charset-unrestricted",
    # Found on 3 October 2026 to be counted as crediting when no code path gives them credit: each
    # only ever raises a finding. Several have a control that could let them credit (see the backlog).
    "probe.directory-listing",
    "probe.open-redirect",
    "probe.docs-or-monitoring-exposed",
    "probe.jsonp-enabled",
    "probe.unused-method-accepted",
    "probe.version-disclosed",
    "probe.account-details-sent-elsewhere",
    # Only what the pages ask for is seen, never what the server asks for (backlog 0496).
    "probe.undeclared-outside-service",
    "probe.activation-code-guessable",
    "probe.activation-link-reusable",
    "probe.default-account",
    "probe.email-code-short",
    "probe.forwarded-for-trusted",
    "probe.password-in-url",
    "probe.password-paste-blocked",
    # A sign-up that did not crash shows nothing about every other request (backlog 0528).
    "probe.crashed-on-ordinary-request",
    # The credit for ending sessions is the re-authentication rule's (backlog 0492).
    "probe.other-sessions-not-ended",
    "probe.reset-code-guessable",
    "probe.reset-code-in-answer",
    "probe.reset-keeps-old-password",
    "probe.reset-reusable",
    "probe.reset-reveals-account",
    "probe.signin-reveals-account",
    "probe.signup-reveals-account",
    "probe.signup-replaces-account",
    "probe.session-id-weak",
    "probe.sign-out-on-get",
    "probe.validation-only-in-the-browser",
    "probe.websocket-after-sign-out",
    "config.workflow-secrets-with-fork-code",
    "probe.role-field-trusted",
    "probe.email-change-role-trusted",
    "probe.owner-field-trusted",
    "probe.stored-unencoded",
    "probe.record-returns-secret-fields",
    "probe.private-page-shared-cache",
    "probe.ai-output-fetched",
    "config.development-server-started",
    "config.mcp-server-unpinned",
    "config.rich-text-without-sanitizer",
    # A list that names every way in found credits nothing: a way in may be one the reader does not find (ADR-123).
    "config.sign-in-pathway-not-listed",
    # A lifetime set elsewhere (a hosting console, a library default) is not read (backlog 0612).
    "config.session-timeout-differs-from-policy",
    # No weak key made in the code says nothing of keys made elsewhere, or of signing and seeding (backlog 0494).
    "ast.weak-key-generation",
    # A refusal of TLS 1.0 and 1.1 is half of V12.1.1; the other half, the newest version preferred,
    # is not something curl reports reliably, so a refusal is said and not credited.
    "probe.old-tls-accepted",
    # One value on three pages is not every place the app writes out what it was sent.
    "probe.reflected-unencoded",
    # Two strangers handed different tokens do not show the app checks them (backlog 0537).
    "probe.csrf-token-shared",
    "probe.reflected-json-unescaped",
    # A few values in a few addresses, told apart or not, is not every query the app builds.
    "probe.sql-injection",
    # One note each and one question is not every way an app searches.
    "probe.ai-retrieval-ignores-user",
    "probe.ai-reply-carries-others-data",
    # The requirement names no size, and this is one size.
    "probe.mcp-server-no-size-limit",
    # The fence has no address that should be allowed, so not fetching is never credit.
    "probe.fetch-goes-anywhere",
    # An app that ignores a token's `jku` cannot be told from one that checks it against a list.
    "probe.app-token-key-source-followed",
    # Not matching a list of placeholder secrets does not show the app's secret is strong.
    "probe.app-token-placeholder-key",
    "config.retired-grant-enabled",
    "config.model-file-can-run-code",
    # A setting made on the server itself is in no file.
    "config.certificate-checks-off",
    # A rules file or migration with none of these shapes may still let one user reach another's data.
    "config.firebase-rules-open",
    "config.supabase-table-without-rls",
    "config.supabase-policy-allows-all",
    "config.secret-under-public-name",
    # Secrets handed out with care in the workflows say nothing of who else can read them.
    "config.workflow-hands-out-all-secrets",
    # An address with no password in it says nothing of keys kept elsewhere.
    "secrets.password-in-url",
    # Text pasted into commands carefully elsewhere says nothing of what the workflow runs.
    "config.workflow-untrusted-text-in-run",
    "probe.retired-grants-offered",
    "probe.admin-opened-by-address",
    "probe.private-files-served",
    "probe.app-stopped-during-questions",
    "probe.ai-input-truncated",
    "probe.ai-injection-other-languages",
    "probe.ai-raw-response-exposed",
    "probe.ai-floating-model-sent",
    "probe.ai-service-error-shown",
    "probe.development-console-open",
    "probe.identity-header-trusted",
    "probe.token-in-browser-storage",
    "probe.password-in-browser-storage",
    # Found on 6 October 2026 by the census of what the suite credits (`check_credits`). A hint or a
    # secret question only ever raises a finding. The assignment rule's clean run is credited as
    # `secrets.scan`, which names the requirements of `data/secret-rules.json` and not V13.2.3.
    "probe.password-hints",
    "secrets.credential-assignment",
    # One API address the owner names is not every endpoint (ADR-027, Later, 6 October 2026).
    "probe.api-redirected-to-https",
}

# The other way round: checks in RUST_CHECKS that only ever credit their requirement. What they
# would find missing may be met some way the check cannot see, so it is left not assessed.
RUST_CREDITS_ONLY = {
    "probe.ai-tool-action-logged",
    "probe.ai-safety-decision-detailed",
    "probe.ai-token-use-attributed",
    "probe.ai-service-failure-logged",
    "probe.password-change-ends-sessions",
    "probe.password-change-notified",
    "probe.ai-tool-timeout",
}

# Checks in RUST_CHECKS whose credit is only ever *in part* (ADR-053): they try one piece of what
# their requirement asks, so a requirement they alone credit is *checked in part*, never *checked*.
# C9.1.1 names five quotas, and a check from outside the app can time only one (ADR-064); C2.1.2's
# smuggling is tried in one family of characters, not in encodings or look-alike letters (ADR-065).
# V1.3.11's mail header injection is tried in one field of one kind of mail, the reset (ADR-069); V16.3.4's
# logged error is one service's failure, found in the app's output (ADR-071); C12.2.5's token use is seen tied
# per user and per session, not per feature or team (ADR-073); C12.1.2's detail is read on one kind of safety
# decision, the injection screen (ADR-074); C12.4.2's audited action is one read-only tool call (ADR-075);
# C11.4.2's flagged input is one textbook injection (ADR-079).
RUST_IN_PART = {"probe.cookie-too-large", "probe.input-length-unlimited", "probe.input-out-of-range-stored", "probe.ai-tool-timeout", "probe.ai-hidden-input", "probe.mail-header-injected", "probe.ai-service-failure-logged", "probe.ai-token-use-attributed", "probe.ai-safety-decision-detailed", "probe.ai-tool-action-logged", "probe.ai-flag-not-gating", "probe.ai-kill-switch-ignored", "probe.bypass-attempt-logged"}

# Ids written into the code as strings that are not evidence: examples in comments on how ids are
# parsed, a requirement named only to say it is not assessed, and the three ids `sv init` prints as
# worked examples of a [design] answer. The design questions cite their requirements in
# `data/design-questions.json`, and they are deliberately absent from this document: an answer there
# is the owner's word, which is the one thing this file must not count as coverage. V12.1.2 is named in the
# report's "Before going live" list (`crates/sv-report/src/live.rs`) as one to check with a scanner, which
# credits nothing. V8.1.1 and V8.1.2 are the questions the app's routes are shown beside (`crates/sv-cli/src/code_lines.rs`,
# backlog 0493), V6.1.3 and V6.3.4 the ones the ways into the app are shown beside (backlog 0491), and V14.1.1 and
# V14.1.2 the ones the fields that look personal are shown beside (backlog 0497), and V11.1.1 and V11.1.2 the ones the
# draft cryptographic inventory is shown beside (backlog 0494), and V6.1.1 the question the wrong-password check is
# shown beside (backlog 0609); none credits anything.
MENTIONS = {"V6.1.1", "AC.4.1", "SBD-AC-01", "V6.2.1", "V3.3.1", "V8.3.1", "V2.2.2", "V13.2.1", "V12.1.2", "V8.1.1", "V8.1.2",
            "V6.1.3", "V6.3.4", "V14.1.1", "V14.1.2", "V11.1.1", "V11.1.2",
            # The hand check on using a cryptography library, shown the draft inventory (0611).
            "V11.2.1",
            # The hand check on one key for several audiences, shown the app's own token's `aud` (0500).
            "V9.2.4",
            # The hand check on data protection and a part it lists beside it, with the report's verdict (0610).
            "V14.2.4", "V16.2.5",
            # The hand check on availability defenses and the running checks it lists beside it (0498).
            "V15.2.2",
            # The owner's confirmed time frames, V15.1.1 on the owner's word; never coverage (0498).
            "V15.1.1",
            # The hook section names the requirements a claim speaks to, beside their verdict; never evidence (0607).
            "V1.2.5", "V11.4.2"}

ID = r"(?:V|C)\d+\.\d+\.\d+|AC\.\d+\.\d+|SBD-[A-Z]+-\d+"


# The chart's four parts, in the order they are drawn, with the colors they are drawn in. The three hues were checked
# with a palette validator for lightness, chroma, color-blind separation, and contrast on light and dark backgrounds;
# "nothing" is drawn in the page's own gray, so what no check reaches looks like the background it is. The brand's
# palette has not chosen status colors yet (docs/brand/README.md, "Not yet chosen"), so these are from its family.
# "The owner answers" (backlog 0624) is no new hue, since none passed the validator beside these three on every pair:
# it is the person's blue hatched over the gray, a person and no check, and its count is written beside it as every
# part's is.
OWNER_BLUE = "#5470C0"
SEGMENTS = (
    ("can be credited", "#3A9470"),
    ("can only be found failing", "#D9692B"),
    ("a check helps, a person decides", OWNER_BLUE),
    ("the owner answers", "hatch"),
    ("no check yet", None),
)


def segment_fill(colour):
    """The attribute that paints a chart part: the gray class for no check, the hatch for the owner's, else a hue."""
    if colour is None:
        return ' class="none"'
    return ' fill="url(#owner)"' if colour == "hatch" else f' fill="{colour}"'


def coverage_svg(title, rows):
    """A stacked bar per row of (label, credited, found failing only, supporting only, the owner answers, nothing),
    each bar as long as
    the row's requirements, with every part's count written beside it, so no part is shown by color alone."""
    def esc(text):
        return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
    label_w, bar_w, count_w, row_h, top = 300, 380, 230, 26, 92
    width = label_w + bar_w + count_w + 24
    height = top + row_h * len(rows) + 40
    most = max(sum(r[1:]) for r in rows)
    out = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" '
        f'height="{height}" role="img" aria-labelledby="t d" '
        'font-family="Atkinson Hyperlegible, -apple-system, Segoe UI, sans-serif">',
        f'<title id="t">{esc(title)}</title>',
        '<desc id="d">' + esc("; ".join(
            f"{r[0]}: {sum(r[1:])} requirements, "
            + ", ".join(f"{n} {name}" for (name, _), n in zip(SEGMENTS, r[1:]))
            for r in rows)) + ". The same counts are the table in docs/COVERAGE.md.</desc>",
        "<style>.bg{fill:#FFFFFF}.ink{fill:#14201C}.soft{fill:#3B4842}.none{fill:#EEF0EA;stroke:#C9CEC4}"
        "@media (prefers-color-scheme: dark){.bg{fill:#14201C}.ink{fill:#F3F4EF}.soft{fill:#C9CEC4}"
        ".none{fill:#182A23;stroke:#3B4842}}</style>",
        f'<defs><pattern id="owner" width="6" height="6" patternUnits="userSpaceOnUse" '
        f'patternTransform="rotate(45)"><rect class="none" width="6" height="6"/>'
        f'<rect width="2.5" height="6" fill="{OWNER_BLUE}"/></pattern></defs>',
        f'<rect class="bg" width="{width}" height="{height}"/>',
        f'<text class="ink" x="12" y="28" font-size="18" font-weight="700">{esc(title)}</text>',
    ]
    # The key, one line, each part's swatch beside its words.
    x = 12
    for name, colour in SEGMENTS:
        out.append(f'<rect{segment_fill(colour)} x="{x}" y="46" width="14" height="14" rx="2"/>')
        out.append(f'<text class="soft" x="{x + 20}" y="58" font-size="13">{esc(name)}</text>')
        x += 32 + 7 * len(name)
    out.append(f'<text class="soft" x="{label_w + bar_w + 12}" y="{top - 10}" font-size="12">'
               "counts, in the key's order</text>")
    for i, (label, *parts) in enumerate(rows):
        y = top + i * row_h
        out.append(f'<text class="ink" x="12" y="{y + 17}" font-size="13">{esc(label)}</text>')
        bx = label_w
        for (name, colour), n in zip(SEGMENTS, parts):
            if not n:
                continue
            w_ = bar_w * n / most
            out.append(f'<rect{segment_fill(colour)} x="{bx:.1f}" y="{y + 4}" width="{max(w_ - 2, 1):.1f}" height="{row_h - 8}" '
                       f'rx="2"><title>{esc(label)}: {n} {esc(name)}</title></rect>')
            bx += w_
        counts = " · ".join(str(n) for n in parts)
        out.append(f'<text class="soft" x="{label_w + bar_w + 12}" y="{y + 17}" font-size="13">'
                   f'{counts}  of {sum(parts)}</text>')
    out.append(f'<text class="soft" x="12" y="{height - 14}" font-size="12">Drawn by tools/coverage.py from the '
               "checks' own citations, with docs/COVERAGE.md; a bar is as long as its requirements.</text>")
    out.append("</svg>")
    return "\n".join(out) + "\n"


def load(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def owner_answers():
    """What StackVet asks the owner for each requirement, and where `sv` points them in the code (backlog 0624):
    {id: [how it is asked]} from the three catalogs `sv review` records answers from, and {id: [locator ids]} from
    data/locators.json. An answer is never *checked*: a section is *documented*, a design question *attested*, and a
    check by hand *checked by hand*. A Secure by Design control is the owner's by construction, so it is not listed."""
    asked = defaultdict(list)
    for section in load(ROOT / "data/security-notes.json")["sections"]:
        asked[section["id"]].append("a section in the security notes")
    for question in load(ROOT / "data/design-questions.json")["questions"]:
        asked[question["id"]].append("a design question")
    for check in load(ROOT / "data/human-checks.json")["checks"]:
        asked[check["id"]].append("a check by hand")
    located = defaultdict(list)
    locators = load(ROOT / "data/locators.json")
    for locator in locators["locators"] + locators["packages"]:
        for q in locator.get("questions", []):
            located[q].append(locator["id"])
    return asked, located


def framework(path):
    out = {}
    for chapter in load(path)["chapters"]:
        for section in chapter.get("sections", []):
            for r in section.get("requirements", []):
                out[r["id"]] = {
                    "level": r.get("level"),
                    "chapter": chapter["id"],
                    "chapter_name": chapter["name"],
                    "section": section["id"],
                    "section_name": section["name"],
                    "text": r.get("description", ""),
                }
    return out


# What a check of sv's own looks for, for the few whose code carries no title or impact of its own to
# read it from: checks that only ever credit a requirement, and the configuration checks built
# differently. Every other check's words are read from the code or data where the check is defined.
DESCRIBED = {
    "config.template-engine-escapes": "The template engine escapes what it writes by itself (Django, Jinja2 under Flask, React, Vue, EJS, Mustache), every outside-tool rule that reads it switched off ran and found nothing, and every language and package list in the app was read; in part, since HTML put together by hand and output to other interpreters are not seen",
    "ast.weak-key-generation": "A key the code makes with a method or a size no longer approved: an RSA key under 2048 bits, a DSA key, a key on a 192-bit curve, or an RSA public exponent of 3",
    "config.session-timeout-differs-from-policy": "A session lifetime the code sets (SESSION_COOKIE_AGE, PERMANENT_SESSION_LIFETIME, an Express session's maxAge) longer than the one stackvet.toml's policy states",
    "config.sign-in-pathway-not-listed": "A way into the app the code shows (a sign-in route, a Passport strategy, an API key read by header) that the list of sign-in pathways in stackvet.toml does not name",
    "config.certificate-checks-off": "A setting that switches off certificate checking for every connection the app makes",
    "config.firebase-rules-open": "A Firebase rules file that lets anybody in: an `allow` with no condition, `if true`, or test mode's date alone, or a Realtime Database `.read` or `.write` set to true",
    "config.supabase-table-without-rls": "A table a Supabase migration creates with no row-level security turned on, open to the key every visitor's browser holds",
    "config.supabase-policy-allows-all": "A Supabase policy that lets rows be added, changed, or deleted on the condition `true`",
    "config.secret-under-public-name": "A server's key under a name the build hands to the browser (`NEXT_PUBLIC_`, `VITE_`, `EXPO_PUBLIC_`, `REACT_APP_`): a name that says it holds a secret, or a value shaped like a service-role, Stripe, OpenAI, Anthropic, Supabase secret, or GitHub key",
    "config.workflow-runs-fork-code": "A CI workflow that runs code from a pull request by someone outside the project with the repository's privileges",
    "config.workflow-checkout-keeps-token": "A CI workflow whose checkout step leaves the repository token where later steps can read it",
    "config.workflow-secrets-with-fork-code": "A CI workflow that hands secrets to a job running code from outside the project",
    "secrets.password-in-url": "A password written into a web address's user part (`postgresql://user:password@host`), placeholders, references, and stock passwords set aside",
    "config.workflow-untrusted-text-in-run": "A CI workflow that pastes text a stranger writes (a pull request's title, a branch name, an issue) into a `run:` line, where it is read as commands",
    "config.workflow-hands-out-all-secrets": "A CI workflow that hands every secret to a job (`toJSON(secrets)`, `secrets: inherit`), or puts secrets in the workflow-wide `env:` where every step reads them",
    "sbom": "Whether the list of what the app ships (its bill of materials) could be read completely from its lockfiles",
    "advisories": "Every package the app ships, compared with a local copy of the OSV database of known vulnerabilities",
    "probe.ai-injection-logged": "Whether the app writes down that it caught the textbook prompt injection the run sent",
    "probe.authentication-logged": "Whether the app writes down a refused sign-in the run made",
    "probe.authorization-failure-logged": "Whether the app writes down a request it refused to someone not allowed to make it",
    "probe.bypass-attempt-logged": "Whether the app writes down a form sent as another site would send it, which it refused",
    "probe.log-line-metadata": "Whether the line recording a security event says when, where from, and who",
    "probe.log-common-format": "Whether that line is written in a format log tools read without being taught",
    "probe.clear-site-data": "Whether signing out tells the browser to clear what the site stored in it",
}


def rust_string(text, i):
    """The Rust string literal that starts at the quote at `text[i]`, with line continuations joined."""
    out, j = [], i + 1
    while True:
        ch = text[j]
        if ch == "\\":
            nxt = text[j + 1]
            if nxt == "\n":
                j += 2
                while text[j] in " \t":
                    j += 1
                continue
            if nxt == "u" and text[j + 2] == "{":
                # `\u{2014}`: the character it names, not the letters.
                end = text.index("}", j + 3)
                out.append(chr(int(text[j + 3:end], 16)))
                j = end + 1
                continue
            out.append({"n": "\n", '"': '"', "\\": "\\", "t": "\t"}.get(nxt, nxt))
            j += 2
            continue
        if ch == '"':
            return "".join(out)
        out.append(ch)
        j += 1


def rust_code():
    """(path, code) for every Rust file that ships: each file under a crate's `src`, subfolders
    included, with its tests cut off at the first `#[cfg(test)]`. A file that is a test module of
    its own, declared `#[cfg(test)] mod name;` by the file beside it (as `signed_in/fake_app.rs`
    is), is left out whole."""
    paths = sorted((ROOT / "crates").glob("*/src/**/*.rs"))
    test_only = set()
    for path in paths:
        folder = path.parent if path.name in ("mod.rs", "lib.rs", "main.rs") else path.with_suffix("")
        for m in re.finditer(r'#\[cfg\(test\)\]\s*(?:#\[path\s*=\s*"([^"]*)"\]\s*)?(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;',
                             path.read_text(encoding="utf-8")):
            # `#[path = "..."]` names the file, from the folder of the file that declares it (backlog 0618).
            if m.group(1):
                test_only.add((path.parent / m.group(1)).resolve())
                continue
            test_only.add(folder / f"{m.group(2)}.rs")
            test_only.add(folder / m.group(2) / "mod.rs")
    return [(p, shipped(p.read_text(encoding="utf-8"))) for p in paths if p not in test_only and p.resolve() not in test_only]


# `#[cfg(test)]` before a module of tests kept in a file of its own (`mod name;`, with or without
# `#[path = "..."]`) declares that file; it does not start the test code of this one.
TEST_MODULE_DECLARED = re.compile(r'#\[cfg\(test\)\]\s*(?:#\[path\s*=\s*"[^"]*"\]\s*)?(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;')


def shipped(code):
    """`code` up to its first `#[cfg(test)]` that is not a test module's declaration."""
    at = code.find("#[cfg(test)]")
    while at >= 0 and TEST_MODULE_DECLARED.match(code, at):
        at = code.find("#[cfg(test)]", at + 1)
    return code if at < 0 else code[:at]


def check_words():
    """sv's own check id -> (label, words): what it looks for, from where the check is defined."""
    words = {}
    for rule in load(ROOT / "data/ast-rules.json")["rules"]:
        words[rule["id"]] = ("Looks for", rule["title"])
    for rule in load(ROOT / "data/secret-rules.json")["rules"]:
        words[rule["id"]] = ("Looks for", rule["title"])
    code = "".join(code for _, code in rust_code())
    for m in re.finditer(r'rule_id:\s*"([^"]+)"', code):
        if m.group(1) in words:
            continue
        nxt = code.find("rule_id:", m.end())
        body = code[m.end():min(nxt if nxt != -1 else len(code), m.end() + 4000)]
        for field, label in (("title", "Looks for"), ("impact", "If it fails")):
            f = re.search(field + r':\s*"', body)
            if f:
                words[m.group(1)] = (label, rust_string(body, f.end() - 1))
                break
    # The credential-assignment rule's title and advice depend on whether the value reads like a
    # sentence, so its words are its `ASSIGNMENT_WHAT`, the constant the citation guard reads too.
    m = re.search(r'pub const ASSIGNMENT_WHAT: &str =\s*"', code)
    if m:
        words.setdefault("secrets.credential-assignment", ("Looks for", rust_string(code, m.end() - 1)))
    for check, text in DESCRIBED.items():
        words.setdefault(check, ("Looks for", text))
    unsaid = sorted(c for c in RUST_CHECKS if c not in words)
    if unsaid:
        sys.exit("checks with no words to show in docs/REQUIREMENTS.md; add them to DESCRIBED: "
                 + ", ".join(unsaid))
    extra = sorted(c for c in DESCRIBED if c not in RUST_CHECKS)
    if extra:
        sys.exit("DESCRIBED names checks RUST_CHECKS does not have: " + ", ".join(extra))
    return words


# Requirement id -> {tool: [what each of its rules looks for]}, for the outside tools.
TOOL_RULES = defaultdict(lambda: defaultdict(list))
# Requirement id -> {tool: the ids of its rules that speak to it}, to count rules rather than their
# descriptions, which many rules share.
TOOL_RULE_IDS = defaultdict(lambda: defaultdict(set))


def appendix_c(path):
    out = {}
    for family in load(path)["families"]:
        for section in family.get("sections") or [family]:
            for r in section.get("requirements", section.get("controls", [])):
                out[r["id"]] = {"chapter": family["id"], "chapter_name": family["name"]}
    return out


def rust_literals():
    found = defaultdict(set)
    for path, code in rust_code():
        for m in re.finditer(r'"(' + ID + r')"', code):
            found[m.group(1)].add(path.name)
    return found


# Requirement id -> the rules that can only ever raise it as a finding (`findings_against` in
# adapters.json), by their folder name. A clean run credits none of these.
# Requirement id -> {(tool, rule)}: rules that are only ever a finding against it.
FINDINGS_ONLY = defaultdict(set)
# Checks whose every credit is in part: the Rust ones listed above, and the AST rules marked `inPart`.
# V15.1.2 asks for an inventory and that its libraries come from trusted, continually maintained repositories:
# a pinned lockfile and a complete bill of materials are the inventory, and the repositories are not read
# (backlog 0307).
RUST_IN_PART |= {"config.versions-pinned", "sbom"}
# Backlog 0430: every other check whose each credit is marked in part where it is made (`.in_part()`, or
# `.in_part_if` on a condition that always holds for it), each with its reason beside that credit in the code.
# Until then they were printed as able to mark a requirement *checked*. `--credits` holds this list to the
# suite: a check it saw credit only in part must be here, and one it saw credit whole once must not.
RUST_IN_PART |= {
    # One other session ended, viewing not tried; one user's one session ended by the admin (backlog 0492).
    "probe.sessions-ended-without-password",
    "probe.admin-end-sessions-ignored",
    # One member's page, at the one address `read` names (backlog 0535).
    "probe.saved-page-rendered",
    # The engine's own escaping, with hand-built HTML and other interpreters unseen (backlog 0613).
    "config.template-engine-escapes",
    "config.gitignore-covers-env",
    "config.workflow-checkout-keeps-token",
    "config.secrets-file-committed",
    "probe.action-done-twice",
    "probe.ai-agent-unbounded",
    "probe.ai-call-log-incomplete",
    "probe.ai-call-log-session",
    "probe.ai-flagged-reply-shown",
    "probe.ai-hidden-content-passed",
    "probe.ai-injection-logged",
    "probe.ai-injection-unscreened",
    "probe.ai-instructions-leaked",
    "probe.ai-mcp-injection-unscreened",
    "probe.ai-mcp-output-unvalidated",
    "probe.ai-output-shape-unchecked",
    "probe.ai-output-unbounded",
    "probe.ai-rate-unlimited",
    "probe.ai-service-failure-handled",
    "probe.ai-tool-reads-others-records",
    "probe.app-stopped-during-questions",
    "probe.app-token-alg-none",
    "probe.app-token-expired-accepted",
    "probe.app-token-signature-not-checked",
    "probe.archive-unchecked",
    "probe.breached-password-accepted",
    "probe.clear-site-data",
    "probe.common-password-accepted",
    "probe.context-word-password-accepted",
    "probe.cookie-without-host-prefix",
    "probe.cors-any-origin",
    "probe.create-rate-unlimited",
    "probe.download-name-injected",
    "probe.download-unnamed",
    "probe.email-code-guessing-unlimited",
    "probe.email-code-long-lived",
    "probe.email-code-reusable",
    "probe.email-code-unbound",
    "probe.error-detail-leak",
    "probe.failed-sign-ins-unlimited",
    "probe.fetch-follows-redirect",
    "probe.file-contents-unchecked",
    "probe.graphql-introspection",
    "probe.graphql-no-amount-limit",
    "probe.logout-keeps-session",
    "probe.mcp-server-origin-unchecked",
    "probe.mcp-server-takes-unknown-or-oversized-arguments",
    "probe.mcp-server-takes-wrong-types",
    "probe.mcp-server-token-unchecked",
    "probe.mcp-session-survives-end",
    "probe.no-hsts",
    "probe.oidc-access-token-audience-not-checked",
    "probe.oidc-audience-not-checked",
    "probe.oidc-id-token-taken-as-access-token",
    "probe.oidc-issuer-not-checked",
    "probe.oidc-logout-token-not-checked",
    "probe.oidc-nonce-not-checked",
    "probe.oidc-sign-in-from-another-session",
    "probe.oidc-signature-not-checked",
    "probe.oidc-user-keyed-on-email",
    "probe.other-users-data",
    "probe.oversized-file-accepted",
    "probe.password-altered",
    "probe.password-change-ends-sessions",
    "probe.password-change-notified",
    "probe.password-change-without-current",
    "probe.password-composition-rules",
    "probe.password-field-unmasked",
    # A refused port 80 is in part too since backlog 0466, as the permanent redirect was (ADR-090, Later).
    "probe.plain-http-served",
    "probe.private-files-served",
    "probe.retired-grants-offered",
    "probe.session-cookie-attributes",
    "probe.session-idle-timeout",
    "probe.session-lifetime",
    "probe.session-not-renewed",
    "probe.session-token-static",
    "probe.sessions-survive-deletion",
    "probe.short-password-accepted",
    "probe.sign-out-control-hidden",
    "probe.source-control-exposed",
    "probe.storage-kept-after-sign-out",
    "probe.totp-old-code-accepted",
    "probe.totp-reused",
    "probe.trace-enabled",
    "probe.upload-not-scanned",
    "probe.upload-path-traversal",
    "probe.uploaded-svg-keeps-script",
}

IN_PART = set(RUST_IN_PART)
# Requirement id -> {tool}: tools with some rule a clean run credits to it.
CREDITED_BY_TOOL = defaultdict(set)
# Requirement id -> {language}: the languages the tool rules that credit it read; `*` for any (backlog 0471).
TOOL_LANGUAGES = defaultdict(set)
# Requirement id -> {rule}: semgrep rules in the map that name it, in no pack the adapter runs.
NOT_RUN = defaultdict(set)


# Requirement id -> {rule}: semgrep rules a pack loads that read only files `sv` never hands semgrep.
NEVER_HANDED = defaultdict(set)


def code_extensions():
    """The file extensions `sv` reads as code (`language_of` in sv-scan's ecosystems.rs): the only
    files it hands semgrep."""
    text = (ROOT / "crates/sv-scan/src/ecosystems.rs").read_text(encoding="utf-8")
    body = text[text.index("pub fn language_of"):]
    body = body[: body.index("_ => return None")]
    # An arm may run over several lines, as rustfmt wraps a long one.
    return {e for arm in re.findall(r'((?:"[^"\n]+"\s*\|\s*)*"[^"\n]+")\s*=>', body)
            for e in re.findall(r'"([^"]+)"', arm)}


def reads_code(rule, extensions):
    """Whether a rule that names the files it reads (`targets`, its `paths.include`) can be handed
    one by `sv`. A sample name is made from each pattern, as its test does: a rule for `*.erb` or
    `*.conf` never is, one for `*.html` or `*session.php` can be."""
    targets = rule.get("targets") or []
    if not targets:
        return True
    samples = [g.replace("**/", "").replace("*", "x").replace("?", "x") for g in targets]
    return any("." in s.rsplit("/", 1)[-1] and s.rsplit(".", 1)[-1] in extensions for s in samples)


# How a condition reads in "run unless the app is known not to …".
CONDITION_WORDS = {"ai": "call a model"}


def semgrep_packs(adapter):
    """Pack -> the condition it runs for, or None for every app. A pack in `conditional_args` is left
    out of a run only for an app known not to meet its condition."""
    out = {}
    for args, condition in [(adapter["run"]["args"], None)] + [
            (c["args"], c["condition"]) for c in adapter.get("conditional_args", [])]:
        for i, a in enumerate(args[:-1]):
            if a == "--config":
                out[args[i + 1]] = condition
    return out


def semgrep_loaded(adapter):
    """The semgrep rules the adapter's packs load, as measured in data/semgrep-packs.json.

    The map knows about a thousand rules; the adapter runs registry packs, and a pack loads only some
    of them. A rule no pack loads is never run, so it is evidence of nothing, whatever the map says.
    """
    packs = list(semgrep_packs(adapter))
    measured = load(ROOT / "data/semgrep-packs.json")["packs"]
    unmeasured = [p for p in packs if p not in measured]
    if unmeasured:
        sys.exit(
            "the semgrep adapter runs " + ", ".join(unmeasured) + ", which data/semgrep-packs.json has "
            "not measured; run tools/semgrep_packs.py"
        )
    return {rule for p in packs for rule in measured[p]["rules"]}


def evidence():
    """Requirement id -> {tier: [check, ...]}."""
    ev = defaultdict(lambda: defaultdict(list))
    for rule in load(ROOT / "data/ast-rules.json")["rules"]:
        for q in rule["requirementIds"]:
            ev[q]["static"].append(rule["id"])
            if rule.get("findingsOnly") or q in rule.get("findingsOnlyFor", []):
                FINDINGS_ONLY[q].add(("sv", rule["id"]))
        # A requirement cited only on a narrower match (`alsoCites`) is never credited by a clean run (backlog 0507).
        for cite in rule.get("alsoCites", []):
            for q in cite["requirementIds"]:
                ev[q]["static"].append(rule["id"])
                FINDINGS_ONLY[q].add(("sv", rule["id"]))
        if rule.get("inPart"):
            IN_PART.add(rule["id"])
    for rule in load(ROOT / "data/secret-rules.json")["rules"]:
        for q in rule["requirementIds"]:
            ev[q]["static"].append(rule["id"])
    for adapter in load(ROOT / "data/adapters.json")["adapters"]:
        # A tool the owner turns on is counted apart, so the counts say what a default run reaches (ADR-115).
        tier = "opt-in" if adapter.get("opt_in") else "tools"
        loaded = semgrep_loaded(adapter) if adapter["id"] == "semgrep" else None
        for rule_id in [r for r in adapter["rules"] if loaded is not None and r not in loaded]:
            rule = adapter["rules"][rule_id]
            for q in rule["requirements"] + rule.get("findings_against", []):
                NOT_RUN[q].add(rule_id)
        rules = {k: v for k, v in adapter["rules"].items() if loaded is None or k in loaded}
        # A tool of every language is handed, besides the code, each file a rule of its names
        # (`handed_files` in adapters.rs; gap analysis, item 34), so none of its rules is never
        # handed one. A tool of one language is handed only that language's code.
        extensions = code_extensions()
        for rule_id in [r for r, rule in rules.items()
                        if adapter["language"] != "*" and not reads_code(rule, extensions)]:
            for q in rules[rule_id]["requirements"] + rules[rule_id].get("findings_against", []):
                NEVER_HANDED[q].add(rule_id)
            del rules[rule_id]
        for rule_id, rule in rules.items():
            for q in rule["requirements"]:
                if adapter["id"] not in ev[q][tier]:
                    ev[q][tier].append(adapter["id"])
                CREDITED_BY_TOOL[q].add(adapter["id"])
                if tier == "tools":
                    TOOL_LANGUAGES[q] |= (set(rule.get("languages") or ["*"]) if adapter["language"] == "*"
                                          else set(adapter["language"].split(",")))
                TOOL_RULE_IDS[q][adapter["id"]].add(rule_id)
                if rule.get("what"):
                    TOOL_RULES[q][adapter["id"]].append(rule["what"])
        for rule_id, rule in rules.items():
            for q in rule.get("findings_against", []):
                if adapter["id"] not in ev[q][tier]:
                    ev[q][tier].append(adapter["id"])
                TOOL_RULE_IDS[q][adapter["id"]].add(rule_id)
                if rule.get("what"):
                    TOOL_RULES[q][adapter["id"]].append(rule["what"])
                # The AI rules by their folder, which names the family across vendors and
                # languages; the rest by their own id.
                parts = rule_id.split(".")
                FINDINGS_ONLY[q].add((adapter["id"], parts[2] if parts[0] == "ai" else parts[-1]))
    for check, (tier, ids) in RUST_CHECKS.items():
        for q in ids:
            ev[q][tier].append(check)
            if check in RUST_FINDINGS_ONLY:
                FINDINGS_ONLY[q].add(("sv", check))
    return ev


PROMPT_STATUSES = {"shown", "not-shown", "untested"}


PROMPT_FILES = ("prompts.json", "design-prompts.json")


def check_prompts(known_requirements, sbd_controls):
    """Holds each prompt in `data/prompts.json` to the rules it names as its check.

    A prompt claims requirements, and names the rules whose result shows whether it worked. A claim
    is only as good as the check behind it, so every requirement a prompt names has to be cited by
    one of its rules, and every rule has to cite one of the prompt's requirements. A rule ending in
    `.` names a family (`secrets.`), and each rule in it is held to the same.
    """
    cites = {}
    for name in ("ast-rules.json", "secret-rules.json"):
        for rule in load(ROOT / "data" / name)["rules"]:
            cites[rule["id"]] = set(rule["requirementIds"]).union(
                *(c["requirementIds"] for c in rule.get("alsoCites", [])))
    for check, (_, ids) in RUST_CHECKS.items():
        cites[check] = set(ids)
    faults = []
    prompts = []
    for name in PROMPT_FILES:
        held = load(ROOT / "data" / name)["prompts"]
        if not held:
            faults.append(f"data/{name} holds no prompts")
        prompts += held
    seen = Counter(p["id"] for p in prompts)
    faults += [f"{pid}: the id is used {n} times across {', '.join(PROMPT_FILES)}"
               for pid, n in seen.items() if n > 1]
    for p in prompts:
        pid = p["id"]
        for c in p.get("sbd_controls", []):
            if c not in sbd_controls:
                faults.append(f"{pid}: {c} is not a control in the Secure by Design checklist")
        wanted = set(p["requirements"])
        if p.get("status") not in PROMPT_STATUSES:
            faults.append(f"{pid}: status {p.get('status')!r} is not one of {sorted(PROMPT_STATUSES)}")
        if p.get("status") in ("shown", "not-shown") and not (p.get("tested") or {}).get("result"):
            faults.append(f"{pid}: says it was tried, and `tested` does not say what happened")
        for q in sorted(wanted - known_requirements):
            faults.append(f"{pid}: {q} is not a requirement in ASVS 5.0 or AISVS 1.0")
        rules = p["check"].get("rules", [])
        # A rule that ran and did not show the prompt working stays listed, with the requirement it
        # cites set aside and the reason said, rather than claimed.
        set_aside = p["check"].get("not_claimed", {})
        for q, why in set_aside.items():
            if q in wanted:
                faults.append(f"{pid}: {q} is both claimed and set aside")
            if len(why.strip()) < 30:
                faults.append(f"{pid}: {q} is set aside without saying why")
        if wanted and not rules:
            faults.append(f"{pid}: names requirements and no rule that could show the prompt working")
        covered = set()
        for rule in rules:
            matched = [r for r in cites if r.startswith(rule)] if rule.endswith(".") else (
                [rule] if rule in cites else [])
            if not matched:
                faults.append(f"{pid}: the rule {rule} is not one sv has")
            for r in matched:
                if not cites[r] & (wanted | set(set_aside)):
                    faults.append(f"{pid}: the rule {r} cites {', '.join(sorted(cites[r]))}, "
                                  f"none of the prompt's {', '.join(sorted(wanted)) or 'requirements (it names none)'}")
                covered |= cites[r]
        for q in sorted(set(set_aside) - covered):
            faults.append(f"{pid}: sets aside {q}, which none of its rules cites")
        for q in sorted(wanted - covered):
            faults.append(f"{pid}: names {q}, which none of its rules cites")
    return faults


def check_credits(log):
    """Holds the findings-only lists to what the test suite saw each check credit.

    Reading the code for where a check gives credit is not reliable: it does so through helpers and
    tables of rules as often as by name. So `Verified::new`, in a debug build with `SV_CREDIT_LOG`
    set, writes each credit to a file, with the place in the code that gave it, and this reads it.
    A credit made in a test module, or in a test of its own, is a test building its own evidence and
    is left out. Faults: a check listed as findings-only that was credited; a check never credited
    that is not listed, which is either findings-only or a credit no test reaches; a credit naming a
    requirement the check does not cite; a requirement a crediting check cites that it was never
    seen crediting; and a check whose every credit was in part that is not listed as only ever in part,
    or one listed so that was seen credit whole (backlog 0430).
    """
    shipping = shipping_lines()
    credited = defaultdict(set)
    # Per check: credits made, marked in part, and made whole again (`Verified::in_part`,
    # `Verified::whole_after_all`, which write a line of four). Each mark follows the credit it marks.
    made, marked, unmarked = Counter(), Counter(), Counter()
    lines = 0
    for line in Path(log).read_text(encoding="utf-8").splitlines():
        parts = line.split("\t")
        check, ids, at = parts[0], parts[1], parts[-1]
        path, number = logged_place(at)
        if shipping.get(path, 0) < int(number):
            continue
        # A credit that names no requirement can make none *checked*, so it says nothing about whether
        # the check is only ever in part (backlog 0469: the checkout check's credit for a workflow that
        # checks nothing out).
        if len(parts) == 4:
            if ids:
                (marked if parts[2] == "in part" else unmarked)[check] += 1
            continue
        lines += 1
        if ids:
            made[check] += 1
        credited[check].update(q for q in ids.split(",") if q)
    if not lines:
        return [f"{log} holds no credit from a check: was the suite run with SV_CREDIT_LOG set?"]
    cites = {check: (set(ids), check in RUST_FINDINGS_ONLY) for check, (_, ids) in RUST_CHECKS.items()}
    listed_in_part = set(RUST_IN_PART)
    for rule in load(ROOT / "data/ast-rules.json")["rules"]:
        # A requirement a clean run never credits (`findingsOnlyFor`) is not cited as a credit: the suite
        # seeing it credited is a fault, as for a findings-only check.
        cites[rule["id"]] = (set(rule["requirementIds"]) - set(rule.get("findingsOnlyFor", [])),
                             bool(rule.get("findingsOnly")))
        if rule.get("inPart"):
            listed_in_part.add(rule["id"])
    faults = []
    # Backlog 0430: the documents print a check as able to mark a requirement *checked* unless it is listed
    # as only ever in part, so the list is held to the suite. A check whose every credit the suite saw was
    # in part belongs on it; one the suite saw credit whole even once does not (`in_part_if`, which is in
    # part only on a single sample, is whole when the suite gives it more).
    for check in sorted(made):
        if check not in cites:
            continue
        whole = made[check] - marked[check] + unmarked[check]
        if whole <= 0 and check not in listed_in_part:
            faults.append(f"{check}: every one of its {made[check]} credits the suite saw was in part, and the "
                          "documents print it as able to mark a requirement checked: add it to RUST_IN_PART "
                          "(or `inPart` in data/ast-rules.json)")
        if whole > 0 and check in listed_in_part:
            faults.append(f"{check} is listed as only ever in part, and the suite saw {whole} of its credits "
                          "whole: take it off the list, or mark the credit in part")
    for check, (ids, findings_only) in sorted(cites.items()):
        got = credited.get(check, set())
        if findings_only and got:
            faults.append(f"{check} is listed as only ever a finding, and the suite saw it credit "
                          + ", ".join(sorted(got)))
        if not findings_only and not got:
            faults.append(f"{check} was never credited in the suite: list it as findings-only, or add "
                          "a test that reaches its credit")
        if got - ids:
            faults.append(f"{check} credited {', '.join(sorted(got - ids))}, which it does not cite")
        # Per requirement, not only per check (the review of 8 October 2026, item 5): a check that cites
        # two requirements and was only ever seen crediting one passed, with the other cited on no evidence.
        if not findings_only and got and ids - got:
            faults.append(f"{check} cites {', '.join(sorted(ids - got))} and the suite never saw it credit "
                          "that: add a test that reaches the credit, or stop citing it")
    faults += check_withheld(log)
    return faults


# Checks that give credit and cannot be made, in a test, to withhold it, each with its reason. Empty: every
# check the suite sees credit was seen withholding (backlog item 32, ADR-059). A check added here is one the
# suite cannot show working; the reason says why, and it is expected to stay short.
NEVER_WITHHELD = {}


def check_withheld(log):
    """ADR-059: every check the suite saw credit was also seen withholding it, by a finding
    (`finding::found`) or through `verified::unless_credited`, in code that ships; unless it is listed in
    `NEVER_WITHHELD` with its reason. A listed one that was seen withholding is listed for nothing."""
    if not Path(str(log) + ".withheld").exists():
        return [f"{log}.withheld is not there: the census of what checks withhold is written beside the credits "
                "in a debug build (finding::found, verified::unless_credited); was the suite run in one?"]
    _, seen, never = withheld_report(log)
    faults = []
    for check in never:
        if check not in NEVER_WITHHELD:
            faults.append(f"{check} gives credit and was never seen withholding it: add a test in which what "
                          "it looks for is missing and it says no (a finding, or verified::unless_credited "
                          "where it has run), or list it in NEVER_WITHHELD with the reason it cannot")
    for check in sorted(NEVER_WITHHELD):
        if check in seen:
            faults.append(f"{check} is in NEVER_WITHHELD, and the suite saw it withhold: take it off the list")
    return faults


def shipping_lines():
    """Each Rust file that ships, by its path from the repository with `/` on every system, and how
    many lines it has. The census logs name a place in the code as Rust's own `file!()` does, which
    on Windows is written with `\\`; both sides are compared with `/` (backlog 0120). Until then, on
    Windows, no logged place matched a file, and every log read as holding no credit."""
    return {p.relative_to(ROOT).as_posix(): code.count("\n") + 1 for p, code in rust_code()}


def logged_place(at):
    """`file:line` from a census log, as (`file` with `/`, line)."""
    path, number = at.rsplit(":", 1)
    return path.replace("\\", "/"), number


def unrecorded_findings():
    """Every place in the code that ships that builds a `Finding` hands it through `finding::found`, so the
    census sees it (ADR-059). Returns "file:line" for each that does not."""
    out = []
    for path, code in rust_code():
        for m in re.finditer(r"(?<![\w:])((?:crate::|sv_check::)?(?:finding::)?Finding) \{", code):
            before = code[max(0, m.start() - 40):m.start()]
            if re.search(r"(struct|impl|->)\s*$", before) or re.search(r"found\(\s*$", before):
                continue
            out.append(f"{path.relative_to(ROOT).as_posix()}:{code[:m.start()].count(chr(10)) + 1}")
    return out


# A check whose findings carry names of their own, one per thing found, rather than the check's: each
# finding named with the prefix is that check withholding.
WITHHELD_UNDER = {"advisory.": "advisories", "sbom.": "sbom"}


def withheld_report(log):
    """The checks the suite saw give credit and never saw withhold it (backlog item 32, step 1).

    A check known to work says no when the thing it guards is broken. `Verified::new` writes each credit
    to `log`; `finding::found` writes each finding a check makes to `log` + `.withheld`, with the place
    in the code that made it. A finding made in a test module, or in a test of its own, is a test
    building its own finding and is left out, as a credit is. Returns (credited, seen withholding,
    never seen withholding), each a sorted list of check ids, for the checks this script knows (the
    Rust checks and the tree-sitter rules). Fails nothing: it measures.
    """
    shipping = shipping_lines()

    def read(path, columns):
        seen = set()
        for line in Path(path).read_text(encoding="utf-8").splitlines():
            parts = line.split("\t")
            if len(parts) != columns:
                continue
            at = parts[-1]
            source, number = logged_place(at)
            if shipping.get(source, 0) < int(number):
                continue
            check = parts[0]
            for prefix, owner in WITHHELD_UNDER.items():
                if columns == 2 and check.startswith(prefix):
                    check = owner
            seen.add(check)
        return seen

    credited = read(log, 3)
    withheld_log = Path(str(log) + ".withheld")
    withheld = read(withheld_log, 2) if withheld_log.exists() else set()
    known = {check for check, _ in RUST_CHECKS.items() if check not in RUST_FINDINGS_ONLY}
    known |= {r["id"] for r in load(ROOT / "data/ast-rules.json")["rules"] if not r.get("findingsOnly")}
    crediting = sorted(credited & known)
    return crediting, sorted(set(crediting) & withheld), sorted(set(crediting) - withheld)


def reach_json(asvs, aisvs, ev, tiers, settles, checks=None):
    """Requirement id -> the kinds of run (`TIERS` ids) with a check that can credit it, for every
    requirement a check can settle. A check that is only ever a finding is left out: it can show a
    requirement failing, never met, so it is not what a run that did not happen would have added."""
    def credits(q, tier, check):
        if is_tool(tier, check):
            return check in CREDITED_BY_TOOL.get(q, set())
        return ("sv", check) not in FINDINGS_ONLY.get(q, set())
    reach = {}
    for q in sorted(set(asvs) | set(aisvs), key=lambda q: (q[0], [int(x) for x in re.findall(r"\d+", q)])):
        if not settles(q):
            continue
        kinds = [t for t in tiers(q) if any(credits(q, t, c) for c in ev[q][t])]
        if kinds:
            reach[q] = kinds
    return json.dumps({
        "_comment": "Generated by tools/coverage.py; do not edit. For each requirement a check can "
                    "settle, the kinds of run with a check that can credit it: static (plain sv check), "
                    "advisories, running (--run), signed-in (--run with test accounts), tools (sv report, unless --no-tools), "
                    "production (sv probe). Read by the report's short version. `checks` gives, for "
                    "every requirement any check speaks to, each check, the kind of run it needs, what "
                    "it looks for, and whether it is only ever a finding; read by `sv explain`. "
                    "`tools_languages` gives, for each requirement an outside tool can credit, the "
                    "languages those tools' crediting rules read (`*` for any), so the short version "
                    "counts a tool's reach only for an app in one of them.",
        "requirements": reach,
        "tools_languages": {q: sorted(TOOL_LANGUAGES[q]) for q in reach if "tools" in reach[q]},
        "checks": checks or {},
    }, indent=1) + "\n"


def explain_checks(rows):
    """Requirement id -> what `sv explain` says about each check that speaks to it: its id, the kind of
    run it needs, what it looks for (cut to one sentence's length), and whether it is only ever a
    finding, so it can show a requirement failing but never met."""
    out = {}
    for row in rows:
        if not row["checks"]:
            continue
        out[row["id"]] = [{
            "id": c["id"], "run": c["kind"],
            "looks_for": (c["words"][:237] + "...") if len(c["words"]) > 240 else c["words"],
            "finding_only": bool(c["finding_only"]),
        } for c in row["checks"]]
    return out


def main():
    if "--withheld" in sys.argv[1:]:
        log = sys.argv[sys.argv.index("--withheld") + 1]
        crediting, seen, never = withheld_report(log)
        if not crediting:
            sys.exit(f"{log} holds no credit from a check: was the suite run with SV_CREDIT_LOG set?")
        if not Path(log + ".withheld").exists():
            sys.exit(f"{log}.withheld is not there: was the suite run with SV_CREDIT_LOG set, in a debug build?")
        print(f"{len(crediting)} checks were seen giving credit; {len(seen)} of them were also seen "
              f"withholding it, and {len(never)} never were:")
        for check in never:
            print(f"  {check}")
        return
    if "--credits" in sys.argv[1:]:
        faults = check_credits(sys.argv[sys.argv.index("--credits") + 1])
        if faults:
            sys.exit("the suite's credits disagree with tools/coverage.py:\n  " + "\n  ".join(faults))
        print("every check credited as the lists say")
        return
    check = "--check" in sys.argv[1:]

    written = rust_literals()
    known = {q for _, ids in RUST_CHECKS.values() for q in ids} | MENTIONS
    missing = {q: sorted(files) for q, files in written.items() if q not in known}
    if missing:
        sys.exit(
            "requirement ids written into sv's code that tools/coverage.py does not know about; add the "
            "check to RUST_CHECKS, or the id to MENTIONS if it is not evidence:\n  "
            + "\n  ".join(f"{q} in {', '.join(f)}" for q, f in sorted(missing.items()))
        )
    code = "".join(code for _, code in rust_code())
    unnamed = sorted(c for c in RUST_CHECKS if f'"{c}"' not in code)
    if unnamed:
        sys.exit("RUST_CHECKS names checks the code does not have: " + ", ".join(unnamed))
    stale = sorted(q for _, ids in RUST_CHECKS.values() for q in ids if q not in written)
    stale += sorted(q for q in MENTIONS if q not in written)
    if stale:
        sys.exit("RUST_CHECKS or MENTIONS names ids no longer in the code: " + ", ".join(stale))

    asvs = framework(ROOT / "data/frameworks/asvs-5.0.0.json")
    aisvs = framework(ROOT / "data/frameworks/aisvs-1.0.json")
    sbd_controls = {f"SBD-{c['id']}" for d in load(ROOT / "data/frameworks/sbd-checklist-0.5.0.json")["checklistDomains"]
                    for c in d["controls"]}
    unrecorded = unrecorded_findings()
    if unrecorded:
        sys.exit("a finding is built without going through finding::found, so the census of what each check "
                 "withholds cannot see it (ADR-059); wrap it, `crate::finding::found(Finding { .. })`:\n  "
                 + "\n  ".join(unrecorded))
    faults = check_prompts(set(asvs) | set(aisvs), sbd_controls)
    if faults:
        sys.exit("the prompt library claims what its checks do not cite:\n  " + "\n  ".join(faults))
    appendix = appendix_c(ROOT / "data/frameworks/aisvs-1.0-appendix-c.json")
    sbd = load(ROOT / "data/frameworks/sbd-checklist-0.5.0.json")
    crosswalk = load(ROOT / "data/sbd-asvs-crosswalk.json")["controls"]
    manual_only = set(load(ROOT / "data/knowledge/applicability.json")["manualOnly"])
    ev = evidence()

    def tiers(q):
        return [t for t, _, _ in TIERS if ev.get(q, {}).get(t)]

    def settles(q):
        return bool(tiers(q)) and q not in manual_only and not q.startswith("SBD-")

    def supports_only(q):
        return bool(tiers(q)) and not settles(q)

    asked, _ = owner_answers()

    def owner_only(q):
        """No check speaks to q, and the owner is asked it: a catalog entry, or a Secure by Design control."""
        return not tiers(q) and (q in asked or q.startswith("SBD-"))

    def credited(q):
        """A check can mark q *checked*, not only *needs attention* (gap analysis 1.8): one of `sv`'s
        own that is not only ever a finding, or a tool with some rule a clean run credits to q."""
        sv_only = {r for tool, r in FINDINGS_ONLY.get(q, ()) if tool == "sv"}
        return settles(q) and any(
            c in CREDITED_BY_TOOL[q] if is_tool(tier, c) else c not in sv_only
            for tier, checks in ev[q].items() for c in checks)

    def credited_whole(q):
        """A check can mark q *checked*, not only *checked in part*: a crediting check of `sv`'s that is not
        only ever in part, or a tool with a rule a clean run credits to q (backlog 0470)."""
        sv_only = {r for tool, r in FINDINGS_ONLY.get(q, ()) if tool == "sv"}
        return settles(q) and any(
            c in CREDITED_BY_TOOL[q] if is_tool(tier, c) else (c not in sv_only and c not in IN_PART)
            for tier, checks in ev[q].items() for c in checks)

    out = []
    w = out.append

    w("# Coverage: what the checks can speak to\n")
    w("Generated by `tools/coverage.py` from the checks' own citations. Do not edit by hand; run")
    w("`python3 tools/coverage.py` after changing a check, and the build fails until you do.\n")
    w("The ASVS tables by chapter and by level below, drawn as bars from the same counts:\n")
    w("![ASVS 5.0 by chapter: what a check can reach](coverage-by-chapter.svg)\n")
    w("![ASVS 5.0 by level: what a check can reach](coverage-by-level.svg)\n")
    w("## How to read this\n")
    w("- **Can settle**: at least one check can mark the requirement *checked* or *needs attention*.")
    w("  A check is almost always about part of a requirement: a clean result is one automated check")
    w("  that was satisfied, not a pass.")
    w("- **Can be credited**: of those, the ones a check can mark *checked* or *checked in part*. The rest")
    w("  can only ever be found failing: a check can show the control missing, and finding nothing does not")
    w("  show it present, so a clean run credits nothing for them. Some can only ever be *checked in part*,")
    w("  because every check that credits them tries part of what they ask; the sentence under the level")
    w("  table counts them, and docs/REQUIREMENTS.md marks each \"(only ever in part)\".")
    w("- **Supporting only**: a check speaks to it, but the requirement asks something no check can")
    w("  answer, such as a documented policy or a design decision. The check is shown beside it and")
    w("  a person still has to answer it. No Secure by Design control is ever settled by a check: each")
    w("  is answered by a person, and those no check speaks to at all are counted under *The owner answers*.")
    w("- **The owner answers**: no check speaks to it, and `sv review` asks the owner for it: a section in the")
    w("  security notes (*documented*), a design question (*attested*), or a check by hand (*checked by hand*).")
    w("  An answer is never *checked*. docs/REQUIREMENTS.md says which, and where `sv` shows the owner the code.")
    w("- **No check yet**: nothing in `sv` speaks to it. The app's own tests that name the requirement id in")
    w("  code and pass, which is how `sv init` asks for tests to be written, give it a tier of their own,")
    w("  *tested by the app's own tests*, below *checked* (ADR-050); otherwise it stays *not verified*.")
    w("- Which requirements apply to a given app is decided separately; this counts every requirement.")
    w("")
    w("What each kind of check needs before it can run:\n")
    w("| Kind | Needs |")
    w("|---|---|")
    for _, name, needs in TIERS:
        w(f"| {name} | {needs} |")
    w("")

    # ---- summary
    def sbd_supported(q):
        return bool(tiers(q)) or any(tiers(a) for a in crosswalk.get(q, {}))

    def summary_row(name, reqs):
        n = len(reqs)
        s = sum(settles(q) for q in reqs)
        p = sum(supports_only(q) or (q.startswith("SBD-") and sbd_supported(q)) for q in reqs)
        c = sum(credited(q) for q in reqs)
        o = sum(owner_only(q) and not (q.startswith("SBD-") and sbd_supported(q)) for q in reqs)
        pct = lambda k: f"{100 * k / n:.0f}%" if n else "–"
        return f"| {name} | {n} | {s} ({pct(s)}) | {c} ({pct(c)}) | {p} | {o} | {n - s - p - o} |"

    sbd_ids = [f"SBD-{c['id']}" for d in sbd["checklistDomains"] for c in d["controls"]]
    w("## Summary\n")
    w("| Framework | Requirements | Can settle | Can be credited | Supporting only | The owner answers | No check yet |")
    w("|---|---|---|---|---|---|---|")
    w(summary_row("OWASP ASVS 5.0", list(asvs)))
    w(summary_row("OWASP AISVS 1.0", list(aisvs)))
    w(summary_row("AISVS Appendix C", list(appendix)))
    w(summary_row("Secure by Design checklist 0.5.0", sbd_ids))
    w("")

    # ---- ASVS by level and kind
    w("## ASVS 5.0 by level\n")
    w("A requirement reached by more than one kind of check is counted under each.\n")
    head = " | ".join(name for _, name, _ in TIERS)
    w(f"| Level | Requirements | Can settle | Can be credited | {head} |")
    w("|---|---|---|---|" + "---|" * len(TIERS))
    for level in sorted({v["level"] for v in asvs.values()}):
        reqs = [q for q, v in asvs.items() if v["level"] == level]
        cells = " | ".join(
            str(sum(1 for q in reqs if settles(q) and t in tiers(q))) for t, _, _ in TIERS
        )
        w(f"| L{level} | {len(reqs)} | {sum(settles(q) for q in reqs)} | "
          f"{sum(credited(q) for q in reqs)} | {cells} |")
    w("")
    settled_asvs = [q for q in asvs if settles(q)]
    found_only = [q for q in settled_asvs if not credited(q)]
    w(f"{len(found_only)} of the {len(settled_asvs)} ASVS requirements that can be settled can only ever be "
      "marked *needs attention*: a check can show the control missing, and finding nothing does not show "
      "it present, so a clean run credits none of them. They are counted under *Can settle* and not under "
      "*Can be credited*, and the kinds of check above count every requirement a check can settle either "
      "way.\n")
    credited_asvs = [q for q in asvs if credited(q)]
    part_only = [q for q in credited_asvs if not credited_whole(q)]
    w(f"{len(part_only)} of the {len(credited_asvs)} ASVS requirements that can be credited can only ever be "
      "*checked in part*: every check that credits them tries part of what they ask, so the most a clean run "
      "gives them is *checked in part*, never *checked*.\n")
    plain = [q for q in asvs if settles(q) and tiers(q) == ["static"]]
    only_tools = [q for q in asvs if settles(q) and set(tiers(q)) <= set(TOOL_TIERS)]
    only_opt_in = [q for q in asvs if settles(q) and tiers(q) == ["opt-in"]]
    w(f"With nothing beyond plain `sv check`, {sum(settles(q) and 'static' in tiers(q) for q in asvs)} "
      f"ASVS requirements can be settled. {len(only_tools)} can be settled only by an outside tool, "
      "almost all by semgrep and CodeQL, and only for the languages their rules are written for. Of those, "
      f"{len(only_opt_in)} only CodeQL reaches ({', '.join(only_opt_in)}), and CodeQL runs only when "
      "stackvet.toml turns it on (`[tools] codeql = true`), for a use its license allows; without it, they "
      "stay not verified.\n")

    # ---- AISVS by level, so a count on the ASVS basis (levels 1 and 2) can be read for AISVS too (backlog 0433)
    w("## AISVS 1.0 by level\n")
    w("| Level | Requirements | Can settle | Can be credited |")
    w("|---|---|---|---|")
    for level in sorted({v["level"] for v in aisvs.values()}):
        reqs = [q for q, v in aisvs.items() if v["level"] == level]
        w(f"| L{level} | {len(reqs)} | {sum(settles(q) for q in reqs)} | {sum(credited(q) for q in reqs)} |")
    w("")

    # ---- semgrep rules in the map that no pack it runs loads
    packs = load(ROOT / "data/semgrep-packs.json")["packs"]
    when = semgrep_packs(next(a for a in load(ROOT / "data/adapters.json")["adapters"]
                              if a["id"] == "semgrep"))
    named_elsewhere = {q for q in NOT_RUN if "semgrep" not in ev[q]["tools"]}
    order = lambda q: [int(x) for x in re.findall(r"\d+", q)]
    w("### Semgrep: rules in its map that are not run\n")
    w("Semgrep is counted above only through rules in a pack the adapter runs ("
      + ", ".join(f"`{p}`, {len(v['rules'])} rules"
                  + (f", run unless the app is known not to {CONDITION_WORDS.get(when[p], 'meet `' + when[p] + '`')}"
                     if when.get(p) else "")
                  + f", measured {v['measured']} with semgrep {v['semgrep']}"
                  for p, v in sorted(packs.items()) if p in when)
      + "). Its map names more requirements through rules no pack it runs loads; nothing counts those,"
      " and a report never credited them either, since a clean run is credited only with the rules its"
      f" own report lists. {len(named_elsewhere)} requirements are named that way and by no semgrep rule"
      " that runs:\n")
    w(", ".join(sorted(named_elsewhere, key=lambda q: (q[0], order(q)))) + ".\n")
    never = sorted({r for rules in NEVER_HANDED.values() for r in rules})
    only_there = {q for q in NEVER_HANDED if "semgrep" not in ev[q]["tools"]}
    if never:
        w(f"{len(never)} rules that a pack loads read only files `sv` never hands semgrep. A rule none "
          "of whose files it was handed ran over nothing, so they are not counted, and a report does "
          "not credit them (ADR-018, Later, 7 October 2026). "
          + (f"{len(only_there)} requirements are named only by them: "
             + ", ".join(sorted(only_there, key=lambda q: (q[0], order(q)))) + ".\n" if only_there
             else "No requirement is named only by them, so the counts above do not change with it.\n"))
    else:
        w("Every rule a pack loads reads files `sv` hands semgrep: the app's code files, templates "
          "among them since 8 October 2026 (ADR-054), and the other files a rule in the map names, "
          "such as nginx's `*.conf` and `web.config`, since the same day (ADR-018, Later). A rule "
          "credits a clean run only for an app that has a file it reads.\n")

    # ---- ASVS by chapter
    w("## ASVS 5.0 by chapter\n")
    w("| Chapter | Requirements | Can settle | Can be credited | Supporting only | The owner answers | No check yet |")
    w("|---|---|---|---|---|---|---|")
    chapters = defaultdict(list)
    chapter_rows = []
    for q, v in asvs.items():
        chapters[(v["chapter"], v["chapter_name"])].append(q)
    for (cid, name), reqs in sorted(chapters.items(), key=lambda x: int(x[0][0][1:])):
        s = sum(settles(q) for q in reqs)
        c = sum(credited(q) for q in reqs)
        p = sum(supports_only(q) for q in reqs)
        o = sum(owner_only(q) for q in reqs)
        w(f"| {cid} {name} | {len(reqs)} | {s} | {c} | {p} | {o} | {len(reqs) - s - p - o} |")
        chapter_rows.append((f"{cid} {name}", c, s - c, p, o, len(reqs) - s - p - o))
    w("")

    # ---- ASVS lists
    def listing(title, ids, show_checks=True):
        w(f"### {title} ({len(ids)})\n")
        if not ids:
            w("None.\n")
            return
        w("| Requirement | Level | Checks |")
        w("|---|---|---|")
        for q in sorted(ids, key=lambda q: [int(x) for x in re.findall(r"\d+", q)]):
            checks = []
            for t, name, _ in TIERS:
                names = ev.get(q, {}).get(t, [])
                if names:
                    shown = ", ".join(f"`{c}`" for c in names[:4])
                    if len(names) > 4:
                        shown += f" and {len(names) - 4} more"
                    checks.append(f"{name}: {shown}")
            only = ""
            if q in FINDINGS_ONLY:
                by_tool = defaultdict(list)
                for tool, r in sorted(FINDINGS_ONLY[q]):
                    by_tool[tool].append(f"`{r}`")
                only = " (" + "; ".join(f"{tool} only ever as a finding: {', '.join(rules)}"
                                        for tool, rules in by_tool.items()) + ")"
            w(f"| {q} | L{asvs[q]['level']} | {'; '.join(checks)}{only} |")
        w("")

    w("## ASVS 5.0 requirement by requirement\n")
    listing("Settled by reading the code", [q for q in asvs if settles(q) and "static" in tiers(q)])
    listing("Settled by asking the running app", [q for q in asvs if settles(q) and ({"running", "signed-in"} & set(tiers(q)))])
    listing("Settled by known-vulnerability data", [q for q in asvs if settles(q) and "advisories" in tiers(q)])
    listing("Settled only by an outside tool", only_tools)
    listing("Supporting only", [q for q in asvs if supports_only(q)])
    l1 = sorted((q for q, v in asvs.items() if v["level"] == 1 and not tiers(q)),
                key=lambda q: [int(x) for x in re.findall(r"\d+", q)])
    w(f"### Level 1 with no check at all ({len(l1)})\n")
    # Not every one is every app's (backlog 0472): read which each applies to from the applicability rules,
    # the most specific scope that names it, as the report does.
    rules = load(ROOT / "data/knowledge/applicability.json")["rules"]

    def condition_of(q):
        scopes = [r for r in rules if q == r["scope"] or q.startswith(r["scope"] + ".")]
        return max(scopes, key=lambda r: len(r["scope"]))["condition"] if scopes else "always"

    by_condition = defaultdict(list)
    for q in l1:
        by_condition[condition_of(q)].append(q)
    # Each condition's question as `sv` asks it, from `sv-frameworks`, so the paragraph is in a person's words.
    questions = dict(re.findall(r'=> "([a-z0-9-]+)", (?:Claim|Derived), "(?:[^"\\]|\\.)*",\s*"((?:[^"\\]|\\.)*)";',
                                (ROOT / "crates/sv-frameworks/src/condition.rs").read_text(encoding="utf-8")))
    every = by_condition.pop("always", [])
    w("Where a new check would do the most good. Which of them apply to an app depends on what the app is "
      "(`data/knowledge/applicability.json`):\n")
    w(f"- Every app ({len(every)}): " + (", ".join(every) or "none"))
    for condition in sorted(by_condition):
        qs = by_condition[condition]
        if condition not in questions:
            sys.exit(f"coverage.py: no question for the condition `{condition}` in crates/sv-frameworks/src/condition.rs")
        w(f"- Only where the answer to \"{questions[condition]}\" is yes ({len(qs)}): " + ", ".join(qs))
    w("")

    # ---- AISVS
    w("## AISVS 1.0 by chapter\n")
    w("Whether these apply at all is decided from what the app says and what its code shows about AI;")
    w("most of AISVS is about how models are trained and run, which reading an application's code")
    w("does not reach.\n")
    w("| Chapter | Requirements | Can settle | Can be credited | Supporting only | The owner answers | No check yet |")
    w("|---|---|---|---|---|---|---|")
    chapters = defaultdict(list)
    for q, v in aisvs.items():
        chapters[(v["chapter"], v["chapter_name"])].append(q)
    for (cid, name), reqs in sorted(chapters.items(), key=lambda x: int(x[0][0][1:])):
        s = sum(settles(q) for q in reqs)
        c = sum(credited(q) for q in reqs)
        p = sum(supports_only(q) for q in reqs)
        o = sum(owner_only(q) for q in reqs)
        w(f"| {cid} {name} | {len(reqs)} | {s} | {c} | {p} | {o} | {len(reqs) - s - p - o} |")
    w("")
    settled_ai = sorted((q for q in list(aisvs) + list(appendix) if settles(q)),
                        key=lambda q: [int(x) for x in re.findall(r"\d+", q)])

    def sv_finding_only(q):
        """`sv`'s own checks that only ever raise q as a finding."""
        return {r for tool, r in FINDINGS_ONLY.get(q, ()) if tool == "sv"}

    def credited_by_other(q):
        return any(c for tier, checks in ev[q].items() for c in checks
                   if c not in sv_finding_only(q)
                   and not (tier in TOOL_TIERS
                            and any(tool == c for tool, _ in FINDINGS_ONLY.get(q, ()))
                            and c not in CREDITED_BY_TOOL[q]))

    only = [q for q in settled_ai if q in FINDINGS_ONLY and not credited_by_other(q)]
    w(f"{len(only)} of these {len(settled_ai)} can only ever be marked *needs attention*: a check can")
    w("show the control missing, and finding nothing does not show it present, so a clean run credits")
    w("none of them. Most are `sv`'s own checks: rules that read the code, and questions asked of the")
    w("running app (`--run`, with an `ai` section). The rest are semgrep's, which runs with `sv report` unless `--no-tools`, and CodeQL's, which runs only when stackvet.toml turns it on.\n")
    for q in settled_ai:
        rules = sorted(FINDINGS_ONLY.get(q, ()))
        # A tool whose rules only ever find this failing does not settle it, whichever tool it is.
        finding_only_tools = {tool for tool, _ in rules} - CREDITED_BY_TOOL[q]
        names = [c for tier, checks in ev[q].items() for c in checks
                 if not (tier in TOOL_TIERS and c in finding_only_tools)
                 and c not in sv_finding_only(q)]
        parts = []
        if names:
            whole = [c for c in names if c not in IN_PART]
            if whole:
                parts.append(f"settled by {', '.join(f'`{c}`' for c in whole[:4])}"
                             + (f" and {len(whole) - 4} more" if len(whole) > 4 else ""))
            partial = [c for c in names if c in IN_PART]
            if partial:
                parts.append(f"checked in part only, by {', '.join(f'`{c}`' for c in partial)}")
        by_tool = defaultdict(list)
        for tool, r in rules:
            by_tool[tool].append(f"`{r}`")
        for tool, found in by_tool.items():
            parts.append(f"found failing by {tool}'s " + ", ".join(found))
        w(f"- {q}: " + "; and ".join(parts) + ".")
    w("")

    # ---- SbD
    w("## Secure by Design checklist 0.5.0 by domain\n")
    w("Every control is design review, answered by a person, so none is ever *checked*. A control with")
    w("an ASVS counterpart takes its level from it, and a check on that counterpart is shown beside the")
    w("control as supporting evidence.\n")
    w("| Domain | Controls | Critical | With an ASVS counterpart | With supporting evidence |")
    w("|---|---|---|---|---|")
    supported = []
    for domain in sbd["checklistDomains"]:
        ids = [f"SBD-{c['id']}" for c in domain["controls"]]
        crit = sum(c["critical"] for c in domain["controls"])
        with_cw = [q for q in ids if crosswalk.get(q)]
        with_ev = [q for q in ids if sbd_supported(q)]
        supported += with_ev
        w(f"| {domain['id']} {domain['name']} | {len(ids)} | {crit} | {len(with_cw)} | {len(with_ev)} |")
    w("")
    for q in supported:
        via = [a for a in crosswalk.get(q, {}) if tiers(a)]
        direct = ", ".join(f"`{c}`" for t in ev.get(q, {}).values() for c in t[:2])
        how = f"through {', '.join(via)}" if via else f"directly, from {direct}"
        w(f"- {q}: {how}")
    w("")

    text = "\n".join(out).rstrip() + "\n"
    rows = requirement_rows(asvs, aisvs, ev, tiers, settles, supports_only, manual_only, credited)
    reach_text = reach_json(asvs, aisvs, ev, tiers, settles, explain_checks(rows))
    # Two readings of "a check can credit it", made separately: the count in the tables above and
    # the kinds of run in data/reach.json. They must name the same requirements.
    reached = set(json.loads(reach_text)["requirements"])
    counted = {q for q in list(asvs) + list(aisvs) if credited(q)}
    if reached != counted:
        sys.exit("the tables and data/reach.json disagree about what a check can credit: "
                 + ", ".join(sorted(reached ^ counted)))
    listing_text = requirements_markdown(rows)
    if "--json" in sys.argv[1:]:
        path = Path(sys.argv[sys.argv.index("--json") + 1])
        write_text(path, json.dumps({"tiers": [{"id": t, "name": n, "needs": d} for t, n, d in TIERS],
                                    "requirements": rows}, indent=1) + "\n")
        print(f"wrote {path}")
    level_rows = []
    for level in sorted({v["level"] for v in asvs.values()}):
        reqs = [q for q, v in asvs.items() if v["level"] == level]
        s_, c_, p_, o_ = (sum(f(q) for q in reqs) for f in (settles, credited, supports_only, owner_only))
        level_rows.append((f"Level {level}", c_, s_ - c_, p_, o_, len(reqs) - s_ - p_ - o_))
    charts = ((CHAPTER_SVG, coverage_svg("ASVS 5.0 by chapter: what a check can reach", chapter_rows)),
              (LEVEL_SVG, coverage_svg("ASVS 5.0 by level: what a check can reach", level_rows)))
    if check:
        for path, want in ((OUT, text), (LIST_OUT, listing_text), (REACH_OUT, reach_text)) + charts:
            current = path.read_text(encoding="utf-8") if path.exists() else ""
            if current != want:
                sys.exit(f"{path.relative_to(ROOT)} is out of date: run `python3 tools/coverage.py` at "
                         "the repository root")
        return
    write_text(OUT, text)
    write_text(LIST_OUT, listing_text)
    write_text(REACH_OUT, reach_text)
    for path, svg in charts:
        write_text(path, svg)
    print(f"wrote {OUT.relative_to(ROOT)}, {LIST_OUT.relative_to(ROOT)}, {REACH_OUT.relative_to(ROOT)}, "
          f"{CHAPTER_SVG.relative_to(ROOT)} and {LEVEL_SVG.relative_to(ROOT)}")


# How each coverage status reads in the list, in words a person who is not a programmer reads easily.
STATUS = {
    "settle": "Can be checked",
    "found": "Can only be found failing",
    "support": "A check helps; a person decides",
    "owner": "The owner answers",
    "none": "No check yet",
}


def requirement_rows(asvs, aisvs, ev, tiers, settles, supports_only, manual_only, credited):
    """One row per ASVS and AISVS requirement: where it sits, what it asks, and what speaks to it."""
    words = check_words()
    tier_name = {t: n for t, n, _ in TIERS}
    asked, located = owner_answers()
    rows = []
    for name, reqs in (("ASVS 5.0", asvs), ("AISVS 1.0", aisvs)):
        for q, v in reqs.items():
            status = ("settle" if credited(q) else "found") if settles(q) else \
                "support" if supports_only(q) else "owner" if q in asked and not tiers(q) else "none"
            only = {r for _, r in FINDINGS_ONLY.get(q, ())}
            checks = []
            for t in tiers(q):
                for c in ev[q][t]:
                    if is_tool(t, c):
                        # Each phrase once: one rule's words can join two with "; " that
                        # another rule says alone.
                        what = sorted({p.strip() for w in TOOL_RULES[q].get(c, [])
                                       for p in w.split("; ") if p.strip()})
                        finding_only = c not in CREDITED_BY_TOOL[q]
                        checks.append({
                            "id": c, "kind": tier_name[t], "tool": True,
                            "label": "Its rules look for",
                            "words": "; ".join(what[:6])
                                     + (f"; and {len(what) - 6} more" if len(what) > 6 else ""),
                            "rules": len(TOOL_RULE_IDS[q][c]), "finding_only": finding_only,
                            "credited_only": False, "in_part": False,
                        })
                    else:
                        label, text = words.get(c, ("Looks for", ""))
                        checks.append({"id": c, "kind": tier_name[t], "tool": False, "label": label,
                                       "words": text, "finding_only": c in only,
                                       "credited_only": c in RUST_CREDITS_ONLY,
                                       # A credit in part is a credit: where the check only finds, it has none.
                                       "in_part": c in IN_PART and c not in only})
            rows.append({
                "id": q, "framework": name, "level": v["level"],
                "family": v["chapter"], "family_name": v["chapter_name"],
                "section": v["section"], "section_name": v["section_name"],
                "text": v["text"], "status": status, "status_words": STATUS[status],
                "person_decides": q in manual_only, "checks": checks,
                "asked": asked.get(q, []), "located": located.get(q, []),
            })
    return rows


def requirements_markdown(rows):
    order = lambda q: [int(x) for x in re.findall(r"\d+", q)]
    out = []
    w = out.append
    w("# Every ASVS and AISVS requirement, and what checks it\n")
    w("Generated by `tools/coverage.py` from the checks' own citations, with `docs/COVERAGE.md`. Do not")
    w("edit by hand; run `python3 tools/coverage.py` after changing a check, and the build fails until")
    w("you do. It lists every requirement, whether or not it applies to a given app; which apply is")
    w("decided per app, from its `stackvet.toml` and its code.\n")
    w("## How to read this\n")
    w(f"- **{STATUS['settle']}**: at least one check can mark it *checked* or *needs attention*. A check")
    w("  is almost always about part of a requirement, so a clean result is one automated check that")
    w("  was satisfied, not proof the whole requirement is met.")
    w(f"- **{STATUS['found']}**: every check that speaks to it can show it is not met, and none can show")
    w("  it is, so it can be marked *needs attention* and never *checked*.")
    w(f"- **{STATUS['support']}**: a check speaks to it, but it asks for something no check can settle,")
    w("  such as a documented policy or a design decision.")
    w(f"- **{STATUS['owner']}**: no check speaks to it, and `sv review` asks the owner for it: a section in")
    w("  the security notes (*documented*), a design question (*attested*), or a check by hand (*checked by")
    w("  hand*). An answer is never *checked*. Where `sv` can find the code the answer is about, it shows the")
    w("  owner where to look, and the row names the locator that does.")
    w(f"- **{STATUS['none']}**: nothing in `sv` speaks to it. A passing test of the app's own that names the")
    w("  requirement's id in code gives it *tested by the app's own tests*, below *checked* (ADR-050);")
    w("  otherwise it stays *not verified*.")
    w("- *found failing only*: that check can show the requirement is not met, and finding nothing does")
    w("  not show it is, so a clean run credits nothing.")
    w("- *credited only*: that check can show the requirement is met, and never marks it *needs")
    w("  attention*, since the app may meet it in a way the check cannot see; it says so instead.")
    w("- *only ever in part*: that check tries one piece of what the requirement asks, so what it credits")
    w("  is marked *checked in part*, never *checked* (ADR-053).")
    w("- Each check says what kind it is and so what it needs to run:\n")
    w("| Kind | Needs |")
    w("|---|---|")
    for _, name, needs in TIERS:
        w(f"| {name} | {needs} |")
    w("")
    for framework in ("ASVS 5.0", "AISVS 1.0"):
        mine = [r for r in rows if r["framework"] == framework]
        w(f"## OWASP {framework}\n")
        counts = Counter(r["status"] for r in mine)
        w(f"{len(mine)} requirements: {counts['settle']} can be checked, {counts['found']} can only be found "
          f"failing, {counts['support']} where a check "
          f"helps but a person decides, {counts['owner']} the owner answers, and {counts['none']} with no check "
          "yet.\n")
        for level in sorted({r["level"] for r in mine}):
            at = [r for r in mine if r["level"] == level]
            c = Counter(r["status"] for r in at)
            w(f"### Level {level} ({len(at)} requirements, {c['settle']} can be checked, {c['found']} can only "
              "be found failing)\n")
            families = defaultdict(list)
            for r in at:
                families[(r["family"], r["family_name"])].append(r)
            for (fid, fname), reqs in sorted(families.items(), key=lambda x: order(x[0][0])):
                w(f"#### {fid} {fname}\n")
                w("| Requirement | Coverage | Checks |")
                w("|---|---|---|")
                for r in sorted(reqs, key=lambda r: order(r["id"])):
                    parts = []
                    for ch in r["checks"]:
                        if ch["tool"]:
                            n = ch["rules"]
                            desc = f"{ch['kind']}: {ch['id']}, {n} rule{'' if n == 1 else 's'}"
                        else:
                            desc = f"{ch['kind']}: `{ch['id']}`"
                        if ch["words"]:
                            desc += f", {ch['label'].lower()}: {ch['words']}"
                        if ch["finding_only"]:
                            desc += " (found failing only)"
                        if ch["credited_only"] and ch["in_part"]:
                            desc += " (credited only, and only ever in part)"
                        elif ch["credited_only"]:
                            desc += " (credited only)"
                        elif ch["in_part"]:
                            desc += " (only ever in part)"
                        parts.append(desc.replace("|", "\\|"))
                    if r["status"] == "owner":
                        where = (f"; `sv` shows where to look ({', '.join(f'`{l}`' for l in r['located'])})"
                                 if r["located"] else "")
                        parts.append(f"Asked of the owner: {' and '.join(r['asked'])}{where}")
                    text = r["text"].replace("|", "\\|").replace("\n", " ")
                    checks = "<br>".join(parts) if parts else "–"
                    w(f"| **{r['id']}** {text} | {r['status_words']} | {checks} |")
                w("")
    return "\n".join(out).rstrip() + "\n"


if __name__ == "__main__":
    main()
