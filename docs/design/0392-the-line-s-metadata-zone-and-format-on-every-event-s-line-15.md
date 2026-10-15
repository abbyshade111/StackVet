# The line's metadata, zone, and format on every event's line (15 October 2026)


Backlog 0232, group E, the rest (ADR-053, "Later, 15 October 2026: the line's metadata, zone, and format on every
event's line"). After "The sign-ins in the log, whole when every one is found" and "The code sign-ins in the log".

**What was built.** `logs::evaluate` gathers, beside the refused sign-in's line, every other security event's line the
log checks found (`Other`): the wrong password, the accepted sign-in, each code taken and refused, and each refused
request found with its status. `metadata_checks` and `format_check` read them too. V16.2.1 is whole when every other
sign-in's line was found by the account's name and carries a time and a place; V16.2.2 when every other line's time
states its zone; V16.2.4 when every other line is JSON, logfmt, or the common log format. Each needs at least one
other line, so one line alone stays in part, as before.

**What did not change.** The refused sign-in's line still decides when it fails, and when it is the only line. A line
with no time of its own is still not assessed for the zone, since the platform may add one.

**Tests.** `crates/sv-check/src/logs/every_line_tests.rs`: all three whole; another line with no zone is the finding,
about that line; another line in free text keeps the format in part; a sign-in line found only by its window keeps
the metadata in part. One existing test, `an_app_that_logs_both_sign_ins_and_the_refusal_is_credited`, now expects
the zone whole, since its three event lines all state it. Broken on purpose, each put back: the format never whole;
who not asked of the other lines; the other lines' zones unread. Each turned its own test red, and only it.
