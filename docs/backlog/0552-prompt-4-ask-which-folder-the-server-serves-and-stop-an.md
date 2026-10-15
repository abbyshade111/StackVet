# Prompt 4: ask which folder the server serves, and stop an older container before going on

**Status:** claimed by wording, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity trial. For the prompt library
(0370); the sv side is the `stackvet_status` item.

**What was seen.** The tool spent 16 minutes connected to a container that served an older app's folder; three stale
containers were running.

**What to change.** Prompt 4: "Ask stackvet_status which folder it serves. If it is not this one, show me `docker ps`,
stop the old container with my yes, and tell me to restart the tool."
