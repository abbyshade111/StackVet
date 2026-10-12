# The typed-text check on every text form (12 October 2026)

Backlog 0232, part 6 (group F of the one-sample inventory, in part), ADR-097, and ADR-053, "Later, 12 October 2026" (the
typed-text one).

**The problem.** `probe.text-rendered-as-markup` (V3.2.2) typed a line holding markup into the one form
`browser.text-form` names and read the one page that showed it. An app usually shows a person's text in several places.

**What was built.** `BrowserSection::text_forms` (`text-forms` in stackvet.toml), a list of `{ form, shows }`;
`BrowserSection::forms()` gives it, or the one `text-form` and `shows` as one entry. Both together are a problem said
when stackvet.toml is read. The browser job types into each form, with its own token (`browser::form_token`: the run's
own for one form, the run's plus `f<i>z` for several), and asks each page. `text_shown_as_text` now returns where the
text was shown as text, rather than giving the credit itself; findings stay per form. One credit names every form shown
as text, whole only when two or more were named and every one was.

**Tests.** `crates/sv-check/src/browser/tests/text_forms_tests.rs` (five) with two new switches in the fake browser
(`runs_for` and `drops_for`, a form's token); `crates/sv-manifest/tests/text_forms.rs` (three).

**Broken on purpose, each put back:** the list ignored (1 red); whole from one form (2 red); whole with a form not shown
(1 red); one token for every form (4 red); both keys accepted (1 red).

**Not built.** The storage check after signing out (one sign-in, one sign-out) stays in part.
