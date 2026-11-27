# The helper images `sv run` starts beside the app, each pinned to its fingerprint (the `@sha256:` part) as well as its
# name, so a moved tag cannot change what runs (backlogs 0238 and 0317). This file is never built. It is the one place
# they are named: `crates/sv-run/src/docker.rs` reads it when `sv` is compiled, each line by its name after `AS`;
# CI fetches what it lists before the tests (`.github/workflows/rust.yml`); and Dependabot proposes a new fingerprint
# here each week (`.github/dependabot.yml`), which CI then runs the whole suite against before it can merge.
# Keep each line as `FROM <image> AS <name>`, with the four names below; a missing one stops `sv` compiling.

# The probes, and the health check: tiny.
FROM docker.io/library/busybox:1.38@sha256:a46257e06c82ad85242d0d38077d0a52d9896389ca60a1a59d3463db586e5fa4 AS probe
# The mail server the app is given when the probes read its email: Mailpit.
FROM docker.io/axllent/mailpit:v1.34@sha256:23a2ef85bc50853ab3093645d1d64679afd8887e89da5e48a88940c172591a4e AS mail
# The test OpenID Connect provider, the test model, the name server, and the browser driver: a stock Node image.
FROM docker.io/library/node:24-alpine@sha256:116b800c3754b440ee0aa3ed2f20f5bf3d99a554bfd9adeea09ea8017f6e85f5 AS provider
# The headless browser. Its version is read from the tag, so a change of tag is a change of browser.
FROM docker.io/chromedp/headless-shell:159.0.8144.0@sha256:691a2f658d68e9855d4be3611b6818378c40bb92a0381a1a239db5c021117e9f AS browser
