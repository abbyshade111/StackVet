# sv probe's one DNS question accepts an answer from any sender

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: the fence. Severity: low. Verified against the code by the reviewing session before filing.

**What is wrong.** Any datagram reaching the ephemeral port with a matching 16-bit id is read as the resolver's answer. ADR-027's entry of 8 October made the id random, which is half the defense.

**Where.** `crates/sv-check/src/live_tls.rs:347-350`: `let (n, _) = socket.recv_from(&mut buf)` discards the sender's address.

**Evidence.** The discarded sender at the line above.

**How it bites.** Something on the local network answering first with a chosen address.

**Fix.** Loop on `recv_from` until the sender is `(server, 53)` or the 5-second timeout passes; a test with a wrong-sender datagram.

**Record.** ADR-027 governs: a Later line (the answer is taken only from the resolver asked).
