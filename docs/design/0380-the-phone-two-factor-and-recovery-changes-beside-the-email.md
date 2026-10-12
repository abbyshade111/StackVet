# The phone, two-factor, and recovery changes beside the email change (12 October 2026)


Backlog 0232 group G, the owner's word of 12 October 2026, recorded as ADR-105.

**What was missing.** V7.5.1 names four things a person must sign in again to change: the email address, the phone
number, the two-factor settings, and what recovers the account. `sv` tried the email address alone, and so credited
V7.5.1 in part at best, whatever the app did.

**What it does now.** Three keys sit beside `change-email` under `[stack.run.users]`: `change-phone`,
`change-two-factor`, and `change-recovery`. Each is a request with `{password}` and `{new_value}`, and `shows`, a private
page that lists the value once the change takes. One account made through `signup`, signed in once, asks for each
change with a wrong password and then the right one, opening `shows` after each. The new value seen after the wrong
password is a finding. Seen only after the right one, it is a credit, in part. Never seen, it is not assessed, and the
report says why. Nothing in a run signs in more than it did before: the three share one session, which keeps a run
inside the sixty sign-ins the spec states.

An app that has no phone number, no two-factor, or no way to recover an account says so in `not-offered`, and the
owner confirms it in `sv review`, as the level's two answers are confirmed. The report credits V7.5.1 whole only when
the email change and each of the other three is refused, or confirmed not offered, and says which parts rest on the
owner's word. A list nobody confirmed is a gap in the report, and counts for nothing.

**Why a page, not a sign-in.** The email check proves a change took by signing in with the new address. A phone
number, a two-factor number, or a recovery address is not a sign-in name, so the page that shows it is the only place a
change can be seen from outside. The values are random for each run, and different for each kind and each attempt,
because all three changes share one account and one page: the first build used the same digits for the phone and the
two-factor number, and the test saw the phone's number on the page and called it a two-factor change made without the
password. A phone number is matched by its digits, since apps format numbers their own way.

**What it cannot see.** A two-factor change that sets nothing, such as turning two-factor off; a value shown past the
first 4,000 characters of the page, or in a different form; and a change that waits for the new value to be confirmed
first. Each reads as a change that did not take, and the report says so, rather than as a pass.

**Broken on purpose, each put back.** The wrong attempt's value not looked for, and each flaw went unfound (one test
failed). The right attempt's control skipped, and a change that never takes was credited (one test failed). The
owner's confirmation not required (two tests failed). The confirmed list not compared with the list as it stands (one
test failed). The seal not checked, so a review written into the file by hand counted (one test failed, the one written
for that case after the others passed without it).
