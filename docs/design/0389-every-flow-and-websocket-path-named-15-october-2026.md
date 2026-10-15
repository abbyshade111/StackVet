# Every flow and WebSocket path named (15 October 2026)


Backlog 0232, part 3 (group C), the rest of it; ADR-118.

**The problem.** The flow check (V2.3.1) and the WebSocket checks (V4.4.4, and V4.4.2 signed in and anonymous) each
tried the one flow or path stackvet.toml had room for, so each credit was *checked in part* for that alone.

**What was built.**
- **Settings.** `flows` beside `flow` and `private-websockets` beside `private-websocket` in the users section, and
  `websockets` beside `websocket` under `[stack.run]`. `UsersSection::all_flows`,
  `UsersSection::private_websocket_paths`, and `RunSection::websocket_paths` give the list, or the one. The run plan
  carries `websockets`, a list.
- **The flow check** (`flows.rs`) tries each flow as the one was: the first keeps its request ids (`login-flow-b<i>`)
  and marker, and each later one has its own (`login-flow-<k>-b<i>`). A flow that can be skipped is a finding naming it
  by its last step; one that cannot be judged (fewer than two steps, no `completed`, or not finishing in order) is
  named as not judged.
- **The private WebSocket check** (`sessions.rs`) opens each path, with ids `-<k>` after the first's. Signing out is
  tried on each path that needed a real session. Each path's signed-in handshake is now part of what both credits rest
  on, which the crash sweep asked for once there were several paths.
- **The anonymous WebSocket check** (`probes.rs`) sends each path's two handshakes and judges each; `evaluate_api`
  takes the paths, and the credit names its own evidence.
- **Each credit** is whole only when two or more were named, each was judged, and every one held; otherwise in part,
  naming what was not judged. One that failed is a finding, and the credit is not given beside it.
- **Coverage.** The three rules come off the list of rules only ever in part: 54 requirements, not 56, can only ever
  be checked in part.

**The stand-in app** answers a second flow (`/apply/1`, `/apply/2`; flaw `apply_unguarded`) and a second private
WebSocket (`/chat`; flaw `chat_ws_open`).

**Tests.** `signed_in/flows/several_tests.rs` (six), `signed_in/sessions/several_websockets_tests.rs` (five), and
`probes/several_websockets_tests.rs` (five).

**Broken on purpose, each put back:**
- Flows: only the first tried (3 red); not-judged ignored (none at first, since the test's one judged flow was in part
  anyway; 1 red once a test with two that held and one not judged was added); the old evidence list (none, until a
  test named the credit's evidence from the table; then 1 red).
- Private WebSockets: only the first path (3 red); not-judged ignored, for either credit (1 red each); whole from one
  path (1 red).
- Anonymous WebSockets: only the first path (4 red); not-judged ignored (1 red); both keys read (1 red).
