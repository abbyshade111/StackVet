# A setup prompt library for people new to building with AI

**Status:** open

Asked for by the owner on 12 October 2026, from the website's words (backlog 0224, `docs/website/home-page.md`,
section 7). The site's "Get started" has two paths, and the first, for someone who has used an AI chat but never
built an app with one, points at **setup prompts**: each is copied into an AI coding tool that can run commands, and
the tool does the setup with the person, checks what the computer already has, says what to install and where to get
it, asks before it installs or changes anything, and stops and says what happened in plain words when a step fails.

Today there is one prompt, `docs/prompts/setup.md`, not yet tried in any AI tool, and it stops on Windows ("StackVet
has not been tried on Windows yet"), which is out of date since Docker and Scoop work there (backlog 0120). To build:
the library, a prompt per step where one prompt is too much at once, Windows taken in; then **the owner tests it**,
and the tools it really worked in are the ones the site names. Until then the site's line naming them stays blank.
