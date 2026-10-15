# `ast.file-path-from-value` fires on a module constant, a `secrets.token_hex` name, and a validated name, making V1.2.1 and V5.3.2 "needs attention"

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: medium (the app's only two "needs attention"
lines, and the tool judged all three wrong with the lines quoted and left the code alone). To verify against the code
before building: the reading cites `vibegames/games.py` for the three.

**Fix.** The rule follows a name to its source when it is in the same file: a module constant, a value from
`secrets.token_hex`/`uuid4`, or a name that passed `secure_filename`-style validation is not "a value from outside";
report the rest as before. Fake-app cases for the three shapes.

**Record.** None (a rule's accuracy).
