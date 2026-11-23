# Nightly: Rust fails: publish, Sign the amd64 image (Sigstore tlog 404)

**Status:** open

Recorded by the nightly CI routine on 23 November 2026. This routine records; it does not fix.

**What failed.** The Rust workflow (`rust.yml`), on push to `main` at commit 2e86c6d3 (22 November, 12:26 UTC, the
merge of #2531, the previous nightly CI record, which changed only `docs/backlog/`):
https://github.com/abbyshade111/StackVet/actions/runs/45022259537. Every build and test job passed; the `publish` job
failed at its step "Sign the amd64 image" (`actions/attest`). The arm64 image's signature, the step before it, was
uploaded to the Sigstore transparency log and to the repository. The amd64 one then stopped with:

    ##[error]InternalError: error fetching tlog entry - (404) Not Found

So the signing service did not hand back the log entry it had been asked for. Earlier in the same job the log also shows
a `digest-mismatch: error` line, in a step's listed settings; whether it bears on the failure was not checked.

**Was it there before.** No. The Rust run before it on `main` (9091bb3e, 05:01 UTC, run 44987593039) passed, and every
Rust run on `main` after it passed, its `publish` job included (for example 1e1186af, 15:31 UTC, run 45038774982), as did
last night's scheduled run (47533b11, run 45105711169). The change it ran on touched no code. It looks like a passing
fault in the Sigstore service rather than in this repository, but that was not proved here.

**What it means.** The `latest` image pushed by that run may lack its amd64 signature, though the next successful
`publish` replaced it. Nothing merged turned `main` red, so no merging session owes a fix.

**What to do.** Check that the current `latest` image's amd64 attestation is present (`gh attestation verify
oci://ghcr.io/abbyshade111/stackvet-sv:latest --owner abbyshade111`). If this failure comes back, consider retrying the
signing step once, without lowering what it checks; if it does not, close this as a passing outage.
