# All your apps on one page, and what changed between checks

For the day you have more than one app, or want to see how one changes: the dashboard, the history it keeps if you
turn it on, and `sv compare`. Each reads reports StackVet already wrote and checks nothing itself. The guide
([`GETTING-STARTED.md`](GETTING-STARTED.md)) says how to get the first report.

Once each app has a report, `sv dashboard` puts them on one page you open in a browser: every app in alphabetical
order, each with its own view of what was found, what was not examined, and where its requirements went. It reads
the reports already there and checks nothing itself, so run `sv report` on an app first to bring its part up to
date. Give it the app folders, and the file to write:

```bash
sv dashboard ~/code/app-one ~/code/app-two --out ~/sv-dashboard.html
```

It writes only that file, never inside an app's folder, and never over a file it did not make.

To see how each app changes from one check to the next, turn on history once: `sv history on`. From then on, each
`sv report` you run keeps a small record of the run in your home folder, readable only by you and never your code, and
`sv dashboard --out ~/sv-dashboard.html` shows every app you have checked, each with its runs over time. A run is only
compared with an earlier one of the same kind, so a quick check after a full one does not make the app look worse.
`sv history off` stops it, and `sv history forget --all` deletes what was kept.

To see what changed between two particular reports, history on or not, keep a copy of the older report folder and run
`sv compare`:

```bash
cp -R stackvet-report ~/report-before   # before the change
sv report .                             # after it
sv compare ~/report-before              # the newer is this folder's report
```

It lists each requirement whose status moved, with what it gained or lost (a check that now credits it, a finding
that went away), the findings that came or went, and the counts. It writes nothing. It says first when it cannot show
that a report is one StackVet wrote on this computer, or when the two runs were not alike (a different kind of run,
level, `stackvet.toml`, security notes, or version of StackVet), since a requirement can then move for that reason
alone.
