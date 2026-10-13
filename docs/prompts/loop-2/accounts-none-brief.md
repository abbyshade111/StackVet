# Garden plots

I look after a community garden and want a simple web app where people manage their own account.

- Anyone can make an account at `/signup` with an email address and a password (fields `email` and
  `password`), and then signs in at `/login` with the same fields. They sign out with a button that posts
  to `/logout`.
- Someone who has forgotten their password asks for a reset at `/forgot` (a field `email`); the app
  emails them a link, and the link's page lets them choose a new password. Send email through the mail
  server at the address in the environment variables `SMTP_HOST` and `SMTP_PORT` (no encryption or
  sign-in needed); if they are not set, say that resets are not available.
- After signing in, a person lands on `/account`, which shows their email address.
- From `/account`, a signed-in person can change their email address. The app keeps no phone number,
  has no two-factor sign-in, and has no recovery address: the email address is the only way to reach
  someone.
- Each person can keep a private list of the plots they look after: a form posting a field `plot` to
  `/plots`, shown only to them on `/account`.
- Admins have a page at `/admin` listing everyone's email address.
- Anyone may see the home page `/`, which says what the garden is.

Technical requirements (these are fixed, because of where the app will be tested):

- Python 3.12, **standard library only** (no pip packages: nothing can be installed). One file,
  `app.py`, using `http.server`, listening on `0.0.0.0` at the port in the environment variable `PORT`
  (default 8080). Store data in SQLite at the path in the environment variable `GARDEN_DB` (default
  `/tmp/garden.db`; the app's own folder is read-only when it runs).
- I check my apps with StackVet (`sv`), which runs them in a container and tests them while they run.
  It needs a `stackvet.toml` in this folder describing the app; its specification is above.
- Keep the app in a git repository in this folder: commit your work as you go, and end each commit
  message with the line `Assisted-by: Claude Code`.
