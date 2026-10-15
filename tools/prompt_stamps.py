#!/usr/bin/env python3
"""Stamps each prompt on the setup page with the version of its text (backlog 0588).

    python3 tools/prompt_stamps.py           # writes the stamps that are missing or out of date
    python3 tools/prompt_stamps.py --check   # says which are, and exits 1 if any

A prompt is pasted into an AI coding tool, and a trial's record could say which text the tool read only by comparing
the pasted words with the page's history. So each prompt on docs/prompts/setup.md ends with one quiet line of its own,
which goes wherever the prompt is pasted:

    > *(StackVet setup prompt 5, text of 15 October 2026, fingerprint 1a2b3c4d)*

The fingerprint is the first eight hex characters of the SHA-256 of the prompt's words, the stamp left out: each
quoted line with its `>` and the spaces around it taken off, joined with single spaces. A commit cannot name itself,
so the line names its text instead; `git log -S "fingerprint 1a2b3c4d" -- docs/prompts/setup.md` finds the commit that
wrote it. The date is the day the stamp was last written, which is the day the text last changed. The guide's copy of
prompt 5 (its step 4) carries the same line. `crates/sv-cli/tests/setup_prompt_stamps.rs` holds both, computing the
fingerprint the same way, and its failure names this script.
"""
import datetime
import hashlib
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
PAGE = os.path.join(ROOT, 'docs/prompts/setup.md')
GUIDE = os.path.join(ROOT, 'docs/GETTING-STARTED.md')
GUIDE_STEP = '## 4. Start the build with this prompt'

# Each prompt's markers on the page, and the name its stamp gives it.
PROMPTS = [
    ('setup-step-1', 'prompt 1'),
    ('setup-step-2', 'prompt 2'),
    ('setup-prompt', 'prompt 3 for a Mac or Linux'),
    ('setup-windows', 'prompt 3 for Windows'),
    ('setup-step-4', 'prompt 4'),
    ('setup-step-5', 'prompt 5'),
    ('setup-step-6', 'prompt 6'),
    ('setup-step-7', 'prompt 7'),
]
STAMP = re.compile(r'^> \*\(StackVet setup (?P<name>.+), text of (?P<date>\d{1,2} \w+ \d{4}), '
                   r'fingerprint (?P<fp>[0-9a-f]{8})\)\*$')


def fingerprint(quoted):
    """The fingerprint of a prompt's quoted lines, any stamp among them left out."""
    words = ' '.join(line.lstrip('>').strip() for line in quoted if not STAMP.match(line))
    return hashlib.sha256(' '.join(words.split()).encode()).hexdigest()[:8]


def today():
    day = datetime.datetime.now(datetime.timezone.utc).date()
    return f'{day.day} {day.strftime("%B")} {day.year}'


def stamp_block(lines, name):
    """The quoted lines of one block with its stamp up to date, and whether that changed anything."""
    quoted = [line for line in lines if line.startswith('>')]
    fp = fingerprint(quoted)
    old = next((STAMP.match(line) for line in lines if STAMP.match(line)), None)
    if old and old['fp'] == fp and old['name'] == name:
        return lines, False
    body = [line for line in lines if not STAMP.match(line)]
    while body and body[-1].strip() == '>':
        body.pop()
    return body + ['>', f'> *(StackVet setup {name}, text of {today()}, fingerprint {fp})*'], True


def stamp_page(text):
    """The page with every prompt's stamp up to date, the names of those that changed, and prompt 5's stamp."""
    lines = text.split('\n')
    changed, five = [], None
    for marker, name in PROMPTS:
        start = lines.index(f'<!-- {marker}:start -->')
        end = lines.index(f'<!-- {marker}:end -->')
        block, moved = stamp_block(lines[start + 1:end], name)
        lines[start + 1:end] = block
        if moved:
            changed.append(name)
        if marker == 'setup-step-5':
            five = block[-1]
    return '\n'.join(lines), changed, five


def stamp_guide(text, five):
    """The guide with its copy of prompt 5 ending in prompt 5's stamp, and whether that changed anything."""
    lines = text.split('\n')
    at = lines.index(GUIDE_STEP)
    start = next(i for i in range(at, len(lines)) if lines[i].startswith('>'))
    end = next(i for i in range(start, len(lines)) if not lines[i].startswith('>'))
    block = [line for line in lines[start:end] if not STAMP.match(line)]
    while block and block[-1].strip() == '>':
        block.pop()
    block += ['>', five]
    if lines[start:end] == block:
        return text, False
    lines[start:end] = block
    return '\n'.join(lines), True


def main():
    check = '--check' in sys.argv[1:]
    page_text = open(PAGE, encoding='utf-8').read()
    guide_text = open(GUIDE, encoding='utf-8').read()
    new_page, changed, five = stamp_page(page_text)
    new_guide, guide_moved = stamp_guide(guide_text, five)
    if guide_moved:
        changed.append("the guide's copy of prompt 5")
    if check:
        for name in changed:
            print(f'out of date: {name}')
        if changed:
            print('Run python3 tools/prompt_stamps.py to write them.')
        return 1 if changed else 0
    if new_page != page_text:
        open(PAGE, 'w', encoding='utf-8').write(new_page)
    if new_guide != guide_text:
        open(GUIDE, 'w', encoding='utf-8').write(new_guide)
    print('stamped: ' + (', '.join(changed) if changed else 'nothing; every stamp was up to date'))
    return 0


if __name__ == '__main__':
    sys.exit(main())
