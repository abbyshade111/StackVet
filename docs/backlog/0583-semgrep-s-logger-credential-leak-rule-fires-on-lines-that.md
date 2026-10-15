# Semgrep's `logger-credential-leak` rule fires on lines that log only ids, and lands as findings

**Status:** claimed by documentation, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: low.

**What was seen.** Four `python.lang.security.audit.logging.logger-credential-leak` findings on log lines that carry a
user id or a game id, no credential; in today's rerun they are listed as "names a requirement this app is not being
assessed against", which is right for the level but not for the rule: it matched a variable name, not a secret.

**Fix.** Read the rule's match: when the logged names are ids and the line holds no password, token, key, or secret
name, list it as information with the reason; `docs/research/SEMGREP-FALSE-ALARMS.md` gains the row.

**Record.** ADR-111 Later if the rule is set aside by name.
