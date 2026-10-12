# A branch's modified rust.yml could publish and sign a latest image that action.yml accepts as genuine (the owner's setting)

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap. From the deep review of 12 October 2026 (backlog 0277), area: tools and CI. Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** The `if:` lives in the workflow file the branch itself controls, and GitHub grants a same-repository pull request run the permissions the workflow asks for.

**Where.** `.github/workflows/rust.yml:299` (`if: github.event_name == 'push' && github.ref == 'refs/heads/main'`) with `packages: write`, `id-token: write`, `attestations: write` at `:308-314`; `action.yml:87-88` verifies only `--repo` and `--signer-workflow`, nothing about the branch.

**Evidence.** `gh attestation verify "oci://$ref" --repo abbyshade111/StackVet --signer-workflow abbyshade111/StackVet/.github/workflows/rust.yml` with no `--source-ref`.

**How it bites.** A branch rewrites the `if:` and pushes `ghcr.io/abbyshade111/stackvet-sv:latest` with a valid signed statement; every user of the Action then runs it, and the verification passes.

**Fix.** Two parts. The owner's: put the `publish` job behind a GitHub Environment restricted to `main` (a repository setting). A build: add `--source-ref refs/heads/main` to the `gh attestation verify` call in `action.yml` where the runner's gh supports it (2.65 and later), else check the attestation's `workflow.ref`.

**Record.** ADR-051 governs `rust.yml`; the Environment is a repository setting and wants the owner's yes and a Later line when made.
