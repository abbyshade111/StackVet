# A short walk-through of a setup, as a video or a click-through slide show

**Status:** done, 22 October 2026

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.

Asked for by the owner on 12 October 2026, from the website's words (backlog 0224, `docs/website/home-page.md`,
sections 1 and 7): "Watch a two-minute walk-through of a setup from start to finish," and "See how a build goes,"
for people who want to see it before trying. Either a short video or a click-through slide show of a real setup, made
after the setup prompt library is tried (it would show that), with every screen from a real run, not a mock-up.

**Note, 14 October 2026:** the owner tries the finalized prompt library in Antigravity, Cursor, and VS Code first (0502); the walk-through is made from one of those runs, so it shows the prompts as they are.

**The owner's answer, 15 October 2026:** after the paper's data (0568's tables and figures) is done.

**Checked against `main`, 20 October 2026** (the backlog clean-up): its conditions are met: the setup prompt library was tried (0370, 0502, 14 October) and the paper's data is done (0568, 15 October). Nothing is made yet. It waits on the owner: every screen comes from a real run, and the trial's records are on the owner's drive.

**The owner's material, 22 October 2026:** a new Cursor build made for this walk-through, from the setup prompts, kept
on the owner's drive with its chat, 31 screenshots, the app, and its report. Claimed to make the walk-through as a
click-through slide show from those screenshots, for stackvet.dev (built by `tools/website.py`, ADR-132), every screen
read first for anything personal. Its record is a Later entry on ADR-132, in the pull request that builds it, since it
changes what the site is built from.

**Partly done, 22 October 2026** (ADR-132, Later): the walk-through of a build, from the owner's screenshots of the
Cursor build, is the site's *How a build goes* (`docs/website/build-walkthrough-page.md`, seventeen screens, the
username covered in two), behind the home page's *See how a build goes*. At the owner's word: the build now, the setup
later, since the setup was not screenshotted. What remains is that walk-through of a setup.

Two corrections to the claim above, from reading the material: the screenshots are 28, not 31 (the other three image
files are icons inside a Python library in the app's folder); and the chat began with the owner's own request to
connect StackVet, not with a prompt from `docs/prompts/setup.md`.

**The owner's word, 22 October 2026:** "we can get rid of the 'coming later' for the video, I want to just start
with the walkthrough for now." Claimed again to finish it that way: the home page's "Want to see it first?" and its
*Watch the walk-through* lead to *How a build goes*, nothing is shown as coming, and the walk-through of a setup is
left for later, not a condition of going live.

**Done, 22 October 2026,** at the owner's word: the walk-through is the build's, *How a build goes*, and nothing on
the site is shown as coming. The home page's "Want to see it first?" in "Get started" now reads "Follow one real
build, screen by screen, from the first decisions to the report", and its button is *See how a build goes*, as the
opening's is; *Watch the walk-through* is gone from `tools/website.py`'s buttons. A walk-through of a setup is left
for later, and is no longer a condition of going live (`docs/website/home-page.md`, "Not yet true", item 4).
