# The amd64 image a computer pulls has no attestation of its own: its tag is an index, and the index is what is signed

**Status:** open

Filed 23 November 2026 by the cato-pipeline session, at the owner's word. That session keeps `sv` pinned by digest in
a continuous-authorization pipeline, and checks the image's build provenance with `cosign` before copying it into a
local registry. It found this while moving its pin to the image for commit 06f5032.

**What was seen.** For commit 06f5032 the registry holds three tags, read with the registry's own API:

| Tag | What it is | Digest | Attested? |
|---|---|---|---|
| `:06f5032…` | an index naming both platforms | `sha256:069d4bf8…9b61` | yes |
| `:06f5032…-amd64` | **an index**, naming the amd64 image and a build record (`attestation-manifest`) | `sha256:5fec1c2a…4b0d` | yes |
| `:06f5032…-arm64` | the arm64 image itself | `sha256:6fae5d15…6cc6` | yes |
| (no tag) | the amd64 image itself, named inside both indexes above | `sha256:99a93788…6e82` | **no** |

"Attested" is `cosign verify-attestation --type slsaprovenance1` (cosign 3.1.3) with the issuer
`https://token.actions.githubusercontent.com` and the identity `rust.yml` on `refs/heads/main`; the same command with
another workflow's identity was refused, so the check does tell the two apart.

**Why.** `rust.yml`'s `publish` job reads each platform's digest with `digest_of "$IMAGE:$sha-amd64"`, which is the
digest of the tag. For arm64 the tag is the image, so the image is signed. For amd64 the tag is an index with the image
inside it, so the index is signed and the image is not. Why the amd64 push makes an index and the arm64 push does not
was not found; both are built with `docker build`, saved, loaded, and pushed.

**What it means.** The comment above the signing steps says they sign "each platform's own, which is what
`docker pull` fetches on that machine". That holds for arm64 and not for amd64: an amd64 computer that pulls `:latest`
ends with the image `99a93788…`, and `gh attestation verify` or `cosign` given that digest finds nothing. Nothing here
shows the image is wrong. The signed index names it by its SHA-256, so someone who checks the index and pulls by the
index's digest is covered; someone who pins or checks the image's own digest, as a tool that resolves a platform first
does, is told there is no attestation. The check backlog 0741 suggests (`gh attestation verify
oci://…:latest --owner abbyshade111`) checks the index, so it passes either way and does not show this.

**To do.**
- Sign the image the amd64 tag names, read from the index (`.manifests[] | select(.platform.architecture == "amd64")
  | .digest`), in place of or beside the tag's own digest; or make the amd64 push a plain image as the arm64 one is,
  and say which was chosen on ADR-080.
- Hold it: after `publish`, a step reads the combined index and fails when a platform's image it names has no
  attestation. Break it on purpose (sign the tag's digest again) and watch that step go red.
- Correct the comment, and README's lines on checking the image if they promise more than is true for amd64.
