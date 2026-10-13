# The helper images `sv run` starts beside the app, each pinned to its fingerprint (the `@sha256:` part) as well as its
# name, so a moved tag cannot change what runs (backlogs 0238 and 0317). This file is never built. It is the one place
# they are named: `crates/sv-run/src/docker.rs` reads it when `sv` is compiled, each line by its name after `AS`;
# CI fetches what it lists before the tests (`.github/workflows/rust.yml`); and Dependabot proposes a new fingerprint
# here each week (`.github/dependabot.yml`), which CI then runs the whole suite against before it can merge.
# Keep each line as `FROM <image> AS <name>`, with the four names below; a missing one stops `sv` compiling.

# The probes, and the health check: tiny.
FROM busybox:1.38@sha256:fd7dc98638c8e305f4dc34e979f1c0fdfdcaeb0fbf8fcff77ae834b6da3d7e6e AS probe
# The mail server the app is given when the probes read its email: Mailpit.
FROM axllent/mailpit:v1.31@sha256:b68349e3a014b90c5610bfb26b2ae36f3892d7b8cf25ee140c6c71c98d2fcf48 AS mail
# The test OpenID Connect provider, the test model, the name server, and the browser driver: a stock Node image.
FROM node:25-alpine@sha256:bdf2cca6fe3dabd014ea60163eca3f0f7015fbd5c7ee1b0e9ccb4ced6eb02ef4 AS provider
# The headless browser. Its version is read from the tag, so a change of tag is a change of browser.
FROM chromedp/headless-shell:151.0.7922.109@sha256:2d349b544a1ea6b5b5fd7c0fe99215ff662339c57407ee2e8c0a11af93516b04 AS browser
