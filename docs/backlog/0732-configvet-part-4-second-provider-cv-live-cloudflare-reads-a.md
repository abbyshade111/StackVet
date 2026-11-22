# ConfigVet part 4, second provider: cv live cloudflare reads a zone's settings with a read-only token

**Status:** partly done: the ConfigVet page on stackvet.dev says Cloudflare is here

Filed 22 November 2026, at the owner's word ("Yes, file Cloudflare"). The second half of part 4 of ADR-157's build
order (the design entry `docs/design/0396-configvet-a-companion-for-an-app-s-deployment-configuration.md`, "Proposed
order of the build"): the live provider after GitHub, as the owner chose on 20 November 2026 (ADR-157, the owner's
answers), behind every rule of ADR-157's point 5. GitHub's half is backlog 0717, and its client, `crates/cv-live`, is
where this one goes. Drift between the files and the account is part 5, after this.

Why Cloudflare (ADR-157): its settings sit between the visitor and the app, so they speak to V12 and to the joins
between layers; its API tokens are built from named permissions, so a token can be made read-only; and stackvet.dev is
hosted on it, so the first client can be tried on a real account the project owns.

**Before any code: settle three questions from Cloudflare's own documentation, and record the answers as a Later entry
on ADR-157,** as 0717 did for GitHub.

- **Can ConfigVet tell that a token is read-only?** ADR-157 left this to be confirmed before the client is built: what
  Cloudflare lets a token show of itself (a token's verify endpoint, and whether it lists the token's permissions),
  for a user's API token and for an account's API token, and which cannot. A token that can be shown to write is
  refused; where Cloudflare cannot say, the report says so before any finding, and the guide says how to make a
  read-only token for ConfigVet (point 5). A Global API Key is refused outright: it can do anything the account can.
- **Which settings, and what each speaks to.** A first list to check against Cloudflare's API documentation, for one
  zone: the SSL/TLS mode (Off, Flexible, Full, Full (strict)), the lowest TLS version accepted, TLS 1.3, Always Use
  HTTPS, HSTS (its max-age, includeSubDomains, and preload), automatic HTTPS rewrites, and the cache rules, which carry
  V14.2.5's cache half (ADR-157, Later, 20 November 2026; backlog 0625, part 10). Each one's requirement is cited only
  after reading its text (CLAUDE.md, "A citation is a claim"); a setting a check in `sv` already reads stays `sv`'s.
  Flexible SSL, where the visitor's connection is encrypted and Cloudflare's to the app is not, is the setting most
  likely to be a finding; whether it can be told from the API alone is part of this question.
- **Where the token comes from.** Its name in the environment (Cloudflare's own tools read `CLOUDFLARE_API_TOKEN`), and
  whether any credential helper is worth reading; never an option, never written anywhere, shown only as
  `Secret::redact` shows a key.

**To build, once all three are recorded:**

- **`cv live cloudflare <zone>`,** a zone named as the person names it (`stackvet.dev`), off until asked for by name,
  in `crates/cv-live` beside GitHub's client and under its dependency test, so `sv`'s binary still cannot reach it.
- **A fixed list of `GET` requests compiled into the binary,** to `api.cloudflare.com` only, HTTPS with certificate
  checks, each redirect checked before it is followed, with limits on requests, response size, and time. No endpoint
  that returns a secret or a stored value (a Worker's secrets, a KV value, an origin certificate's private key).
- **The report:** each setting read, credited as the provider's account of its own settings (*checked* for its value
  only, never for the behavior it causes: ADR-157, point 6); every request listed with its method, host, path, and
  status; and what could not be read said as not assessed, never as a pass. The same report shape as GitHub's.
- **The fence:** `docs/ARCHITECTURE.md`, "The fence", gains `api.cloudflare.com` beside `api.github.com` for `cv live`,
  and `sv`'s list is unchanged; `docs/INSTALL.md` says how to make the read-only token; `SECURITY.md`, which names
  `cv live github` as the one way `cv` connects anywhere, names `cv live cloudflare` beside it.
- **Tested** as 0717's client is: a stand-in `curl` that records its arguments, with the token never on its command
  line, each guard broken in turn and the test that catches it named.
- **Tried first on stackvet.dev's own zone,** with a read-only token the owner makes; never on anyone else's account
  before that. Then the stackvet.dev ConfigVet page says Cloudflare is here, as 0722 did for GitHub.

**Records:** ADR-157, Later entries for the three answers and for "As built" part 4, Cloudflare. The token's handling
and the request list are its point 5; any change to either is a decision of its own.

**Built, 22 November 2026,** in the pull request that closes this in part: `cv live cloudflare <zone>` (ADR-157, "As
built, part 4, Cloudflare"). **Tried the same day on stackvet.dev** with the owner's read-only token (Zone Read, Zone
Settings Read, Cache Rules Read): ten requests, all answered. Two readings were corrected by what the site really sends
(the SSL/TLS mode where a Worker serves the zone; Cloudflare's HSTS off where the site sends its own header); the lowest
TLS version and Always Use HTTPS were real findings, which the owner changed in Cloudflare. Left: the stackvet.dev
ConfigVet page.
