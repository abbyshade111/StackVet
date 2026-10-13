#!/usr/bin/env python3
"""The loop trials' measures (docs/prompts/loop-protocol.md), from each build's transcript and report.

    python3 loop_measures.py OUT BUILD... [--json FILE]
    python3 loop_measures.py --self-test

Reads OUT/<build>.jsonl (the stream-json transcript) and OUT/runs/<build>-out/report.json (tools/prompt_trial.py's
`run`). Prints one line per build and writes OUT/loop-measures.json. How each finding went away, and the edits that
seek credit, are `loop_dodging.py`'s (gap analysis of 7 October 2026, finding 21). Since the second loop trial
(ADR-109): the outside tools' findings, from the second check `prompt_trial.py` makes with `--tools` when asked
(OUT/runs/<build>-tools-out/report.json), which the builder never sees; and what the report's build-loop record says.
"""
import json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import loop_dodging  # noqa: E402  how each finding went away, and credit-seeking edits (gap analysis 21)

OUT = os.getcwd()
APP_FILES = ('app.py', 'seed.py')
# The outside tools `sv report --tools` runs, by the first part of their findings' rule ids (data/adapters.json).
OUTSIDE = ('bandit', 'semgrep', 'codeql-python', 'codeql-javascript', 'codeql-ruby', 'gosec', 'brakeman')


def sv_tool(name):
    """The `sv` tool a call made (`check`, `spec`, ...), under either of the server's names, or None.

    The server was attached as `securevibe` with `securevibe_*` tools until the second loop trial (ADR-109), and as
    `stackvet` with `stackvet_*` since; reading only the old names counted every check of a new build as none."""
    for server in ('mcp__stackvet__', 'mcp__securevibe__'):
        if name.startswith(server):
            tool = name[len(server):]
            for prefix in ('stackvet_', 'securevibe_'):
                if tool.startswith(prefix):
                    return tool[len(prefix):]
            return tool
    return None


def transcript(build):
    """Every tool call in order (name, input), the result message, and each sv check's text result."""
    calls, results, final = [], {}, None
    for line in open(os.path.join(OUT, build + '.jsonl'), encoding='utf-8', errors='replace'):
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        kind = event.get('type')
        if kind == 'assistant':
            for block in event.get('message', {}).get('content', []):
                if block.get('type') == 'tool_use':
                    calls.append((block['id'], block['name'], block.get('input', {})))
        elif kind == 'user':
            for block in event.get('message', {}).get('content', []):
                if isinstance(block, dict) and block.get('type') == 'tool_result':
                    content = block.get('content')
                    text = content if isinstance(content, str) else ' '.join(
                        c.get('text', '') for c in content or [] if isinstance(c, dict))
                    results[block['tool_use_id']] = text
        elif kind == 'result':
            final = event
    return calls, results, final


def use(calls, results):
    sv = [(i, sv_tool(name)) for i, (_, name, _) in enumerate(calls) if sv_tool(name)]
    writes = [i for i, (_, name, args) in enumerate(calls)
              if name in ('Write', 'Edit') and os.path.basename(args.get('file_path', '')) in APP_FILES]
    first_write = writes[0] if writes else None
    before = {n for i, n in sv if first_write is None or i < first_write}
    checks = [(cid, name) for cid, name, _ in calls if sv_tool(name) == 'check']
    attention = []
    for cid, _ in checks:
        # The check answers in JSON: how many requirements need attention, and the findings.
        try:
            answer = json.loads(results.get(cid, ''))
            attention.append({'needs_attention': answer.get('needsAttention'),
                              'findings': [f"{f.get('rule_id')}:{f.get('severity')}" for f in answer.get('findings', [])]})
        except (json.JSONDecodeError, AttributeError):
            attention.append(None)
    # A round is a check followed by a change to the app's code; "checked again" is a check after such a change.
    order = [('check' if sv_tool(name) == 'check' else
              'edit' if name in ('Write', 'Edit') and not os.path.basename(args.get('file_path', '')).endswith(('.md', '.toml'))
              else None) for _, name, args in calls]
    order = [o for o in order if o]
    rechecked = any(order[i] == 'check' and 'edit' in order[i + 1:j] for i in range(len(order))
                    for j in range(i + 2, len(order) + 1) if j < len(order) + 1 and order[j - 1] == 'check' and j - 1 > i)
    return {
        'preflight_calls': sum(1 for _, n in sv if n == 'preflight'),
        'checked_again_after_a_fix': rechecked,
        'sv_calls': [n for _, n in sv],
        'spec_before_code': 'spec' in before,
        'plan_before_code': 'plan' in before,
        'checks': len(checks),
        'attention_per_check': attention,
    }


def report(build):
    path = os.path.join(OUT, 'runs', build + '-out', 'report.json')
    if not os.path.exists(path):
        return {'started': False, 'signed_in': False, 'answered': 0, 'running_findings': 0, 'own_high': 0}
    r = json.load(open(path, encoding='utf-8'))
    started = (r.get('run_status') or {}).get('state') == 'started'
    signed_in = started and not any(
        g['why'].startswith('Signing in as the first test user did not open') for g in r['gaps'])
    credited = {c['check_id'] for q in r['requirements'] for c in q['checked_by'] if c['check_id'].startswith('probe.')}
    found = {f['rule_id'] for f in r['findings'] if f['rule_id'].startswith('probe.')}
    own_high = sum(1 for f in r['findings'] if not f['rule_id'].startswith('probe.')
                   and f.get('severity') in ('critical', 'high'))
    return {'started': started, 'signed_in': signed_in, 'answered': len(credited | found),
            'running_findings': len([f for f in r['findings'] if f['rule_id'].startswith('probe.')]),
            'own_high': own_high}



