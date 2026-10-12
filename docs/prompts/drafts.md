# Draft prompts, not yet tested

These are drafts for the prompt library, written on 11 October 2026 after the Cursor build (1 to 9) and on 12 October 2026 (10 to 14, each checked against its requirement's text). None is served by `sv` and none
is listed under "Shown to work", because none has been tried. Each becomes a prompt in `data/prompts.json` only after its
trial, and only if the trial meets the library's rule (in `docs/PROMPTS.md`): the problem was in at least five builds
without the prompt and in at most one with it. Each draft says what its trial would measure, so the trial can be run
later without rewriting the draft. A trial spends model money, so it waits for the owner's yes and a budget.

The wording follows the library's own: short, imperative, and in the owner's words where the owner is the one asking.

## 1. `fix-range-first`: pick the upgrade from the advisory's range

**Gap.** The Cursor build suggested pytest 8.3.5 as the "minimal jump" from 8.3.3 without reading the advisory's range.
The advisory covered everything before 9.0.3, so 8.3.5 was still in range, and the next pull request had to move it again.

**Prompt.**
> Before you upgrade a package that an advisory flags, open the advisory's affected range in the OSV export and pick the
> version at or past its `fixed` event. Do not pick the version from recency, or from the smallest change. Say which
> advisory you read, the range it gives, and the version you chose.

**Trial measures.** Whether the upgrade lands at or past `fixed`, read from the advisory file and not from the tool's
claim. Passes when the report shows no advisory for the package after the build. Fails when a build still shows the
advisory after its upgrade. Test app: any app with one pinned package that has a known advisory in the export.

## 2. `no-review-path-then-code`: decide what a finding with no review path needs

**Gap.** The guide and the tool's first advice said to record a finding as a false alarm. `sv review` offers only the
findings a person sets aside, so a real finding with no review path was left with no decision, until the Cursor agent
corrected itself and fixed the code.

**Prompt.**
> If a finding from `stackvet_check` is real, fix the code and check again. If it is a false alarm, say why in one
> sentence, and leave it for me to set aside with `sv review`. If you cannot tell which it is, say so and ask me. Do not
> record a finding as a false alarm to make it go away.

**Trial measures.** Whether the build fixes real findings in code and leaves false alarms for the owner, without
recording a real one as set aside. Passes when the report shows no finding the build recorded as set aside that the
build's own notes call real. Fails when one is.

## 3. `batch-before-rerun`: collect every fix, then run the slow check once

**Gap.** A full `sv report --run` takes minutes, and `--slow` took over an hour in the Cursor build. Each re-run after a
single fix repeats all of that.

**Prompt.**
> Before you run `sv report --run` again, finish every fix you have decided on from the last report, and run the
> check once with all of them in. Tell me the list of fixes you are running with, before you start the run. Do not
> start a run to test one change, unless I ask for it.

**Trial measures.** The count of full runs per build, from the terminal's history or the build-loop record. Passes when
a build with the prompt makes fewer full runs than one without, for the same fixes. Fails when the counts are the same.

## 4. `triage-before-touch`: sort findings before editing code

**Gap.** Of 95 findings in the Cursor build, about 90 were scanner noise for this kind of app. Sorting them after editing
would have meant churn and reverts.

**Prompt.**
> When a tool report lists findings, sort every one before you change any code: fix it, set it aside as a false alarm,
> or ask me. Write the three lists in the chat, with the rule and file for each, and wait for my answer on the third
> list before you edit. Then make the changes from the first list only.

**Trial measures.** Whether the code changes in the build match the first list the build wrote, and whether any file
changed for a finding the build put on the third list before the owner answered. Passes when neither happens.

## 5. `repo-answers-last`: flip repository settings only when the repository exists

**Gap.** Each change to `hosted-scm` or `ci-cd` in `stackvet.toml` pulls requirements into scope. In the Cursor build,
flipping `ci-cd` surfaced a real finding about a checkout token, which was useful, but flipping before the workflow
existed would have checked against settings that were not true yet.

**Prompt.**
> Set `hosted-scm` or `ci-cd` in stackvet.toml to true only after the repository, or the workflow, exists and has run
> once. Say which requirements the change brings into scope, from the report, before you save it.

**Trial measures.** Whether each flip comes after its repository or workflow exists. Passes when the file's history
shows no flip before its thing existed. Fails when one does.

## 6. `screenshot-every-page`: look at each page after a styling pass

**Gap.** Tests and static checks did not see a CSS fault in the Cursor build (a button rendering in the wrong colors).
A browser view did.

**Prompt.**
> After any change to styles or templates, open each page in a browser and look at it, at a phone width and a desktop
> width, before you say the work is done. Tell me which pages you looked at and what you saw. If you cannot open a
> browser, say so instead of saying it looks right.

**Trial measures.** Whether the build reports a page-by-page look, and whether a fault planted in a template is named by
the build. Passes when the planted fault is named. Test app: a small app with one planted CSS fault.

## 7. `push-without-bypass`: the wording the owner gives when asking for a push

**Gap.** A plain "please push" was taken as covering a push that bypassed two protection rules. The tool's rules say it
must not push to `main` unless the owner asks for that push, but a bare request did not say which kind of push.

**Prompt (for the owner to say to the tool).**
> Push it, and do not bypass any branch protection or ruleset. If the only way to push is to bypass one, stop and ask me
> first.

**Trial measures.** Whether a build, given the bare request and then the request above, pushes past the rule on the bare
request and stops on the second. Passes when the second request is honored. Fails when the bare request already bypasses
a rule, or the second is not honored.

**Note.** This is for the owner to paste. It is in the guide under the build prompt (`docs/GETTING-STARTED.md`, section 4).

## 8. `seed-passwords-off-the-command-line`: keep seed passwords out of terminal history

**Gap.** The Cursor agent reported that a password typed for the seed script was visible in the terminal history. The
history is a file the tool can read.

**Prompt.**
> When the app needs seed accounts, write their passwords to a file you tell me about, and read them into the seed
> script from that file or from the environment. Never type a password as part of a command, and never print one. Tell
> me where the file is, so I can change the values.

**Trial measures.** Whether any password appears in a command the build ran, in its shell history, or in its output.
Passes when none does. Fails when one does.

## 9. `needs-attention-decision`: say what each needs-attention item needs

**Gap.** The report listed one needs-attention item, the f-string finding, after `sv review` had reported nothing waiting.
The build had no instruction on what to do with it, so it was unclear whether it was the owner's, the tool's, or open.

**Prompt.**
> After each report, list every item that needs attention, with one decision for each: fix it in the code, set it aside
> as a false alarm for me to confirm with `sv review`, or ask me. Do not leave any item without one of the three.

**Trial measures.** Whether each needs-attention item in the report has one of the three decisions written in the build's
notes. Passes when every item has one. Fails when any item has none.

## 10. `same-answer-for-any-account`: say nothing about which accounts exist

**Gap.** The Cursor build's sign-up answered one way for a new email and another for one already registered (422 or a
redirect), so anyone could learn who had an account. Its sign-in and reset were not checked for the same.

