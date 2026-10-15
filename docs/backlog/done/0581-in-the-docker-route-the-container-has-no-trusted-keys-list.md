# In the Docker route the container has no trusted-keys list, so after `sv review` the tool's `stackvet_check` cannot count the seals

**Status:** done, 15 October 2026

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

**The owner's second answer, 15 October 2026, on what building A found:** mount the trusted-keys list only. The seal
check never reads the public key file, and a mount of a file not yet made puts a folder in its place, which would stop
`sv review` from making its key.

**Done, 15 October 2026.** `sv connect --docker --keys DIR` mounts the key folder's `allowed_signers`, one file,
read-only, at `/sv-config/stackvet/allowed_signers`, and sets `XDG_CONFIG_HOME=/sv-config`; never the folder, never the
public key (the owner's second answer). Setup prompt 3's step 6 makes the empty list first, with the person's yes, and
passes `--keys "$HOME/.config/stackvet"`; the guide's section 3 does the same, and its section 5 points to `--keys`
where it showed hand-written settings. Outside the container `sv connect` refuses `--keys` until the list is there,
since Docker would make a folder in its place; a path that is not full, or holds `:` or `,`, is refused. Tests:
`crates/sv-cli/tests/connect_keys.rs` (new), and `connect.rs` and `review_container.rs` updated to the new words. The
record is a Later entry on ADR-101, which governs `sv connect`; the answer named ADR-108, which is the Windows release.
Not tried in an AI coding tool or with a real container: what the container then counts rests on the native test that
a folder holding the list alone counts a signed answer.
