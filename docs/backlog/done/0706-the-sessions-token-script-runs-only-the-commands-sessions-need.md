# The sessions' token script runs only the commands sessions need, each with the permissions it needs

**Status:** done, 21 November 2026

Filed 21 November 2026, at the owner's word, from the security review of 21 November (finding A6, its first point).
`tools/session_token.py run` (ADR-158) passes any `gh` or `git` command through with the App's token, and every token
carries all of the App's permissions, so one command can do more than a session ever needs, including say the token.

- **To build:** `run` takes only the `gh` and `git` subcommands sessions use (opening, updating, and merging a pull
  request; pushing and fetching), and refuses every other, naming the ones it takes. Each token asks GitHub for only
  the permissions its command needs: a push gets contents to write, and workflows to write only when the owner's
  rule allows it and the session asks for it (`--workflows`); a pull request command gets pull requests to write. The
  command runs with `git`'s hooks and trace output switched off, so nothing it starts is handed the token or prints it.
  The self-test refuses each command left out; ADR-158 gains a Later entry; CLAUDE.md says the narrower use.
- **Not here:** A6's other points. Squash and rebase on `main` in `.github/rulesets.json` is a change to the
  repository's settings, the owner's to make; `fuzz/Cargo.lock`'s audit and the pinned rustup installer are items of
  their own; a cloud session's actions recorded under the owner's account is the gap ADR-158 already names.
