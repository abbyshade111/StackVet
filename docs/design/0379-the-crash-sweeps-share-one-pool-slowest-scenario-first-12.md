# The crash sweeps share one pool, slowest scenario first (12 October 2026)

Backlog 0271. ADR-021 is the rule these two tests guard, unchanged.

**The problem.** `a_crash_never_turns_a_finding_into_a_pass` and `a_crash_on_a_correct_app_raises_no_finding`
(`crates/sv-check/src/signed_in/mod.rs`) run each scenario once, then once for every request it sends with that request
crashed, so their cost is the requests sent times the time of one run, summed over the scenarios. In CI the
`sv-check` unit tests took 267 to 279 s (three runs of `main` that afternoon), and both sweeps were still running past
the sixty-second mark.

**Measured first.** Alone on the owner's Mac (8 cores): the first sweep 84 s, the second 14 s, the whole program 167 s.
Per scenario, 107 to 410 requests and 18 to 296 ms a run; the slowest single sweep was the JWT scenario's, 238 runs of
249 ms that took 23.6 s where the same work spread over 8 cores is 7.4 s. Each sweep gave every thread a fixed share of
one scenario's requests and waited for the slowest share before starting the next scenario, 29 times over. Timing on
the Mac did not settle anything further: its load average was 26 to 128 that afternoon (other sessions' builds), and old
and new each took 84 to 100 s, alternating. Compiling `sv`'s own crates at `opt-level = 1` for tests was also measured,
in CPU time, which load does not move: 550 CPU-seconds against 582 to 617, about 8 percent, for a slower build of every
crate; not taken.

**What was built.** One pool (`each_at_once`): a thread per core, each taking the next job from a shared counter. Each
test lists every scenario's crashed runs as one set of jobs, the slowest scenario's first (from its run without a
crash, timed), so no long run is left alone at the end. `Scenario::sweep` is split into `plain` (the run without a crash
and what it held back, unchanged) and `turned_by` (one crashed run, judged as before). Which runs are made, and what each
is judged on, is the same; the results are sorted before the assertion, so a failure reads the same each time.

**Measured after, on CI's runner** (draft #1535): the `sv-check` unit tests took 149 s, against 267 to 279 s on `main`,
and only the first sweep still passed the sixty-second mark.

**Broken on purpose, before and after:** the forged-create entry taken out of `RESTS_ON_A_REFUSAL`. Both the old sweep and
the new fail, naming the same three credits turned by a crash (the seeded scenario, the second create from another site,
and the JSON create also taken as a form).

**Not done.** The other two ideas in the item: reusing the run without a crash (it is made once per scenario already,
and a crashed run cannot start from its middle), and crashing only one of two identical requests (two requests alike are
not the same experiment once either changes what the app holds for the rest of the run).
