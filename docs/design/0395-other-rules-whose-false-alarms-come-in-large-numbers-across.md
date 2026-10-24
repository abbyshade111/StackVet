# Other rules whose false alarms come in large numbers, across the trial apps (23 October 2026)


Backlog 0648, at the owner's word of 23 October 2026: "investigate other high volume false positives like this one to
see if those can be fixed as well". This entry is what was found; one choice is left for the owner.

## How it was measured

Today's `sv` (after backlog 0643 to 0646, 0653 to 0657) made a report, without starting the app, of the six apps from
the trials of 14 and 15 October 2026, the owner's Cursor build of 22 October, and the five apps under `examples/`, each
written to a folder outside the app. The things to fix were read from each report's own `fix-now.md`, so a finding the
report lists apart (test code, a rule for another framework, worth a look) is not counted among them.

Per app, the things to fix and the findings in all: Cursor build 9 of 20; the other trial apps 3 of 21, 2 of 5, 1 of
31, 2 of 12, 2 of 6, and 4 of 14; the examples 3 of 4, 3 of 6, 1 of 3, 3 of 4, and 1 of 3.

## What was found

- **No rule has the path rule's volume any more.** The most frequent among the things to fix is Bandit's B108, 12
  across 8 apps; nothing else passes 5.
- **The noisiest outside rules are already listed apart.** Semgrep's Django rule `django-no-csrf-token` fired 36 times
  in 5 apps, none of which uses Django, on forms that each carry a CSRF token field; the report already sets such a
  finding apart as a rule for another framework (backlog 0521), where it decides nothing, which is right for a
  Django rule in an app with no Django (this entry said "still counts it" until corrected the same day). Semgrep's Flask rule
  `direct-use-of-jinja2` fired 21 times in one FastAPI app that turns escaping on (`select_autoescape`); set apart the
  same way.
- **Bandit's B108 (a fixed path under `/tmp`) is right to report.** Every one is a database or data folder defaulting to
  `/tmp` (`os.environ.get("DATABASE_PATH", "/tmp/...")`): a folder every user of the machine can write, emptied on
  reboot. A real weakness, if a low one, and the fix is small.
- **The credential rule is right to report.** Its five are a real invite code written into `stackvet.toml`'s sign-up
  form and into the app's own code, and quoted in an AI tool's transcripts kept in the app folder. ADR-124 says such a
  value is printed by the seed and cited as `{seed:NAME}`, never written in the manifest.
- **Bandit's B104 (listening on every address) contradicts `sv`'s own advice.** Four apps, two of them examples, each
  `HTTPServer(("0.0.0.0", port), ...)`. `sv preflight` tells an app to listen on every address (ADR-035), because
  inside a container that is the only way to be reached; then B104 reports the same line as a thing to fix. Outside a
  container, on a laptop, the same line puts the app on the local network, so it is not always a false alarm.

## The choice

**A. When stackvet.toml runs the app in a container, B104 is worth a look (recommended).** An app with `[stack.run]`,
which `sv` starts in a container, must listen on every address there; its B104 finding is listed apart as worth a look,
said beside it (in a container this is how the app is reached; outside one it is on the network), and still counted.
An app with no `[stack.run]` keeps B104 among the things to fix. Built as other outside rules are set apart (backlog
0521), with a test each way.

**B. Leave B104 as it is.** Every app that follows `sv`'s own advice keeps one thing to fix for doing so, and the
report says to change what the preflight said to do.

**C. Have the preflight's advice name the risk instead.** Keep B104 among the things to fix, and have `sv preflight`
say that listening on every address is right in the container and to keep the port off the network otherwise. The
contradiction stays in the report, explained.

Nothing else from this survey needs building: the other frequent findings are real, and the noisy outside rules are
already set apart.

## The owner's answer, 23 October 2026

"Container: worth a look (Recommended)". Built as backlog 0658.
