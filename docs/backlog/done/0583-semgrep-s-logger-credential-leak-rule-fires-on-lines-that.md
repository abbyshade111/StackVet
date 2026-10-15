# Semgrep's `logger-credential-leak` rule fires on lines that log only ids, and lands as findings

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: low.

**What was seen.** Four `python.lang.security.audit.logging.logger-credential-leak` findings on log lines that carry a
user id or a game id, no credential; in today's rerun they are listed as "names a requirement this app is not being
assessed against", which is right for the level but not for the rule: it matched a variable name, not a secret.

**Fix.** Read the rule's match: when the logged names are ids and the line holds no password, token, key, or secret
name, list it as information with the reason; `docs/research/SEMGREP-FALSE-ALARMS.md` gains the row.

**Record.** ADR-111 Later if the rule is set aside by name.

**Done, 15 October 2026.** `sv` now reads the logger call each finding of the rule points at
(`crates/sv-check/src/logged_ids.rs`): when its message is a plain string and every value it logs is an id (`user_id`,
`game.id`, `accountId`) with no part named for a secret, the finding is listed apart as `Outranked::LogsOnlyIds`, shown
with the names it logs, and decides nothing, as a finding from another framework's rule does (backlog 0521). A line
logging a password, a token, an email, or anything `sv` cannot read for certain still counts. The rule is not set aside
by name. ADR-111, a Later entry; `docs/research/SEMGREP-FALSE-ALARMS.md` gains a "Since" note and the row. Held by
three test files, each red with its part undone. Not verified: the Opus app's own four lines, which are not in the
repository, and a run of Semgrep itself, which is not installed here; the reading is tested on lines written to match
what the reading of 0577 describes.
