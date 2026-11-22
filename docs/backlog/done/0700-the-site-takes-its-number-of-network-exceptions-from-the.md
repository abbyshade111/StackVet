# The site takes its number of network exceptions from The fence, and refuses phrases that say fewer

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, from the whole-site review (backlog 0696). Two pages said less than the
network fence allows: How it works showed the paper's "the one network connection sv makes of its own", and the
check-a-project page said a run "talks to GitHub and GitLab only" while Semgrep fetched its rules in the same run.
The fence is tabled in `docs/ARCHITECTURE.md`, "The fence"; the site doesn't read it.

- **What to build:** `tools/website.py` reads the number of exceptions from that table and puts it in the home
  page's "There are four exceptions", as with the coverage numbers. `crates/sv-cli/tests/website.rs` refuses, in every
  page's shown words, phrases that say fewer than the table ("the one network connection", "talks to GitHub and GitLab
  only"), unless the same paragraph names the exception it leaves out. The paper's own text on How it works is shown
  under a note saying what changed (backlog 0696), so the test reads that note, not the paper.
- **Done when:** adding a row to "The fence" without changing the home page stops the build, and putting back
  check-a-project's old sentence turns the test red.
- **Order,** after 0696.

**Done, 22 November 2026:** the build holds the home page's exceptions paragraph to the fence table and stops on a difference, rather than writing the number in, since the paragraph names each one; `crates/sv-cli/tests/website_fence_words.rs` refuses phrases that say fewer. Read differently from the item in one place: the refused phrase is "nothing else leaves your computer", the old sentence's real fault, since the page's "talks to GitHub and GitLab only" is true of the download. ADR-132, Later, 22 November 2026.
