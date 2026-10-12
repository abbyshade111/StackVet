#!/usr/bin/env python3
"""Runs `sv report --run` on builds of the design-time prompts' test app and prints what each prompt's checks said.

    python3 tools/prompt_trial.py TRIAL_DIR BUILD[:slow] ...

TRIAL_DIR holds one folder per build (an app.py, seed.py, and stackvet.toml a helper agent wrote from
docs/prompts/trial/brief.md, with or without one prompt from data/design-prompts.json) and a policy.toml
(docs/prompts/trial/policy.toml): the numbers standing in for the owner's, added to a build's [policy] only where
it wrote none of its own. Each build is copied to TRIAL_DIR/runs/, made a git repository, and run behind the
fence, so it needs Docker or Colima; `:slow` adds `--slow` for the session timeouts. Used for docs/prompts/design-time.md.

Two more steps for the second loop trial (ADR-109), each only when its variable is set, so an earlier trial's run is
unchanged:
- `PT_REVIEW` names a script that stands in for the owner in `sv review` (docs/prompts/loop-2/scripted_review.py). It
  runs on the copy before it is committed, and only when the build's settings list `not-offered`; every result that
  rests on it is labeled as confirmed by a script, not by a person.
- `PT_TOOLS=1` checks each build a second time, with `--tools --keep-tool-output` and without `--run`, into
  TRIAL_DIR/runs/<build>-tools-out: the outside tools' view, which the builder never saw.
"""
import json, os, shutil, subprocess, sys
HERE = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.getcwd()
SV = os.environ.get('SV', 'sv')
TARGETS = {
    'p1': ['probe.private-page-anonymous', 'probe.admin-page-ordinary-user',
           'probe.admin-action-ordinary-user', 'probe.other-users-data'],
    'p2': ['probe.action-done-twice'],
    'p3': ['probe.create-rate-unlimited', 'probe.failed-sign-ins-unlimited'],
    'p4': ['probe.error-detail-leak', 'probe.ai-service-failure-handled', 'probe.ai-service-error-shown'],
    'p6': ['probe.log-line-metadata', 'probe.log-timestamp-zoned', 'probe.authorization-failure-logged'],
    'p7': ['probe.session-idle-timeout', 'probe.session-lifetime', 'probe.logout-keeps-session',
           'probe.app-token-signature-not-checked', 'probe.app-token-alg-none', 'probe.app-token-expired-accepted'],
}
REQS = {'p1': ['V8.2.1', 'V8.2.2', 'V8.3.1'], 'p2': ['V2.3.4'], 'p3': ['V2.4.1', 'V6.3.1'],
        'p4': ['V16.5.1', 'V16.5.2'], 'p6': ['V16.2.1', 'V16.2.2', 'V16.3.2'],
        'p7': ['V7.3.1', 'V7.3.2', 'V7.4.1', 'V9.1.1', 'V9.1.2', 'V9.2.1']}

def merge_policy(path):
    """Adds the tester's [policy] numbers the build did not set itself, and says which were whose."""
    import tomllib
    tester = tomllib.load(open(os.path.join(HERE, 'policy.toml'), 'rb')).get('policy', {}) \
        if os.path.exists(os.path.join(HERE, 'policy.toml')) else {}
    # A build with no stackvet.toml, or one that does not parse, is checked as it is: what `sv`
    # makes of it is the outcome (loop-protocol.md, "What makes a build unusable").
    if not os.path.exists(path):
        print('  no stackvet.toml: checked as it is', flush=True)
        return
    text = open(path, encoding="utf-8").read()
    try:
        own = tomllib.loads(text).get('policy', {})
    except tomllib.TOMLDecodeError as e:
        print(f'  stackvet.toml does not parse ({e}): checked as it is', flush=True)
        return
    missing = {k: v for k, v in tester.items() if k not in own}
    print(f'  policy the build set itself: {own}; added by the tester: {missing}', flush=True)
    if not missing:
        return
    lines = '\n'.join(f'{k} = {v}' for k, v in missing.items())
    if 'policy' in tomllib.loads(text):
        text = text.replace('[policy]\n', '[policy]\n' + lines + '\n', 1)
    else:
        text = text.rstrip() + '\n\n[policy]\n' + lines + '\n'
    tomllib.loads(text)
    open(path, 'w', encoding='utf-8', newline='\n').write(text)

