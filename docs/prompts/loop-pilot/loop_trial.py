#!/usr/bin/env python3
"""Builds the club app with a headless AI coding tool, one build per call, for the loop trials.

    python3 loop_trial.py OUT ARM MODEL N [--budget USD] [--api] [--with-spec] [--prompt ID] [--allow-git]
                          [--draft N] [--model-id ID] [--label NAME] [--brief PATH]

MODEL names the build (`haiku`, `sonnet`); `--model-id` is the exact model the build runs on (`claude-haiku-5-5`), so
a nickname the Claude program resolves for itself cannot choose another model (the drafts trial, backlog 0257).
`--draft N` adds draft N of docs/prompts/drafts.md, as `--prompt` adds a library prompt.

Follows docs/prompts/loop-protocol.md. OUT is a folder under the home folder (Colima shares only that). Each build
gets a fresh folder OUT/<model>-<arm>-<n>, the request (docs/prompts/trial-3/plain-brief.md) as its prompt, and,
in the arms with the server, `sv mcp --root <that folder>` attached for this run only. Nothing in the person's
settings is read or changed: --restricted ignores the settings files and confines the file tools to the build's
folder, --strict-mcp-config uses no MCP server but this one, and --no-session-persistence keeps the run out of the
session list. The transcript (stream-json) is written beside the folder, not in it, so `sv` does not read it.
"""
import json, os, re, shlex, subprocess, sys, time

CLAUDE = os.environ['CLAUDE_BIN']      # the Claude Code program to run, by its full path
SV = os.environ['SV']                  # the sv release build every build is checked and helped by
REPO = os.environ['SV_REPO']           # a checkout of SecureVibe at that build's commit

ARMS = {
    # arm: (attach the server, the server's tools it may use, put the instructions in the request)
    'none': (False, [], False),
    'loop': (True, ['mcp__stackvet'], False),
    'instructions': (False, [], True),
    'check': (True, ['mcp__stackvet__stackvet_spec', 'mcp__stackvet__stackvet_check'], False),
    'plan': (True, ['mcp__stackvet__stackvet_spec', 'mcp__stackvet__stackvet_plan'], False),
    # Amendment 4 (item 6): the same two arms with the server's other tools hidden (--disallowedTools), not only
    # refused, after the refused builders gave up on every SecureVibe tool, the allowed ones included.
    'checkhidden': (True, ['mcp__stackvet__stackvet_spec', 'mcp__stackvet__stackvet_check'], False),
    'planhidden': (True, ['mcp__stackvet__stackvet_spec', 'mcp__stackvet__stackvet_plan'], False),
}
# The server is attached as `stackvet`, and its tools are `stackvet_*` (ADR-062). Until the second loop trial
# (ADR-109) it was attached as `securevibe` and the arms named `securevibe_*` tools, which the server answers as
# old names: the trials before then are read with both (loop_measures.py).
SV_TOOLS = ['before', 'bundle', 'check', 'explain', 'guidance', 'notes_file', 'plan', 'preflight', 'prompts',
            'questions', 'record_answer', 'spec', 'write_report']
# Amendment 1 (5 October 2026, after the pilot): a headless build has no owner to answer it, and the Haiku pilot
# builds stopped to ask. Every build from then on ends its request with this, in every arm. Amendment 5 (6 October
# 2026, after item 6, where 2 of 35 Haiku builds still stopped to ask): the stronger wording, for every trial after.
NO_OWNER = ("\n\nI won't be around to answer questions while you build, so do not stop to ask me anything. Where "
            "something needs deciding, choose the safer option, write down what you chose, and keep going until the "
            "app is built and runs.\n")
FILE_TOOLS = ['Read', 'Write', 'Edit', 'Glob', 'Grep']
SHELL = 'Bash(python3:*)'
# With --api, the Claude program gets the key from this helper when it needs it (apiKeyHelper), so the key is never in
# the environment of anything the builder starts: not its shell, not the app, not the sv server.
KEY_HELPER = os.environ.get('KEY_HELPER', '')


def instructions(folder):
    """The MCP server's opening instructions, as it gives them, for the arm that has them in the request."""
    hello = json.dumps({'jsonrpc': '2.0', 'id': 1, 'method': 'initialize',
                        'params': {'protocolVersion': '2025-11-25', 'capabilities': {},
                                   'clientInfo': {'name': 'loop-trial', 'version': '0'}}})
    out = subprocess.run([SV, 'mcp', '--root', folder], input=hello + '\n', capture_output=True, text=True,
                         timeout=60).stdout
    return json.loads(out.splitlines()[0])['result']['instructions']


def draft_prompt(number):
    """The prompt text of draft `number` in docs/prompts/drafts.md: its section's quoted lines after **Prompt**."""
    text = open(os.path.join(REPO, 'docs/prompts/drafts.md'), encoding='utf-8').read()
    section = re.search(rf'^## {number}\. .*?(?=^## |\Z)', text, re.S | re.M)
    if not section:
        raise SystemExit(f'no draft {number} in docs/prompts/drafts.md')
    after = section.group(0).split('**Prompt', 1)[1]
    quoted = []
    for line in after.splitlines()[1:]:
        if line.startswith('>'):
            quoted.append(line[1:].strip())
        elif quoted:
            break
    if not quoted:
        raise SystemExit(f'draft {number} has no quoted prompt')
    return ' '.join(quoted)


