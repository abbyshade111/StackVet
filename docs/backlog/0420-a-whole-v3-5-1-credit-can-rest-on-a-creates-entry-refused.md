# A whole V3.5.1 credit can rest on a `creates` entry refused for any reason: no control shows that request works with its token

**Status:** claimed by honesty, 13 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the deep review of 13 October 2026 (backlog 0417), area: the last 48 hours' code, `sv-check` (checks' honesty).
Severity: medium. Verified against the code by the reviewing session before filing.

**What is wrong.** The forgery check sends each `creates` entry with a signed-in user's cookies from another origin and
without a token, and reads anything that is not `accepted()` (200 to 399) as the forgery refused. The `owned` create
has a control: user A made that record just before, in the same session, so a refusal of the forged copy means
something. A `[users] creates` entry has none. The forged copy is sent with empty values (an empty `{marker}`, an empty
`{csrf}`), so an app that answers 400 to an empty field, 404 to a mistyped path, 405 to a form POST where it wants
JSON (the case 0208 found for `update` and `delete`), or 422 is read as refusing the forgery; and `accepted` is false
for no answer at all, so a crash or a timeout reads as a refusal too (the principle 0280 fixed past the sign-in
limit). One such entry beside the `owned` create gives `refused_at.len() == 2`, and with no other changing request
named the credit is whole: "2 requests that create records, each sent ... from another origin and without a token,
and refused". Before 0232 part 3 the credit was always in part, so the same answer could not read as plain *checked*.

**Where.** `crates/sv-check/src/signed_in/forgery.rs:79-108` (the loop; `accepted` is `signed_in/mod.rs:542-546`),
`:149-172` (`.in_part_if(refused_at.len() == 1 || !untried.is_empty())`); `signed_in/admin.rs:1047-1053`
(`owner_field_creates`, the only earlier send of a `creates` entry with its token, only for an entry with `read`, its
result not carried); `signed_in/mod.rs:2310` (`burst_check` sends every create with its token, but runs after).

**Fix.** Count a `creates` entry's refusal only beside a control: the same request sent with the token and the
session, accepted (2xx, or a redirect that is not to the sign-in page), either sent here first or carried from
`owner_field_creates` or `burst_check`. An entry with no accepted control goes to `not_assessed` by path and leaves
the credit in part; no answer at all is never a refusal. Must not lose: the forged send making a record only, never
a change; the `owned` create's existing control; the finding naming every accepted path. Fake-app witness: a
`creates` entry that answers 405 to everything, which today makes the credit whole.

**Record.** ADR-053 (what counts as evidence for the signed-in credits, group C's Later entry of 12 October): a Later
entry.