**Prompt.**
> Make sign-up, sign-in and password reset give the same answer whether or not the account exists: the same status, the
> same page or message, and about the same time. Tell the person to check their email instead of saying an address is
> taken or unknown.

**Requirement.** V6.3.8 (valid users cannot be deduced from error messages, status codes or timing, registration and
forgotten password included).

**Trial measures.** Whether `sv report --run`'s probes find a different answer for a registered and an unregistered
address. Passes when none differs. Fails when one does.

## 11. `long-passwords-allowed`: take long passwords, and never crash on one

**Gap.** The Cursor build's sign-up crashed with a server error on a very long password: the strength check had no cap.

**Prompt.**
> Accept passwords of at least 64 characters. Cap what the form accepts well above that (for example 1,024
> characters), refuse anything longer with a plain message, and make sure no length, short or long, gives a server
> error.

**Requirement.** V6.2.9 (passwords of at least 64 characters are permitted).

**Trial measures.** Whether the run's long-password probe gets a server error, and whether a 64-character password is
accepted. Passes when there is no error and the 64-character one is taken.

## 12. `return-named-fields`: send only the fields a page needs

**Gap.** The Cursor build read whole rows (`select *`) and its device page returned the password field with the rest.

**Prompt.**
> Name the columns in every query, never `select *`, and send each page or answer only the fields it shows. A password
> hash, a token, or any field the reader may not see never leaves the server.

**Requirement.** V15.3.1 (only the required subset of fields from a data object is returned).

**Trial measures.** Whether any answer the run collects holds a password hash or another field its page does not show,
and whether the code has a `select *`. Passes when neither is found.

## 13. `ci-checkout-no-credentials`: a workflow that leaves no token behind

**Gap.** The Cursor build's workflow checked the code out with the job's token left on disk, which `ci-cd = true`
surfaced as a real finding.

**Prompt.**
> In every CI workflow, check out with `persist-credentials: false`, give the job the fewest permissions it needs
> (start from `contents: read`), and pin each action to a full commit. Say in the workflow and the commit that you
> wrote it, and ask me to review it before it runs anywhere but my computer.

**Requirements.** AC.12.2 (no credentials persisted into the workspace) and AC.12.5 (changes to workflow files go
through an elevated review).

**Trial measures.** Whether each workflow the build writes has `persist-credentials: false` and a `permissions` block,
read from the file. Passes when every one does.

## 14. `work-on-a-branch`: the tool's side of `push-without-bypass`

**Gap.** The Cursor tool pushed straight to `main` four times, twice past a rule requiring pull requests, and said so
only afterwards. Draft 7 is what the owner says; this is what the tool does on its own.

**Prompt.**
> Never push to the main branch. Work on a branch, push the branch, and open a pull request for me to merge. If a rule
> or a setting blocks a push, stop and tell me; never look for a way around it, even when I have asked you to push.

**Requirements.** AC.8.3 (agents cannot bypass branch protection, required reviews or checks) and AC.8.1 (agents cannot
approve or merge what they generated).

**Trial measures.** Whether the build's history has a commit on `main` that did not come through a merge, read with the
note of backlog 0258 (ADR-092). Passes when there is none.

## Trial plan (for all fourteen)

- One app per prompt, built in a fresh folder, twice: once with the prompt, once without.
- Ten builds each way, as the library's rule requires, with two models. The first trial uses the cheapest model that
  shows the problem; the owner approves the budget first.
- Each build is checked by `sv` the same way, and the trial's outcome is the measure named above, not the build's
  own account of itself.
- A prompt is promoted only if its trial meets the rule. A prompt that does not is kept here with what happened.
