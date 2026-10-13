# An empty bind address is every interface, as 0.0.0.0 is

**Status:** open

Found 13 October 2026 in the owner's trial in Antigravity. Bandit's B104 flagged `HOST = os.environ.get("HOST",
"0.0.0.0")` in the app's `server.py`; the AI tool changed the default to `""`, and the finding went away. In Python an
empty host given to a server (`HTTPServer(("", port), ...)`, `socket.bind(("", port))`) binds every interface, exactly
as `"0.0.0.0"` does, so the app behaved as before and nothing reported it.

To build: StackVet's own rules recognize an empty host, and `"::"`, as binding every interface wherever they recognize
`"0.0.0.0"`, in the languages whose servers read an empty host that way (Python's `socket` and `http.server` first),
with a fixture for each form and a deliberate break. And find out whether StackVet has its own rule for an app that
listens on every interface, or relies on Bandit's alone; if alone, decide whether it needs one, and what requirement,
if any, it really speaks to: listening on every interface is often intended (a phone on the same Wi-Fi), so it may be a
note for the owner's decision rather than a finding.
