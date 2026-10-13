# An account change's form token is read from the change's own address, and the refusal is blamed on the page

**Status:** open

From the second loop trial (backlog 0257, `docs/prompts/loop-2/results.md`), 13 October 2026.

In four Sonnet 5.5 builds of the accounts brief, every account change was reported as not taking, with the reason
"the page writes it differently, shows it past the first 4,000 characters, or waits for it to be confirmed first". The
kept exchanges show the real chain in all four: `sv` read the form token (`{csrf}`) from the change's own address
(`GET /account/email`, answered 405, because these apps put every change form on `/account`), sent the change without
a valid token, and the app refused it, rightly (`POST /account/email`, 403, "The form expired"). The account changes
(ADR-105) were never tried in those builds, and the gap blamed the page.

Two parts: read the token from the page that holds the form (the `shows` page, or a `form-page` key) when the change's
own address has none; and when a change is refused before it is tried (403, 405, 429), say that, not that it did not
take.

**The same fault in the password reset, found the same night.** In 16 of the 40 accounts builds the reset was reported
as "Using the code from the reset email through /reset did not change the password". Read by hand in `sonnet-b1-8`:
the app shows its reset form, and its token, only at the emailed link (`/reset?token=...`); plain `/reset` answers
"This link is not valid" with no form, so `sv` found no token there, and the app refused the reset (403, "Your form
expired"). The other fifteen are not yet read. For `reset.use`, the form token should come from the page at the
emailed link, where a person would find the form. Every request template with `{csrf}` reads its token the same way,
so the fix is one place.

