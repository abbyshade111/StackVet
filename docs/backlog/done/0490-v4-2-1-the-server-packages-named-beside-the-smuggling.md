# V4.2.1: the server packages named beside the smuggling question, and why nothing in the fence can settle it

**Status:** done, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V4 API and Web Service (V4.2.1)**

- **V4.2.1** (L2), every component "determine[s] boundaries of incoming HTTP messages" to prevent request smuggling.
  Design question exists. Route 2 is tempting (send a request with both `Content-Length` and `Transfer-Encoding` to
  the fenced app) but the components the requirement names, load balancers and front proxies, are not in the fence, so
  a result says nothing about production and would read as more than it is; not proposed. Route 1: nothing to read.
  Route 4: the pointer is the web server package and version from the bill of materials (gunicorn, uvicorn, node's
  `http`, puma, nginx in a Dockerfile) shown beside the question, with a note that a known smuggling bug in it would
  already appear from `--advisories`. Tier: *attested by the owner*. The partial-checks entry proposes the advisory
  match, which needs no new check. Honest answer for the rest: a manual check at the hosting platform, with the reason
  "the proxy in front of the app is not in the folder".

**The owner's choice, 14 October 2026:** build it, after 0495; the package and version beside the question, the plain reason the fence cannot settle it, and the requirement stays the owner's word.

**The orchestrator's answer to honesty's board question of 19:56, 15 October 2026:** 0490 goes back to honesty, whose branch `claude/honesty-0490-server` holds the whole build, tested; open the pull request from that branch. Documentation starts its lane at 0493.

**Built, 15 October 2026 (honesty):** the web server package and its version from the bill of materials beside V4.2.1
(a package locator, `locate.web-server`, in `data/locators.json`), with the sentence that it is where to look and not
evidence, a pointer to `--advisories` for a known flaw in that version, and the plain reason nothing StackVet does can
check it (`whyNotChecked` in `data/design-questions.json`). The requirement stays the owner's word. ADR-022, Later
entry of 15 October 2026. Not covered: Node, Go, Java, and PHP apps, and nginx named in a Dockerfile.

**Closed by the board, 15 October 2026:** built by honesty and merged (PR 2081); left open for the board to close.