def outside(build):
    """The outside tools' findings in the second check, made with `--tools` and never shown to the builder (ADR-109).

    None when there was no second check, so a trial without one reads as not measured, never as no findings."""
    path = os.path.join(OUT, 'runs', build + '-tools-out', 'report.json')
    if not os.path.exists(path):
        return {'outside_checked': False, 'outside_findings': None, 'outside_by_tool': None}
    r = json.load(open(path, encoding='utf-8'))
    by_tool = {}
    for f in r['findings']:
        tool = f['rule_id'].split('.')[0]
        if tool in OUTSIDE:
            by_tool[tool] = by_tool.get(tool, 0) + 1
    return {'outside_checked': True, 'outside_findings': sum(by_tool.values()), 'outside_by_tool': by_tool}


def build_loop(build):
    """What the report's build-loop record says (ADR-076, ADR-084): the calls and checks the MCP server recorded."""
    path = os.path.join(OUT, 'runs', build + '-out', 'report.json')
    if not os.path.exists(path):
        return {'record_calls': None, 'record_checks': None, 'record_unreadable': None}
    loop = json.load(open(path, encoding='utf-8')).get('build_loop') or {}
    return {'record_calls': loop.get('calls'), 'record_checks': loop.get('checks'),
            'record_unreadable': loop.get('unreadable')}


def self_test():
    """Each measure added for the second loop trial, against written-out builds, so a broken one fails here."""
    import tempfile
    global OUT
    assert sv_tool('mcp__stackvet__stackvet_check') == 'check'
    assert sv_tool('mcp__securevibe__securevibe_check') == 'check'
    assert sv_tool('mcp__securevibe__stackvet_preflight') == 'preflight'
    assert sv_tool('Write') is None and sv_tool('mcp__other__stackvet_check') is None
    calls = [('a', 'mcp__stackvet__stackvet_spec', {}), ('b', 'Write', {'file_path': '/x/app.py'}),
             ('c', 'mcp__stackvet__stackvet_check', {}), ('d', 'Edit', {'file_path': '/x/app.py'}),
             ('e', 'mcp__stackvet__stackvet_check', {}), ('f', 'mcp__stackvet__stackvet_preflight', {})]
    u = use(calls, {})
    assert u['checks'] == 2 and u['spec_before_code'] and u['preflight_calls'] == 1, u
    assert u['checked_again_after_a_fix'], u
    with tempfile.TemporaryDirectory() as d:
        OUT = d
        # The setup: no second check reads as not measured.
        assert outside('b1') == {'outside_checked': False, 'outside_findings': None, 'outside_by_tool': None}
        os.makedirs(os.path.join(d, 'runs', 'b1-tools-out'))
        os.makedirs(os.path.join(d, 'runs', 'b1-out'))
        findings = [{'rule_id': r} for r in ('bandit.B104', 'bandit.B608', 'semgrep.python.x', 'ast.sql', 'probe.y')]
        with open(os.path.join(d, 'runs', 'b1-tools-out', 'report.json'), 'w', encoding='utf-8') as f:
            json.dump({'findings': findings}, f)
        with open(os.path.join(d, 'runs', 'b1-out', 'report.json'), 'w', encoding='utf-8') as f:
            json.dump({'build_loop': {'calls': 7, 'checks': 3, 'unreadable': 0}}, f)
        o = outside('b1')
        assert o == {'outside_checked': True, 'outside_findings': 3, 'outside_by_tool': {'bandit': 2, 'semgrep': 1}}, o
        assert build_loop('b1') == {'record_calls': 7, 'record_checks': 3, 'record_unreadable': 0}
    print('loop_measures self-test passed')


if __name__ == '__main__':
    if '--self-test' in sys.argv:
        self_test()
        sys.exit(0)
    OUT = os.path.abspath(os.path.expanduser(sys.argv[1]))
    rows = []
    for build in [a for i, a in enumerate(sys.argv[2:], 2) if a != '--json' and sys.argv[i - 1] != '--json']:
        calls, results, final = transcript(build)
        row = {'build': build, **use(calls, results), **report(build), **loop_dodging.measures(calls, results),
               **outside(build), **build_loop(build),
               'cost_usd': (final or {}).get('total_cost_usd'), 'seconds': ((final or {}).get('duration_ms') or 0) / 1000,
               'turns': (final or {}).get('num_turns'), 'stopped': (final or {}).get('subtype')}
        rows.append(row)
        print(f"{build:22} sv={','.join(row['sv_calls']) or '-'} spec-first={row['spec_before_code']} "
              f"plan-first={row['plan_before_code']} checks={row['checks']} attention={row['attention_per_check']} "
              f"started={row['started']} signed-in={row['signed_in']} answered={row['answered']} "
              f"went-away={row['went_away']} credit-seeking={row['credit_seeking']} "
              f"outside={row['outside_by_tool']} record-calls={row['record_calls']} "
              f"cost=${row['cost_usd']} {row['seconds']:.0f}s {row['stopped']}")
    path = sys.argv[sys.argv.index('--json') + 1] if '--json' in sys.argv else os.path.join(OUT, 'loop-measures.json')
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(rows, f, indent=1)
