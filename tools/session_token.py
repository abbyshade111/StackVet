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
    python3 -I tools/session_token.py run --workflows -- git push ...    # commits that change .github/workflows

`run` takes only the commands sessions need (`allowed()`, backlog 0706), and each one's token asks GitHub for only the
permissions that command needs; `git` runs with its hooks and its trace output off.
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
    # The key, a few kilobytes, fits in the pipe before `openssl` starts (a pipe holds 64 KB on macOS and Linux), so it
    # is written whole and the writing end closed; `openssl` then reads it to the end, and the message on its input.
    if len(key_pem) > 60_000:
        raise Stop("the keychain item is too large to be the App's key; the owner stores it again (ADR-158)")
    read_end, write_end = os.pipe()
    try:
        os.write(write_end, key_pem)
        os.close(write_end)
        write_end = None
        signed = subprocess.run(
            ["openssl", "dgst", "-sha256", "-sign", f"/dev/fd/{read_end}"],
            input=message, capture_output=True, pass_fds=(read_end,),
        )
    finally:
        for fd in (read_end, write_end):
            if fd is not None:
                os.close(fd)
    if signed.returncode != 0 or not signed.stdout:
        # openssl's message names no part of the key; it is shown so a broken key says what broke.
        raise Stop("openssl could not sign with the App's key: " + signed.stderr.decode(errors="replace").strip()[:200])
    return signed.stdout


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


def installation_token(repo, key_pem=None, permissions=None):
    """An hour's token for `repo` alone, from the App's installation there: with only `permissions`, when given."""
    if repo not in REPOSITORIES:
        raise Stop(f"{repo} is not one of the App's repositories: {', '.join(REPOSITORIES)}")
    jwt = app_jwt(key_pem if key_pem is not None else keychain_key())
    installation = call("GET", f"repos/{repo}/installation", jwt)
    body = {"repositories": [repo.split("/", 1)[1]]}
    if permissions is not None:
        body["permissions"] = permissions
    try:
        made = call("POST", f"app/installations/{installation['id']}/access_tokens", jwt, body)
    except Stop as stop:
        if permissions is not None and " 422 " in str(stop):
            raise Stop(f"the App is not given {permissions} on {repo}; the owner's settings decide what it may do "
                       f"(ADR-158). GitHub said: {stop}") from None
        raise
    return made["token"], made.get("expires_at", ""), made.get("permissions", {})


def command_env(token, base=None):
    """The environment the one command runs in: the token for `gh`, and a credential helper for `git`."""
    env = dict(os.environ if base is None else base)
    env["GH_TOKEN"] = token
    for name in list(env):
        # The token goes to `gh` and `git` alone: nothing that prints what they send, or runs in their place.
        if name in UNSAFE_ENV or name.startswith("GIT_TRACE"):
            env.pop(name)
    env["GIT_TRACE_REDACT"] = "1"
    # An empty helper first clears the ones already set (the owner's `gh auth git-credential`), so `git` asks this one.
    # Hooks and a file monitor are programs `git` would start with the token in their environment: none are run.
    count = int(env.get("GIT_CONFIG_COUNT", "0"))
    settings = [("credential.https://github.com.helper", ""), ("credential.https://github.com.helper", HELPER),
                ("core.hooksPath", os.devnull), ("core.fsmonitor", "false")]
    for i, (key, value) in enumerate(settings, start=count):
        env[f"GIT_CONFIG_KEY_{i}"] = key
        env[f"GIT_CONFIG_VALUE_{i}"] = value
    env["GIT_CONFIG_COUNT"] = str(count + len(settings))
    env["GIT_TERMINAL_PROMPT"] = "0"
    return env


# The commands a session runs as the App, and the permissions each one's token asks for (backlog 0706). Everything
# else is refused: a command left out could say the token, hand it to a program the command starts (a `gh` extension
# or alias, a `git` hook), or use a permission the work never needed. Reading may use the owner's own sign-in.
READ = {"contents": "read", "pull_requests": "read"}
GH = {
    ("pr", "create"): {"contents": "read", "pull_requests": "write"},
    ("pr", "edit"): {"contents": "read", "pull_requests": "write"},
    ("pr", "comment"): {"contents": "read", "pull_requests": "write"},
    ("pr", "ready"): {"contents": "read", "pull_requests": "write"},
    ("pr", "close"): {"contents": "read", "pull_requests": "write"},
    ("pr", "merge"): {"contents": "write", "pull_requests": "write"},  # Merging writes the base branch.
    ("pr", "view"): READ,
    ("pr", "list"): READ,
    ("pr", "checks"): READ,
}
GIT = {
    "push": {"contents": "write"},
    "fetch": {"contents": "read"},
    "ls-remote": {"contents": "read"},
}
# Variables that make `gh` or `git` print what they send, or run another program in their place.
UNSAFE_ENV = ("GH_DEBUG", "DEBUG", "GIT_CURL_VERBOSE", "GIT_EXEC_PATH", "GIT_ASKPASS", "SSH_ASKPASS", "GITHUB_TOKEN")


def allowed():
    """The commands `run` takes, as a person reads them."""
    return ", ".join([f"gh {a} {b}" for a, b in GH] + [f"git {c}" for c in GIT])


