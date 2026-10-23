# Build stackvet.dev from the settled words and direction C

**Status:** done, 22 October 2026

**Was queued on the coordination board** (backlog 0273) until the board was removed on 20 October 2026 (ADR-061, Later); it is now taken the normal way, by a claim.

From backlog 0224 (done 12 October 2026): the home page's words and its look are settled in
`docs/website/home-page.md`, direction C in the logo's own colors. To build: the page itself, the credits page, and
stackvet.app redirecting to stackvet.dev, with the coverage numbers read from `docs/COVERAGE.md` when the site is built
rather than copied in. Where it is hosted, and how the owner's two addresses point at it, are the owner's to choose
(hosting may cost money, so it is asked first).

**It does not go live** while any of backlog 0370 to 0374 is open: the setup prompts tried and the tools named
(0370), the report leading with what to fix (0371), requirements in plain language (0372), the walk-through (0373),
and every credited license confirmed (0374). The research section stays a placeholder until the owner's paper is
published. Nothing is live from this item until the owner says so.

**Two more pages, at the owner's word on 12 October 2026:**
- **How StackVet works**, for people who want the detail: built from the inventory of what a run gathers against what
  the report shows (the owner: "this inventory can serve dual purposes"), and from `docs/paper/HOW-SV-WORKS.md`.
- **StackVet checks itself**: StackVet's own report on its own repository (`docs/paper/self-assessment-v2`), its SBOM
  (backlog 0382), and the rest, linked to where each lives on GitHub.

**The owner's answer, 15 October 2026:** after the paper's data is done.

**Checked against `main`, 20 October 2026** (the backlog clean-up): every condition to build is met (0224, 0433, 0370 to 0374, 0568); nothing is built. It now also has the reports page (0413) and the framework pages (0619, `tools/framework_pages.py`, ADR-129) to take in. Hosting and the two addresses are the owner's to choose, and nothing goes live while 0373 is open or before the owner's word. `docs/website/home-page.md`'s list of what is not yet true is brought up to date with this clean-up.

**Record:** ADR-132, proposed with this claim.

**The owner's answer, 22 October 2026,** asked where the site should be hosted: "Build first then do some research and
provide up to 5 options, a mix of free and low cost, with the pros and cons and your recommendation". So this claim
builds the site into the repository (`exports/site/`, beside the framework pages), with nothing published, and then
brings the owner up to five hosting options to choose from.

**Built, 22 October 2026** (ADR-132, accepted): `python3 tools/website.py` writes the whole site into
`exports/site/`, which git ignores, and publishes nothing: the home page, How StackVet works, the reports page with
the example report, StackVet checks itself, and the credits, with the framework pages in `standards/`. The words are
read from `docs/website/`, with the notes on them left out; the numbers from `docs/COVERAGE.md`; and the AI tools the
setup prompts were tried in from `docs/prompts/setup.md`. `crates/sv-cli/tests/website.rs` holds it to them.

In the same pull request, at the owner's word: a new self-assessment, StackVet's report on its own repository held to
level 3, in `docs/website/self-assessment/` (its small files only), with its 312 things to fix to be gone through in
backlog 0633; level 3 rows in the home page's "What it checks"; and the opening conversation's lines, proposed in
`docs/website/home-page.md` for the owner to settle.

What remains: hosting, which the owner chooses from up to five options, free and low cost, brought after this build;
stackvet.app pointed at stackvet.dev; and going live, after the walk-through (backlog 0373) and the owner's word.

**The owner's answers, 22 October 2026:** hosting on Cloudflare, where both addresses are already registered ("Let's
go with Cloudflare, the domains are registered via Cloudflare already"), and the real Cursor conversation on the home
page, with a VS Code line on How StackVet works. The build writes the `_headers` file Cloudflare reads. What remains:
publishing (a GitHub Actions job that builds with `tools/website.py` and deploys, with a Cloudflare token the owner
makes and stores as a secret, which is the owner's to do; and a redirect rule on stackvet.app's zone), and going live,
after the walk-through (backlog 0373) and the owner's word.

**22 October 2026:** the walk-through (backlog 0373) is done as the build's, at the owner's word, so going live waits
only on publishing and the owner's word.

**The owner's word, 22 October 2026:** "go ahead and write the deploy job. I added the API token and the A record for
.app, waiting to do the redirect rule." Both secrets are in the repository (`CLOUDFLARE_API_TOKEN`,
`CLOUDFLARE_ACCOUNT_ID`). Claimed to write the deploy: a workflow the owner starts by hand (`workflow_dispatch`
only), which builds the site with `tools/website.py` and publishes `exports/site/` to Cloudflare as a Worker of static
files at stackvet.dev, with Cloudflare's own action pinned by commit. Writing it publishes nothing: the first run the
owner starts is going live. The redirect rule on stackvet.app stays the owner's to add.

**Record, proposed:** a Later entry on ADR-132 (the deploy, its new dependency on `cloudflare/wrangler-action`, what
it uploads, and that only the owner starts it), made accepted in the pull request that builds it.

**The deploy, written 22 October 2026** (ADR-132, Later): `.github/workflows/site.yml` builds the site and publishes it
to Cloudflare at stackvet.dev when the owner starts it from the Actions tab, and only then; a check job with no secret
reads the deploy without publishing on pull requests that change the site. What remains is the owner's: starting it,
which is going live, and the redirect rule on stackvet.app.

**Done, 23 October 2026 (UTC; the evening of 22 October, the owner's time): stackvet.dev is live.** The owner reviewed
and merged the deploy (#2232), set its account secret right after the first run refused it (an account ID with a
character it may not hold), and started **Actions → Site**: the second run uploaded 58 files and attached stackvet.dev
as the Worker's custom domain (version `e81470b8-c3aa-4c29-bab7-8c9d0b589d59`). Its last step, asking stackvet.dev
for its first page, failed: Cloudflare's Bot Fight Mode was on in the stackvet.dev zone and answered every request
that was not a browser with a challenge (`cf-mitigated: challenge`; the zone's firewall events named it), the
runner's and `curl`'s alike. The owner turned it off. Then, checked from the session's computer:

- stackvet.dev answers 200 with the site's headers: the content security policy with its one script's hash,
  `Strict-Transport-Security`, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, the permissions
  policy, and `Cross-Origin-Opener-Policy: same-origin`;
- every one of the 58 files the site serves answers 200 (after Cloudflare's redirect of `.html` pages to the same
  address without it), and the folders' markers and `_headers` answer 404;
- in a browser, every screenshot of *How a build goes* loads, the policy blocks what the site did not write (the
  browser's own errors quote it), and the standards pages' one script runs (the level filter);
- stackvet.app, www.stackvet.app, and www.stackvet.dev each redirect to stackvet.dev keeping the path, by the
  owner's redirect rules.

The managed rules the zone keeps were seen blocking a scanner's requests for `/wp-config.php` the same hour; they are
separate from Bot Fight Mode and stay on.
