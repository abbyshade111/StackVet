# `sv review`'s key prompts led the owner to save the word "none" as the passphrase, and the unlock prompt gives no way out

**Status:** claimed by wording, 15 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found by the owner on 15 October 2026 in the follow-up model trial (backlog 0577): the first `sv review` made the key;
the second, in Cursor's terminal, refused the passphrase typed. Severity: high (the person's own key, locked by a
misunderstanding the prompts invited; the review is the step that makes answers theirs).

**What happened.** `crates/sv-cli/src/review.rs:252`: "Protect the key with a passphrase? Press Enter to choose one,
or type `none` for none." The natural answer to a yes/no question is Enter, which here means "yes, choose one"; the
next prompt is "Passphrase:", where a person who wanted no passphrase types the word they were just told, `none`,
then types it again at "The same passphrase again:" (`:263-281`), and the key is made with the passphrase `none`.
The report then says "a key on this computer with a passphrase". At the next review, the unlock prompt (`:225`)
refuses what the person types and asks again, with no word on what to try or what to do if the passphrase is lost.

**Fix.**
- The first question becomes a plain yes/no with the safe default spelled out: "Protect the key with a passphrase?
  (yes/no, Enter for yes)". No word to type that is also a plausible passphrase.
- At "Passphrase:", if the person types `none`, `no`, or an empty line, ask "Did you mean no passphrase? (yes/no)"
  rather than accept the word; and say in the prompt that the passphrase can be anything but those words.
- The unlock prompt, after one refusal, says: "If you typed `none` when the key was made, that word is the passphrase.
  If it is lost, move the key aside (the command) and run sv review again: a new key is made, and the seals made with
  the old one keep counting as long as its public half stays in the trusted list."
- One test per prompt path, including the `none`-typed-as-passphrase path, which today makes a key with that word.
The owner's wish of 15 October: fix it.

**Record.** ADR-043 (the signing key) Later: what the prompts say and that a passphrase cannot be one of the refusal
words.
