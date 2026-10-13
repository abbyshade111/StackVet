#!/usr/bin/env python3
"""Stands in for the owner in `sv review`, for the second loop trial only (ADR-109; the owner, 12 October 2026:
"scripted review is fine if labeled").

    python3 scripted_review.py SV APP_FOLDER
    python3 scripted_review.py --self-test

`sv review` runs only in a terminal someone is typing in, so this gives it one (a pseudo-terminal) and answers what it
asks. It confirms one thing, the `not-offered` list of the account changes (ADR-105), by typing `owner`; it makes the
signing key without a passphrase; and it presses Enter at every other question, which leaves that item as it was. The
key is the trial's own: the run script sets XDG_CONFIG_HOME to a folder of the trial's, so the owner's real key and
settings are never read or touched. Every result that rests on this says it was confirmed by a script standing in for
the owner, not by a person. Never used outside a trial.

What `sv review` printed, and what this typed, go to standard output, so the run keeps a record of each answer.
"""
import os, select, subprocess, sys, time

NOT_OFFERED = 'Type `owner` if this is your app and it has none of these'
PASSPHRASE = 'Protect the key with a passphrase?'
LIMIT_SECONDS = 300
SAME_PROMPT_LIMIT = 3


def answer(screen):
    """What to type at the question that ends `screen`: `owner` only for the not-offered list, `none` for the key's
    passphrase, and Enter (an empty line) for anything else."""
    last = screen[screen.rfind('\n\n') + 1:] if '\n\n' in screen else screen
    if NOT_OFFERED in last:
        return 'owner'
    if PASSPHRASE in last:
        return 'none'
    return ''


def waiting(screen):
    """Whether `sv review` has stopped at a question: each of its questions ends with `> `."""
    return screen.rstrip(' ').endswith('>') and screen.endswith(' ')


def run(sv, app):
    import pty
    leader, follower = pty.openpty()
    proc = subprocess.Popen([sv, 'review', app], stdin=follower, stdout=follower, stderr=follower, close_fds=True)
    os.close(follower)
    screen, since_answer, began, asked = '', '', time.time(), {}
    while proc.poll() is None and time.time() - began < LIMIT_SECONDS:
        ready, _, _ = select.select([leader], [], [], 1.0)
        if ready:
            try:
                chunk = os.read(leader, 4096).decode('utf-8', 'replace')
            except OSError:
                break
            sys.stdout.write(chunk)
            sys.stdout.flush()
            screen += chunk
            since_answer += chunk
            continue
        if not waiting(since_answer):
            continue
        # The same question again and again means an Enter is not taken there: end the review instead (end of input
        # is how a person stops it), so nothing is recorded by accident.
        key = since_answer.strip()[-200:]
        asked[key] = asked.get(key, 0) + 1
        if asked[key] > SAME_PROMPT_LIMIT:
            print('\n[scripted review: the same question three times; ending the review]', flush=True)
            os.write(leader, b'\x04')
            since_answer = ''
            continue
        typed = answer(since_answer)
        print(f'\n[scripted review typed: {typed!r}]', flush=True)
        os.write(leader, (typed + '\n').encode())
        since_answer = ''
    if proc.poll() is None:
        print('\n[scripted review: stopped after its time limit]', flush=True)
        proc.kill()
    code = proc.wait()
    print(f'\n[scripted review: sv review exited {code}]', flush=True)
    return code


def self_test():
    offered = ('stackvet.toml says nobody can change these in your app, because it has none: a phone number.\n'
               '  [stack.run.users] not-offered = ["phone"]\nYour AI coding tool usually writes this.\n'
               'Type `owner` if this is your app and it has none of these, to confirm; or press Enter to leave the '
               'list unconfirmed.\n> ')
    assert waiting(offered) and answer(offered) == 'owner'
    key = ('`sv review` signs what you record with a key of its own.\n'
           'Protect the key with a passphrase? Press Enter to choose one, or type `none` for none.\n> ')
    assert waiting(key) and answer(key) == 'none'
    other = ('A finding waits for your review.\nType `owner` if this is your own answer, to record it as yours; or '
             'press Enter to leave it.\n> ')
    assert waiting(other) and answer(other) == ''
    # An earlier question's text still on the screen does not decide a later one.
    assert answer(offered + 'owner\n  Confirmed.\n\n' + other) == ''
    assert not waiting('Reading the app...\n')
    print('scripted_review self-test passed')


if __name__ == '__main__':
    if '--self-test' in sys.argv:
        self_test()
    else:
        sys.exit(run(sys.argv[1], sys.argv[2]))
