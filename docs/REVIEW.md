# Recording your own word: `sv review` and its seals

This is the one full account of `sv review`: what it records, how its seal is made and checked, and how to set it up
on this computer, in the container, and in CI. The README and the guide each say it in a few lines and link here.

## Why the step exists

Your AI coding tool writes most of what goes into `stackvet.toml` and `security-notes.md`, and it rewrites code until
a warning stops. Writing `by = "owner"` into the file is the easiest way to stop one. So an entry counts as yours only
once **you** record it with `sv review`, which runs only in a terminal someone is typing in, something an AI coding
tool does not have. Until then it counts as the tool's word, a step lower, and the report says so. A line saying
`owner` that was never recorded this way still counts as the tool's word, because a tool trying to quiet a warning
could write that line too.

## What it records

- **A finding set aside** under `[[finding-review]]` in `stackvet.toml`, with a reason: a false alarm, or a risk you
  choose to live with for now. A real finding is fixed in the code and checked again; recording it does not make it
  go away.
- **An answer of the tool's that you looked at yourself and confirm.** The report then shows it as the tool's words a
  person confirmed, never as your own.
- **Your own answers:** a `[design]` answer or a `[checked-by-hand]` result written with `by = "owner"`, and a section
  of `security-notes.md` marked `Written by: owner`.
- **The two answers that set the app's level,** who uses the app (`[app] audience`) and what it holds about people
  (`[data] categories`). Your AI coding tool usually writes both. Once you confirm them, the report says the level
  rests on answers you confirmed, and when; change either later and they are unconfirmed again until you run
  `sv review` once more. Confirming them never changes the level: it says whose word the level rests on.

Your AI coding tool may write entries too, as proposals (`by = "ai-tool"`), and the report lists them, but a proposal
counts for nothing.

## How it goes

```text
 AI coding tool             you, at a terminal         sv review                    wherever the report is made
 ──────────────             ──────────────────         ─────────                    ───────────────────────────
 writes an entry as a
 proposal (by = "ai-tool",
 or "Written by: AI
 coding tool")
        │
        └──────────────────► run `sv review ./my-app`
                                      │
                                      └──────────────► shows each entry not yet
                                                       yours (the finding's line
                                                       of code, never a line that
                                                       may hold a key), asks your
                                                       name or `owner`
                                                              │
                                                       signs it with the key in
                                                       ~/.config/stackvet/
                                                       (review-signing-key), and
                                                       puts the key's public half,
                                                       with the app, on the list
                                                       ~/.config/stackvet/allowed_signers
                                                              │
                                                              └──────────────────► checks each signature against
                                                                                   the list: on this computer,
                                                                                   in the container (the list
                                                                                   mounted read-only), or in CI
                                                                                   (SV_TRUSTED_SEALS); a good one
                                                                                   counts as yours

 Side notes: a passphrase on the key means nothing can sign as you without it, your AI coding tool included.
 The seal cannot show who was at the keyboard (without a passphrase), or that a trusted key is yours.
```

What you record is written back into `stackvet.toml` with the date and a *seal*.

## The key, and its passphrase

`sv review` signs with a key of its own, kept in your own settings folder (`~/.config/stackvet/review-signing-key`),
outside the app. The first time, it makes the key and asks for a passphrase to protect it with: press Enter to choose
one, or type `none` to go without. With one, nothing can sign as you without it, your AI coding tool included, since
the passphrase is only in your head; you type it each time you run `sv review`. Without one, every entry the key
signed says so in the report, on the computer that holds the key.

## The list of trusted keys

A signature is checked with the key's *public half*, which can check a signature but never make one. `sv review` puts
that public half, with the app it may sign for, on a list of trusted keys beside the key
(`~/.config/stackvet/allowed_signers`), and shows you one line to give any other computer that should count what you
record: CI, the container, or a second computer of yours. The line is not a secret: anyone can read it, and nobody can
sign with it.

Wherever the report is made, it checks each signature against the list. An entry that was changed afterwards, never
signed, or signed with a key the list does not name, is a proposal again, and the report names the key it trusted and
where the list came from, so you can tell it is yours. A signature also names the app, so an entry copied into another
app is a proposal there. On your own computer that means the app's folder: an app moved to another folder is a
proposal until you run `sv review` in it. On a computer with no list at all the signature cannot be checked, so the
entry does not count, and the report says how to make it count.

## What it cannot show

Who was at the keyboard, unless your key has a passphrase; and that a trusted key is yours, since whoever can change the
list, or the repository variable, can add one. A tool set on faking it, running as you, could do either; it stops the
easy way, not every way. Keep the key file private.

## Setting it up where the report is made

**On this computer**, with `sv` installed, nothing more: `sv review ./my-app`, and the reports made here read the list
beside the key.

**With only Docker**, once, make the folder the key is kept in, so that it is yours and only yours, with the list in it
(made now, because a file that is not there when a container starts is made as a folder in its place, and `sv review`
can no longer add to it):

```bash
mkdir -p ~/.config/stackvet && chmod 700 ~/.config/stackvet && touch ~/.config/stackvet/allowed_signers
```

Then, each time, in a terminal in the app's folder (on Linux, add `--user "$(id -u):$(id -g)"` after `docker run`):

```bash
docker run --rm -it --network none -v "$PWD":"$PWD" -w "$PWD" -v "$HOME/.config/stackvet":/sv-config/stackvet -e XDG_CONFIG_HOME=/sv-config ghcr.io/abbyshade111/stackvet-sv review .
```

`-it` gives it your terminal to ask its questions in, and the second `-v` lets it keep its key in that folder on your
computer rather than inside the container, which is thrown away when it ends.

**The container that writes your reports** needs only the list, and never the key beside it: pass
`-e SV_TRUSTED_SEALS="$(cat ~/.config/stackvet/allowed_signers)"`, or mount the list alone, read-only, with
`-v "$HOME/.config/stackvet/allowed_signers":/sv-config/stackvet/allowed_signers:ro -e XDG_CONFIG_HOME=/sv-config`.
For the container your AI coding tool starts, the guide (`docs/GETTING-STARTED.md`, step 5) gives the whole
`.mcp.json` with that mount. `sv review` adds to the same file, so the container sees each app you add without a
change there.

**CI**: give it the one line `sv review` showed you as the variable `SV_TRUSTED_SEALS`. On GitHub: the repository's
Settings, then Secrets and variables, then Actions, then Variables, and pass
`SV_TRUSTED_SEALS: ${{ vars.SV_TRUSTED_SEALS }}` to the step that runs `sv`.

Entries recorded before 6 October 2026 were sealed another way; the README's last section says what to do with them.
