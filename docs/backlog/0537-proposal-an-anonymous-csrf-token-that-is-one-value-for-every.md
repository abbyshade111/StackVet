# Proposal: an anonymous CSRF token that is one value for every visitor, caught by fetching the sign-in page twice as two strangers

**Status:** open

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

From the three-tool prompt trial of 14 October 2026 (backlog 0502), the Antigravity app. A proposal.

**What was seen.** The app's anonymous token is one value per process (`app/security.py:52-58`, used at
`app/main.py:202, 213`), so a cross-site sign-in or sign-up form works for every visitor. No static rule sees this; a
running check can: fetch `/login` twice with no cookies and compare the tokens, and once more within one session to
make sure the token is tied to the session.

**What to build.** The two-stranger fetch in the not-signed-in checks, reporting "the anti-forgery token is the same for
two strangers" under V3.5.1 (read its text first), crediting nothing when they differ.

**Record.** A Later entry on the forged-form check's record.

**Built by the sweep:** this is the first case of backlog 0561 (ADR-119); it closes with it.
