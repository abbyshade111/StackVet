# ConfigVet part 4: `cv live github` reads a repository's settings with a read-only token

**Status:** claimed by Claude Code session (packaging and ConfigVet), 21 November 2026

Filed 21 November 2026, at the owner's word. Part 4 of ADR-157's build order (the design entry
`docs/design/0396-configvet-a-companion-for-an-app-s-deployment-configuration.md`, "Proposed order of the build"):
the first live provider, GitHub, behind every rule of ADR-157's point 5. Cloudflare comes after it (ADR-157, the
owner's answers); drift between the files and the account is part 5.

**Before any code: settle two questions, and record the answers as a Later entry on ADR-157.**

- **Can ConfigVet tell that a token is read-only?** The design entry says GitHub's fine-grained tokens can be made
  read-only "and ConfigVet can check that". That is to be confirmed from GitHub's own documentation before the client
  is built, as ADR-157 asked of Cloudflare: which token kinds say their permissions (a classic token's
  `X-OAuth-Scopes` header; what a fine-grained token or a GitHub App's installation token shows), and which cannot.
  A token that can be shown to write is refused; where GitHub cannot say, the report says so before any finding,
  and the guide says how to make a read-only fine-grained token for ConfigVet (point 5).
- **Which settings, and what each speaks to.** A first list to check against GitHub's REST documentation: the rules
  on the default branch (rulesets and branch protection), the default permissions of a workflow's `GITHUB_TOKEN`,
  whether workflows may approve pull requests, which actions may run, each environment's protection rules and
  deployment branches, and whether secret scanning and push protection are on. Each one's requirement is cited only
  after reading its text (CLAUDE.md, "A citation is a claim"); a setting a check in `sv` already reads from the files
  stays `sv`'s (the design entry: one rule has one home).

**To build, once both are recorded:**

- **A crate of its own for the live clients,** which `sv-cli` does not depend on, and a test that reads the
  workspace's dependency graph and fails if `sv`'s binary reaches it, shown to work by adding the dependency and
  watching it go red (ADR-157, point 4).
- **`cv live github <owner>/<repo>`,** off until asked for by name. The token from `GH_TOKEN` or `GITHUB_TOKEN`, or
  from `gh auth token` only if the owner chooses that when asked; never from an option, never written anywhere, shown
  only as `Secret::redact` shows a key.
- **A fixed list of `GET` requests compiled into the binary,** to `api.github.com` only, HTTPS with certificate checks,
  each redirect checked before it is followed, with limits on requests, response size, and time. No endpoint that
  returns a secret's value; an environment variable's or secret's name at most.
- **The report:** each setting read, credited as the provider's account of its own settings (*checked* for its value
  only, never for the behavior it causes: ADR-157, point 6); every request listed with its method, host, path, and
  status; and what could not be read said as not assessed, never as a pass.
- **The fence redrawn:** `docs/ARCHITECTURE.md`'s "The fence", the picture and the table, gains `cv live` beside `sv`'s
  exceptions, and `sv`'s list is unchanged.
- **Tried first on this project's own repository,** with a read-only fine-grained token the owner makes; never on
  anyone else's account before that.

**Records:** ADR-157, Later entries for the two answers and for "As built" part 4. The token's handling and the
request list are its point 5; any change to either is a decision of its own.

**The two questions, answered 21 November 2026** (ADR-157, Later): a classic token says its scopes and is refused when
any allows writing; a fine-grained token cannot say, so the report says ConfigVet could not confirm it is read-only
before any finding. The settings list, each with what it speaks to in part, is in the same entry. Next: the crate and
its dependency test, then the client, tried on this repository with a read-only fine-grained token the owner makes.

