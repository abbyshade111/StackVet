# Mock-ups of the website: what stackvet.dev could look like and say

**Status:** done, 12 October 2026

Asked for by the owner on 9 October 2026, for after the roadmap: the owner holds `stackvet.dev` and `stackvet.app`,
and nothing is on either yet. Start with what the site is for and what it says before how it looks: who arrives
(a person building an app in an AI coding tool, not a programmer), the one sentence that tells them what `sv`
does, the honest report as the thing to show, getting started in the fewest steps, the paper, and where the code
is. Then mock-ups: a few pages as artifacts or plain HTML the owner can open, using the logo files from item 0219,
with two or three directions to choose between rather than one finished design. The owner picks; nothing goes
live from this item.

**From backlog 0219, 12 October 2026:** the two sites, stackvet.dev and stackvet.app (0219 part 3), are part of this item, at the owner's word; a session can take it while another builds the loop trial.

**Claimed 12 October 2026 by session securevibe-trial-prep**, at the owner's word ("start drafting the website
content for 0224 and then I can help be the test user"). The plan, in order:
- **The content first**, as a page the owner can read and mark up: who arrives, the one sentence, what a report says,
  getting started, the paper, and where the code is. Every claim in it checked against `main` and the guide.
- **The owner reads it as the test user**, someone who is not a programmer, and says what is unclear or missing.
- **Then two or three mock-up directions** with the logo (0219), for the owner to choose between. Nothing goes live.

**The words settled 12 October 2026**, read section by section by the owner as the test user, and kept in
`docs/website/home-page.md`. The owner's choices, in the order they came:
- **The opening** leads with using StackVet from the first prompt, not checking afterwards: "Vibe-code your app with
  security in mind from the first prompt" (the owner's headline; "built-in security" was set aside as reading like a
  promise that the app is secure). The report is described as written once the app works, not as a live view.
- **"How it works"** is written for someone who has used an AI chat but never built an app with one.
- **The report leads with the most important things to fix** and states every requirement in plain language, with
  what could not be checked highlighted for follow-up (backlog 0371 and 0372; not yet true of the report).
- **The numbers are ASVS levels 1 and 2** (153 of 253), with a note that level 3 is left out so far, since "174 of
  345" read as worrying. **Secure by Design** is "built into planning", not checked.
- **The AI tool's own access** to every file in the app's folder is said plainly, beside "your code stays on your
  computer".
- **Two ways to get started**: setup prompts for people new to AI (backlog 0370, which the owner tests, and the tools
  it works in are the ones named), and the guide for people who have used a terminal or Docker. A short
  walk-through as a video or slide show (backlog 0373).
- **The MIT license explained in a sentence**; contact through GitHub issues only, for now.
- **Credits** for all three OWASP frameworks, Semgrep, and the other outside tools and data, garak left out as decided
  (backlog 0140), each license confirmed before launch (backlog 0374).
- **The research section stays blank** until the owner's paper, for a master's program through SANS, is published.
- **stackvet.dev is the site; stackvet.app redirects to it.**

The site does not go live while 0370 to 0374 are open. Next in this item: two or three mock-up directions with the
logo, for the owner to choose between.

**The look chosen 12 October 2026.** Three directions were made on a design canvas (A, the report up front, dark;
B, the plain guide, light; C, the conversation, whose opening is a short exchange between a person and their AI tool
with StackVet in it), and then C in five palettes side by side. The owner chose **direction C in the logo's own
colors** ("it helped to see them side by side"); the choice, its type, and its colors are in
`docs/website/home-page.md`, "The look". With the words and the look settled, this item is done. Building the site
from them is backlog 0376, which waits for 0370 to 0374, since the site does not go live saying what is not yet true.
