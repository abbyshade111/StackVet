#!/usr/bin/env python3
"""Runs one `gh` or `git` command as the sessions' GitHub App, so a session's pull request is the App's (ADR-158).

Sessions on the owner's Mac used to open pull requests signed in as the owner, and GitHub lets no author approve their
own pull request, so the code-owner review of ADR-051 never held a session's change back (backlog 0692). The App is
the owner's, installed on this repository, the Scoop bucket, and the Homebrew tap, with contents, pull requests, and
workflows to write and nothing else. Its private key is in the owner's macOS keychain, put there by the owner.

Each run reads the key from the keychain, signs a ten-minute JSON Web Token with it through `openssl` (the key goes
to `openssl` through a pipe, never into a file), trades that for an installation token GitHub makes for one hour and
one repository, and runs the command with the token in its environment alone: `GH_TOKEN` for `gh`, and a credential
helper for `git` that reads it from there. The key and the token are never printed, logged, or written. With no key, or
no token, it says so and stops; it never falls back to the owner's own sign-in.

It opens connections to api.github.com only. It is a tool for this repository's upkeep, not part of `sv`, whose
fence (docs/ARCHITECTURE.md) is unchanged.

    python3 -I tools/session_token.py run -- gh pr create ...            # this repository
    python3 -I tools/session_token.py run --repo abbyshade111/scoop-stack-vet -- git push -u origin HEAD
    python3 -I tools/session_token.py check                              # what the App can reach, nothing secret
    python3 -I tools/session_token.py --self-test                        # a throwaway key; never the real one
"""

import base64
import json
import os
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request

APP_ID = "5862353"  # Not a secret: it names the App, and is shown on its public settings page.
KEYCHAIN_SERVICE = "stackvet-sessions-app"
DEFAULT_REPO = "abbyshade111/StackVet"
REPOSITORIES = ["abbyshade111/StackVet", "abbyshade111/homebrew-stackvet", "abbyshade111/scoop-stack-vet"]
API = "https://api.github.com/"
# `git` asks this helper for a password; it answers with the token from the environment it was given, so the token is
# on no command line and in no file.
HELPER = '!f() { test "$1" = get || exit 0; echo username=x-access-token; echo "password=$GH_TOKEN"; }; f'


class Stop(Exception):
    """A reason the command cannot run as the App, said to the person in plain words."""


def b64url(data):
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()


def sign(message, key_pem):
    """The RS256 signature of `message`, by `openssl`, which reads the key from a pipe."""
    read_end, write_end = os.pipe()
    try:
        process = subprocess.Popen(
            ["openssl", "dgst", "-sha256", "-sign", f"/dev/fd/{read_end}"],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, pass_fds=(read_end,),
        )
        os.close(read_end)
        read_end = None
        with os.fdopen(write_end, "wb") as pipe:
            write_end = None
            pipe.write(key_pem)
        signature, error = process.communicate(message)
    finally:
        for fd in (read_end, write_end):
            if fd is not None:
                os.close(fd)
    if process.returncode != 0 or not signature:
        # openssl's message names no part of the key; it is shown so a broken key says what broke.
        raise Stop("openssl could not sign with the App's key: " + error.decode(errors="replace").strip()[:200])
    return signature


def app_jwt(key_pem, now=None, app_id=APP_ID):
    """A JSON Web Token for the App: issued a minute ago (GitHub's advice, for a clock a little ahead), ten minutes long."""
    now = int(now if now is not None else time.time())
    header = b64url(json.dumps({"alg": "RS256", "typ": "JWT"}, separators=(",", ":")).encode())
    payload = b64url(json.dumps({"iat": now - 60, "exp": now + 540, "iss": app_id}, separators=(",", ":")).encode())
    message = f"{header}.{payload}".encode()
    return f"{header}.{payload}.{b64url(sign(message, key_pem))}"


def keychain_key():
    """The App's private key, from the item the owner stored (ADR-158), as the bytes of its PEM file."""
    found = subprocess.run(
        ["security", "find-generic-password", "-s", KEYCHAIN_SERVICE, "-a", APP_ID, "-w"],
        capture_output=True,
    )
    if found.returncode != 0:
        raise Stop(
            f"no key for the App in the keychain (service {KEYCHAIN_SERVICE!r}, account {APP_ID!r}). The owner "
            "stores it (ADR-158); until then sessions do not open pull requests, and do not use the owner's sign-in."
        )
    try:
        return base64.b64decode(found.stdout.strip(), validate=True)
    except ValueError:
        raise Stop("the keychain item is not the base64 of a key file; the owner stores it again (ADR-158)") from None


