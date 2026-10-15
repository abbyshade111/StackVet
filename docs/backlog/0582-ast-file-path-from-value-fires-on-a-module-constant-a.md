# `ast.file-path-from-value` fires on a module constant, a `secrets.token_hex` name, and a validated name, making V1.2.1 and V5.3.2 "needs attention"

**Status:** claimed by honesty, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: medium (the app's only two "needs attention"
lines, and the tool judged all three wrong with the lines quoted and left the code alone). To verify against the code
before building: the reading cites `vibegames/games.py` for the three.

**Fix.** The rule follows a name to its source when it is in the same file: a module constant, a value from
`secrets.token_hex`/`uuid4`, or a name that passed `secure_filename`-style validation is not "a value from outside";
report the rest as before. Fake-app cases for the three shapes.

**Record.** None (a rule's accuracy).

**Built, 15 October 2026 (honesty):** `ast.file-path-from-value` now follows a name in a Python path to where the same
file sets it (`safeNamesRead`, `crates/sv-check/src/ast/followed.rs`). Not reported: a module constant built from the
app's own folder, a name made with `secrets.token_hex` or `uuid4`, a `secure_filename` result kept in a name, and a
parameter a narrow `re.fullmatch` (or a `secure_filename` comparison) stops the function without. Still reported: a
request's value by name or not, a name with any value not shown safe, a loose or non-stopping check, and a module name a
function sets from a request. `vibegames/games.py` was not in the cloud container, so the three shapes are the ones the
reading names, not its lines. Tests in `crates/sv-check/src/ast/followed_tests.rs`.
