# Sign-up checks fill one seed value into every account they make, so a one-use invite code fails all but the first

**Status:** open

Found in the owner's Cursor build of 22 October 2026 (backlog 0373), read in the app's folder on the owner's drive. The
app is invite-only, and its invites work once: `app.py` records who used one (line 637) and refuses one already used
(line 623). Its `stackvet.toml` passes the invite into the sign-up form as `invite = "{seed:INVITE}"`, and its
`seed.py` prints a single `INVITE=...`. `sv report --run` makes several accounts through that form (for the password
change, the email change, the guessing limit, and the role written into the form), each with the same seed value. The
first sign-up uses the invite up, and the rest were refused (422).

The report said what it could not do, honestly, but guessed the cause: "The sign-up form may need a value
stackvet.toml does not carry, such as an invite code." Here the value was carried; it had been used. Six checks were
not run on that account of things (V6.3.1, V6.2.2, V6.2.3, V7.4.3, V6.3.7, V7.5.1).

To do, with a decision record, since it changes what `stackvet.toml` can say and what `sv run` sends:

- a way for a seed to give one value per sign-up: for example a seed that prints `INVITE_1=`, `INVITE_2=`, and so on,
  used in turn by `{seed:INVITE}`, or a seed command run again for each account; and
- when a sign-up after the first is refused and the first was accepted with the same seed value, say that in the
  report: "the first sign-up was accepted and the next were refused, with the same `{seed:INVITE}`: a value that works
  once needs one per account (see the guide)"; and
- the setup prompts and the guide's `stackvet.toml` section saying so for invite-only apps.

The rate limit that held back other signed-in checks in the same run (10 new games a minute) is the app's own, and the
report handled it as it should: it waited as asked, then said which checks a refusal there could not credit. Nothing
to change in StackVet for that.