def run(build, slow):
    src = os.path.join(HERE, build)
    dst = os.path.join(HERE, 'runs', build)
    out = dst + '-out'
    shutil.rmtree(dst, ignore_errors=True); shutil.rmtree(out, ignore_errors=True)
    shutil.copytree(src, dst, ignore=shutil.ignore_patterns('*.db', '__pycache__', 'OWNER-REQUEST.md'))
    # The settings by whichever name the build wrote: the club brief still says securevibe.toml, which `sv` reads
    # until 1 January 2027 (ADR-062); a policy added only to stackvet.toml would miss a build that followed it.
    names = [n for n in ('stackvet.toml', 'securevibe.toml') if os.path.exists(os.path.join(dst, n))]
    merge_policy(os.path.join(dst, names[0] if names else 'stackvet.toml'))
    # The scripted review (ADR-109) writes its confirmation into the settings, so it comes before the commit.
    if os.environ.get('PT_REVIEW') and names and 'not-offered' in open(os.path.join(dst, names[0]), encoding='utf-8').read():
        with open(dst + '.review.log', 'w', encoding='utf-8', newline='\n') as log:
            subprocess.run([sys.executable, os.environ['PT_REVIEW'], SV, dst], stdout=log, stderr=subprocess.STDOUT,
                           timeout=600)
    # --allow-empty: a build that wrote nothing is checked as it is (it has no report), not a crash of the run.
    subprocess.run('git init -q && git add -A && git -c user.email=t@t -c user.name=t commit -qm build --allow-empty',
                   shell=True, cwd=dst, check=True)
    cmd = [SV, 'report', dst, '--run', '--out', out] + (['--slow'] if slow else [])
    with open(dst + '.log', 'w', encoding='utf-8', newline='\n') as log:
        code = subprocess.run(cmd, stdout=log, stderr=subprocess.STDOUT, timeout=3600).returncode
    if os.environ.get('PT_TOOLS') == '1':
        tools_out = dst + '-tools-out'
        shutil.rmtree(tools_out, ignore_errors=True)
        with open(dst + '.tools.log', 'w', encoding='utf-8', newline='\n') as log:
            subprocess.run([SV, 'report', dst, '--tools', '--keep-tool-output', '--out', tools_out], stdout=log,
                           stderr=subprocess.STDOUT, timeout=3600)
    path = os.path.join(out, 'report.json')
    return code, (json.load(open(path, encoding="utf-8")) if os.path.exists(path) else None)

def summary(report):
    lines = []
    for p, rules in TARGETS.items():
        lines.append(f'  {p}:')
        for rule in rules:
            found = [f['title'] for f in report['findings'] if f['rule_id'] == rule]
            found += [o['rule_id'] + ' (above level)' for o in report.get('out_of_scope', []) if o['rule_id'] == rule]
            credited = [c['scope'] for r in report['requirements'] for c in r['checked_by'] if c['check_id'] == rule]
            state = 'FOUND' if found else ('credited' if credited else '-')
            lines.append(f'    {rule}: {state}')
        for req in REQS[p]:
            # Each gap names its requirements in `requirements` (backlog 226, part 2, item 20); a report from
            # before then has none, and its text is read as it was.
            gaps = [g['why'][:160] for g in report['gaps'] if req in g.get('requirements', []) or ('requirements' not in g and (req in g['what'].split(',')[0].split()[0:1] or g['what'].startswith(req)))]
            for g in gaps:
                lines.append(f'      gap {req}: {g}')
    return '\n'.join(lines)

if __name__ == '__main__':
    import time
    builds = sys.argv[2:]
    for i, arg in enumerate(builds, 1):
        build, _, flag = arg.partition(':')
        began = time.time()
        code, report = run(build, flag == 'slow')
        # One line a build on stderr, so a run whose summaries go elsewhere still shows how far it has got.
        state = (report.get('run_status') or {}).get('state', 'not run') if report else 'no report'
        print(f'[{i}/{len(builds)}] {build}: {state}, {time.time() - began:.0f}s', file=sys.stderr, flush=True)
        if report is None:
            said = open(os.path.join(HERE, 'runs', build + '.log'), encoding="utf-8").read().strip().splitlines()[-3:]
            text = f'== {build} (exit {code}, no report: ' + ' / '.join(said) + ')'
        else:
            text = f'== {build} (exit {code}, run {report.get("run_status")})\n' + summary(report)
        print(text, flush=True)
        open(os.path.join(HERE, 'runs', build + '.summary'), 'w', encoding='utf-8', newline='\n').write(text + '\n')
