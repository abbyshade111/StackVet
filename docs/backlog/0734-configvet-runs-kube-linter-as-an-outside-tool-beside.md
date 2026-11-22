# ConfigVet runs kube-linter as an outside tool, beside hadolint, Checkov, and Trivy

**Status:** open

Filed 22 November 2026, at the owner's word. ADR-157's part 3 named four outside tools and built three (backlog 0693;
ADR-157, "As built", part 3: "Not done: kube-linter").

- **To do:** an adapter for kube-linter held to what the other three are (ADR-018, ADR-111): SARIF only; not run is
  never a pass, with how to install it; no network; the app's own `.kube-linter.yaml` and its annotations unable to
  hide a check unseen (as hadolint's `-c` and Trivy's `--config` were handled); handed only the manifests `cv` read;
  each check id mapped to a requirement one at a time after reading its text, and a finding one of `cv`'s own rules
  also made counted once, naming the tools that agree.
- **Tried for real** on the owner's Mac, installed with Homebrew only after asking, as the other three were.
- **Witnessed:** each guard against the app's own settings removed in turn turns a test red.
- **Records:** ADR-157, a Later entry; `data/adapters.json`'s line in `data/README.md`; the site's credits (backlog 0703
  stops the build when they leave out a tool `cv` runs).
