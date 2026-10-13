# The helper images `sv run` starts beside the app, each pinned to its fingerprint (the `@sha256:` part) as well as its
# name, so a moved tag cannot change what runs (backlogs 0238 and 0317). This file is never built. It is the one place
# they are named: `crates/sv-run/src/docker.rs` reads it when `sv` is compiled, each line by its name after `AS`;
# CI fetches what it lists before the tests (`.github/workflows/rust.yml`); and Dependabot proposes a new fingerprint
# here each week (`.github/dependabot.yml`), which CI then runs the whole suite against before it can merge.
# Keep each line as `FROM <image> AS <name>`, with the four names below; a missing one stops `sv` compiling.

# The probes, and the health check: tiny.
FROM busybox:1.36@sha256:73aaf090f3d85aa34ee199857f03fa3a95c8ede2ffd4cc2cdb5b94e566b11662 AS probe
# The mail server the app is given when the probes read its email: Mailpit.
FROM axllent/mailpit:v1.31@sha256:b68349e3a014b90c5610bfb26b2ae36f3892d7b8cf25ee140c6c71c98d2fcf48 AS mail
# The test OpenID Connect provider, the test model, the name server, and the browser driver: a stock Node image.
FROM node:22-alpine@sha256:0a7108bf6c7bf5de370ffb1a3ed6be93d405b43ff159f681a8d18c0e2bc2e402 AS provider
# The headless browser. Its version is read from the tag, so a change of tag is a change of browser.
FROM chromedp/headless-shell:155.0.8059.26@sha256:47e3a0c0cbef29e321d16c893cd804996243f710ca7855770dd4f9c4cd2b8627 AS browser