def plan(argv):
    """The repository, the command, and the permissions its token asks for; or a Stop naming what `run` takes."""
    repo, workflows = DEFAULT_REPO, False
    while argv[:1] in (["--repo"], ["--workflows"]):
        if argv[0] == "--workflows":
            workflows, argv = True, argv[1:]
        elif len(argv) < 2:
            raise Stop("--repo needs owner/name")
        else:
            repo, argv = argv[1], argv[2:]
    if repo not in REPOSITORIES:
        raise Stop(f"{repo} is not one of the App's repositories: {', '.join(REPOSITORIES)}")
    if argv[:1] == ["--"]:
        argv = argv[1:]
    refused = Stop(f"run takes only these commands after `--`: {allowed()}")
    if argv[:1] == ["gh"]:
        permissions = GH.get(tuple(argv[1:3]))
        if permissions is None or len(argv) < 3:
            raise refused
    elif argv[:1] == ["git"]:
        # `git -C <folder>` is the one option taken before the command; `-c` could set a helper or a hook back.
        rest = argv[1:]
        while rest[:1] == ["-C"] and len(rest) > 1:
            rest = rest[2:]
        permissions = GIT.get(rest[0]) if rest else None
        if permissions is None:
            raise refused
    else:
        raise refused
    permissions = dict(permissions, metadata="read")
    if workflows:
        if permissions.get("contents") != "write" or argv[:1] != ["git"]:
            raise Stop("--workflows is for a `git push` whose commits change .github/workflows")
        permissions["workflows"] = "write"
    return repo, argv, permissions


def run(argv):
    repo, argv, permissions = plan(argv)
    token, _, _ = installation_token(repo, permissions=permissions)
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
    env = command_env("tok_" + "x" * 8, base={"PATH": os.environ.get("PATH", ""), "GITHUB_TOKEN": "other",
                                               "GIT_TRACE_CURL": "1", "GH_DEBUG": "api"})
    assert env["GH_TOKEN"] == "tok_xxxxxxxx" and "GITHUB_TOKEN" not in env
    assert "GIT_TRACE_CURL" not in env and "GH_DEBUG" not in env and env["GIT_TRACE_REDACT"] == "1"
    assert env["GIT_CONFIG_COUNT"] == "4" and env["GIT_CONFIG_VALUE_0"] == ""
    assert "tok_" not in env["GIT_CONFIG_VALUE_1"], "the helper holds the token itself"
    answered = subprocess.run(["git", "credential", "fill"], input=b"protocol=https\nhost=github.com\n\n",
                              env=env, capture_output=True)
    assert b"password=tok_xxxxxxxx" in answered.stdout, answered.stdout + answered.stderr
    # A hook in the folder `git` runs in is not started: the token never reaches it.
    with tempfile.TemporaryDirectory() as folder:
        subprocess.run(["git", "init", "-q", folder], check=True, env=env)
        hook = os.path.join(folder, ".git", "hooks", "pre-commit")
        with open(hook, "w", encoding="utf-8", newline="\n") as f:
            f.write("#!/bin/sh\ntouch \"$(dirname \"$0\")/ran\"\n")
        os.chmod(hook, 0o755)
        committed = subprocess.run(["git", "-C", folder, "-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q",
                                    "--allow-empty", "-m", "t"], env=env, capture_output=True)
        assert committed.returncode == 0, committed.stderr
        assert not os.path.exists(os.path.join(folder, ".git", "hooks", "ran")), "a hook ran with the token"
    # Each command's token asks for what it needs and nothing more; workflows only when asked, and only for a push.
    assert plan(["--", "git", "push", "-u", "origin", "HEAD"])[2] == {"contents": "write", "metadata": "read"}
    assert plan(["--workflows", "--", "git", "-C", "x", "push"])[2]["workflows"] == "write"
    assert plan(["--", "gh", "pr", "create", "--fill"])[2] == {"contents": "read", "pull_requests": "write",
                                                             "metadata": "read"}
    assert "workflows" not in plan(["--", "gh", "pr", "merge", "1", "--auto"])[2]
    assert plan(["--repo", "abbyshade111/scoop-stack-vet", "--", "gh", "pr", "view"])[0] == "abbyshade111/scoop-stack-vet"
    # Everything else stops before a token is made: another repository, another program, a `gh` or `git` command
    # left out (one that says the token among them), an option that sets git's helpers or hooks, and --workflows
    # where no push is.
    for argv in (["--repo", "someone/else", "--", "gh", "pr", "list"], ["--", "curl", "https://example.com"],
                 ["--", "gh", "auth", "token"], ["--", "gh", "auth", "status", "--show-token"], ["--", "gh", "api", "user"],
                 ["--", "gh", "extension", "exec", "x"], ["--", "gh", "pr"], ["--", "gh", "pr", "checkout", "1"],
                 ["--", "gh", "run", "rerun", "1"], ["--", "git", "credential", "fill"], ["--", "git", "config", "-l"],
                 ["--", "git", "-c", "core.hooksPath=.githooks", "push"], ["--", "git"], ["--", "git", "-C"],
                 ["--workflows", "--", "gh", "pr", "create"], ["--workflows", "--", "git", "fetch"]):
        try:
            plan(argv)  # Never `run`: a command let through would read the owner's keychain and reach GitHub.
            raise AssertionError(f"took {argv}")
        except Stop as stop:
            assert argv[0] == "--repo" or "--workflows" in argv or "git push" in str(stop), (argv, stop)
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
