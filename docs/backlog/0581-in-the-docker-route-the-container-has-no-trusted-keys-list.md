# In the Docker route the container has no trusted-keys list, so after `sv review` the tool's `stackvet_check` cannot count the seals

**Status:** claimed by wording, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the Opus run's reading (backlog 0577), 15 October 2026. Severity: medium (prompt 7's last step shows nothing in the
route most people use).

**What was seen.** After the owner's `sv review` on the Mac, the tool's `stackvet_check` (sv in the container, reached
through Antigravity's MCP settings) reported the confirmations as not counted: `seal.rs` `NoTrustedList`, "this computer
has no list of trusted keys to check it against", because `~/.config/stackvet/allowed_signers` is not in the container.
The tool then proposed `sv report .` on the Mac, which is what overwrote the --run report (0578).

**Fix.** One of: `sv connect --docker` mounts `~/.config/stackvet` read-only into the container (the keys never leave the
computer; the allowed-signers list and the public key are what the check needs; the private key must not be mounted,
so mount the two files, not the folder); or `sv review` writes the trusted list's public half into the app's
`.stackvet/` beside the record, where the container already reads; or prompt 7 says the confirmation shows only in a
report written on the computer. The first keeps the review where it is; the second changes what sv writes into the
folder (a record). The owner decides which.

**Record.** ADR-108 (the Docker route) or ADR-026 Later, depending on the choice.

**The owner's answer, 15 October 2026:** A: sv connect --docker adds two read-only file mounts, the trusted-keys list and the public key, never the private key or the folder; the container then counts seals as the computer does. ADR-108 Later. (B rejected: the AI tool can write the app's .stackvet folder, so a trusted list there could be added to by the tool.)