def call(method, path, bearer, body=None):
    request = urllib.request.Request(
        API + path, method=method, data=None if body is None else json.dumps(body).encode(),
        headers={"Accept": "application/vnd.github+json", "Authorization": "Bearer " + bearer,
                 "X-GitHub-Api-Version": "2022-11-28"},
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as answer:
            return json.load(answer)
    except urllib.error.HTTPError as error:
        # GitHub's message, never the request: the request's header holds the token.
        detail = error.read().decode(errors="replace")[:300]
        raise Stop(f"GitHub refused {method} {path}: {error.code} {detail}") from None
    except urllib.error.URLError as error:
        raise Stop(f"GitHub could not be reached ({error.reason}); outside the sandbox, the network is needed") from None


def installation_token(repo, key_pem=None):
    """An hour's token for `repo` alone, from the App's installation there."""
    if repo not in REPOSITORIES:
        raise Stop(f"{repo} is not one of the App's repositories: {', '.join(REPOSITORIES)}")
    jwt = app_jwt(key_pem if key_pem is not None else keychain_key())
    installation = call("GET", f"repos/{repo}/installation", jwt)
    made = call("POST", f"app/installations/{installation['id']}/access_tokens", jwt,
                {"repositories": [repo.split("/", 1)[1]]})
    return made["token"], made.get("expires_at", ""), made.get("permissions", {})


def command_env(token, base=None):
    """The environment the one command runs in: the token for `gh`, and a credential helper for `git`."""
    env = dict(os.environ if base is None else base)
    env["GH_TOKEN"] = token
    env.pop("GITHUB_TOKEN", None)  # `gh` prefers GH_TOKEN; another token beside it would only confuse which is used.
    # An empty helper first clears the ones already set (the owner's `gh auth git-credential`), so `git` asks this one.
    count = int(env.get("GIT_CONFIG_COUNT", "0"))
    for i, value in enumerate(["", HELPER], start=count):
        env[f"GIT_CONFIG_KEY_{i}"] = "credential.https://github.com.helper"
        env[f"GIT_CONFIG_VALUE_{i}"] = value
    env["GIT_CONFIG_COUNT"] = str(count + 2)
    env["GIT_TERMINAL_PROMPT"] = "0"
    return env


def run(argv):
    repo = DEFAULT_REPO
    if argv[:1] == ["--repo"]:
        if len(argv) < 2:
            raise Stop("--repo needs owner/name")
        repo, argv = argv[1], argv[2:]
    if argv[:1] == ["--"]:
        argv = argv[1:]
    if not argv or argv[0] not in ("gh", "git"):
        raise Stop("run takes a `gh` or `git` command after `--`")
    token, _, _ = installation_token(repo)
    return subprocess.call(argv, env=command_env(token))


def check():
    """What the App can reach in each repository, read with its own tokens; nothing secret is shown."""
    key = keychain_key()
    failed = False
    for repo in REPOSITORIES:
        try:
            token, expires, permissions = installation_token(repo, key)
        except Stop as stop:
            print(f"{repo}: no token ({stop})")
            failed = True
            continue
        seen = call("GET", "installation/repositories", token)
        names = [r["full_name"] for r in seen.get("repositories", [])]
        granted = ", ".join(f"{k} {v}" for k, v in sorted(permissions.items()))
        print(f"{repo}: a token until {expires}, reaching {', '.join(names) or 'nothing'}; {granted}")
        if "administration" in permissions:
            print(f"{repo}: the App can change settings, which ADR-158 says it must not", file=sys.stderr)
            failed = True
    return 1 if failed else 0


def self_test():
    with tempfile.TemporaryDirectory() as folder:
        private, public = os.path.join(folder, "k.pem"), os.path.join(folder, "k.pub")
        subprocess.run(["openssl", "genpkey", "-algorithm", "RSA", "-pkeyopt", "rsa_keygen_bits:2048",
                        "-out", private], check=True, capture_output=True)
        subprocess.run(["openssl", "pkey", "-in", private, "-pubout", "-out", public], check=True, capture_output=True)
        with open(private, "rb") as f:
            key = f.read()
        jwt = app_jwt(key, now=1_000_000, app_id="42")
        header, payload, signature = jwt.split(".")
        unpad = lambda s: base64.urlsafe_b64decode(s + "=" * (-len(s) % 4))  # noqa: E731
        assert json.loads(unpad(header)) == {"alg": "RS256", "typ": "JWT"}
        assert json.loads(unpad(payload)) == {"iat": 999_940, "exp": 1_000_540, "iss": "42"}
        # The signature is the key's: openssl checks it with the public half.
        with open(os.path.join(folder, "sig"), "wb") as f:
            f.write(unpad(signature))
        checked = subprocess.run(["openssl", "dgst", "-sha256", "-verify", public, "-signature",
                                  os.path.join(folder, "sig")], input=f"{header}.{payload}".encode(),
                                 capture_output=True)
        assert checked.returncode == 0, checked.stdout + checked.stderr
        # A key that is not one stops with a reason, and the reason does not hold the key.
        try:
            app_jwt(b"not a key at all")
            raise AssertionError("a broken key signed")
        except Stop as stop:
            assert "not a key" not in str(stop), stop
    # The command's environment has the token for gh, and git's helper reads it from there, not from its own text.
    env = command_env("tok_" + "x" * 8, base={"PATH": os.environ.get("PATH", ""), "GITHUB_TOKEN": "other"})
    assert env["GH_TOKEN"] == "tok_xxxxxxxx" and "GITHUB_TOKEN" not in env
    assert env["GIT_CONFIG_COUNT"] == "2" and env["GIT_CONFIG_VALUE_0"] == ""
    assert "tok_" not in env["GIT_CONFIG_VALUE_1"], "the helper holds the token itself"
    answered = subprocess.run(["git", "credential", "fill"], input=b"protocol=https\nhost=github.com\n\n",
                              env=env, capture_output=True)
    assert b"password=tok_xxxxxxxx" in answered.stdout, answered.stdout + answered.stderr
    # Only the App's repositories, and only gh or git.
    for argv in (["--repo", "someone/else", "--", "gh", "pr", "list"], ["--", "curl", "https://example.com"]):
        try:
            run(argv)
            raise AssertionError(f"ran {argv}")
        except Stop:
            pass
    print("session_token self-test: ok")
    return 0


def main(argv):
    try:
        if argv[:1] == ["--self-test"]:
            return self_test()
        if argv[:1] == ["check"]:
            return check()
        if argv[:1] == ["run"]:
            return run(argv[1:])
        print(__doc__.strip().splitlines()[0], file=sys.stderr)
        print("usage: session_token.py run [--repo owner/name] -- gh|git ... | check | --self-test", file=sys.stderr)
        return 2
    except Stop as stop:
        print(f"session_token: {stop}", file=sys.stderr)
        return 3


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
