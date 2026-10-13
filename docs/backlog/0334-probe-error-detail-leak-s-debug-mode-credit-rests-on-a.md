# probe.error-detail-leak's debug-mode credit rests on a finite marker list that misses ASP.NET Core and Laravel

**Status:** claimed by builder-none, 12 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: checks' honesty. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** ASP.NET Core's developer exception page and Laravel Ignition's page are not in the list, so a debug page from those reads as a clean error. Already in part; worth saying in the scope.

**Where.** `crates/sv-check/src/probes.rs:650-668` (credits V13.4.2 in part when a 5xx body has none of `TRACE_MARKERS`); the list at `:1460-1474` (Python, Werkzeug, Django, Spring, Java, Go, Rails, PHP, and "    at ").

**Evidence.** The marker list.

**How it bites.** A .NET or Laravel app's debug page is credited as a clean error.

**Fix.** Add the two markers, and say in the scope "against N known debug pages"; a test with each page's distinctive text.

**Record.** No record governs `probes.rs` by name; say so.