def build(out, arm, model, n, budget, api):
    attach, sv_tools, with_instructions = ARMS[arm]
    # A prompt arm is named after its prompt, with `_` for `-`, so a build's name still splits as model-arm-n.
    label = ('p_' + sys.argv[sys.argv.index('--prompt') + 1].replace('-', '_')) if '--prompt' in sys.argv else arm
    if '--draft' in sys.argv:
        label = 'd' + sys.argv[sys.argv.index('--draft') + 1]
    # `--label` names the build's part of a trial when two parts share an arm (the second loop trial's `a` and `b2`
    # are both the loop arm, on two briefs): letters and digits only, so the name still splits as model-label-n.
    if '--label' in sys.argv:
        label = sys.argv[sys.argv.index('--label') + 1]
        if not label.isalnum():
            raise SystemExit(f'--label {label!r}: letters and digits only')
    name = f'{model}-{label}-{n}'
    folder = os.path.join(out, name)
    os.makedirs(folder, exist_ok=False)
    # The brief: the club app's unless `--brief PATH` (relative to the repository) names another, as the
    # recipe trial does (docs/prompts/library-trial/recipe-protocol.md).
    brief = (sys.argv[sys.argv.index('--brief') + 1] if '--brief' in sys.argv
             else 'docs/prompts/trial-3/plain-brief.md')
    request = open(os.path.join(REPO, brief)).read()
    if '--before-amendment-1' not in sys.argv:
        request = request.rstrip('\n') + NO_OWNER
    # Amendment 3 (item 6): every arm's request starts with `sv init`'s output, the specification.
    if '--with-spec' in sys.argv:
        spec = subprocess.run([SV, 'init'], capture_output=True, text=True, timeout=60, check=True).stdout
        # Named for the file `sv init` writes; until the second loop trial (ADR-109) this said securevibe.toml,
        # and so did the club brief, and one drafts-trial build wrote both files, which `sv` refuses.
        request = ("This is StackVet's specification for the stackvet.toml my apps need:\n\n" + spec
                   + '\n\n---\n\n' + request)
    # The prompt-library trial (docs/prompts/library-trial/protocol.md): one library prompt added at the end.
    if '--prompt' in sys.argv:
        pid = sys.argv[sys.argv.index('--prompt') + 1]
        library = [q for f in ('data/prompts.json', 'data/design-prompts.json')
                   for q in json.load(open(os.path.join(REPO, f)))['prompts']]
        text = next(q['prompt'] for q in library if q['id'] == pid)
        request = request.rstrip('\n') + '\n\nFollow this while you build:\n\n' + text + '\n'
    if '--draft' in sys.argv:
        text = draft_prompt(sys.argv[sys.argv.index('--draft') + 1])
        request = request.rstrip('\n') + '\n\nFollow this while you build:\n\n' + text + '\n'
    if with_instructions:
        request = ('These are the instructions SecureVibe gives an AI coding tool; follow them:\n\n'
                   + instructions(folder) + '\n\n---\n\n' + request)
    model_id = sys.argv[sys.argv.index('--model-id') + 1] if '--model-id' in sys.argv else model
    cmd = [CLAUDE, '-p', request, '--model', model_id, '--restricted', '--strict-mcp-config',
           '--tools', ','.join(FILE_TOOLS + ['Bash']),
           '--allowedTools', ' '.join(FILE_TOOLS + [SHELL] + (['Bash(git:*)'] if '--allow-git' in sys.argv else [])
                                      + sv_tools),
           '--permission-mode', 'dontAsk', '--no-session-persistence',
           '--output-format', 'stream-json', '--verbose', '--max-budget-usd', str(budget)]
    if api:
        # The Claude program runs the helper through the shell, so a path with a space in it (the USB drive's, in the
        # drafts trial) is quoted; unquoted, every build of 12 October 2026 stopped at once with "apiKeyHelper failed".
        cmd += ['--settings', json.dumps({'apiKeyHelper': shlex.quote(KEY_HELPER)})]
    # Never install packages on this computer: `python3 -m pip` would match `Bash(python3:*)` and write into
    # the owner's own Python. Packages reach the app only through `sv`'s install step (ADR-052).
    refused = ['Bash(python3 -m pip:*)', 'Bash(python3 -m ensurepip:*)', 'Bash(pip:*)', 'Bash(pip3:*)']
    if arm.endswith('hidden'):
        refused += [f'mcp__stackvet__stackvet_{t}' for t in SV_TOOLS
                    if f'mcp__stackvet__stackvet_{t}' not in sv_tools]
    cmd += ['--disallowedTools', ' '.join(refused)]
    if attach:
        config = {'mcpServers': {'stackvet': {'command': SV, 'args': ['mcp', '--root', folder]}}}
        cfg = os.path.join(out, name + '.mcp.json')
        json.dump(config, open(cfg, 'w'))
        cmd += ['--mcp-config', cfg]
    started = time.time()
    with open(os.path.join(out, name + '.jsonl'), 'w') as transcript:
        env = {k: v for k, v in os.environ.items() if k not in ('ANTHROPIC_API_KEY', 'KEY_HELPER')}
        code = subprocess.run(cmd, cwd=folder, stdout=transcript, stderr=subprocess.STDOUT, env=env,
                              timeout=3600).returncode
    print(f'{name}: exit {code}, {time.time() - started:.0f}s', flush=True)


if __name__ == '__main__':
    out, arm, model, n = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
    budget = float(sys.argv[sys.argv.index('--budget') + 1]) if '--budget' in sys.argv else 3.0
    build(os.path.abspath(os.path.expanduser(out)), arm, model, n, budget, '--api' in sys.argv)
