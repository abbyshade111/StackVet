#!/usr/bin/env python3
"""Scores the second loop trial (protocol.md, ADR-109): what each of `sv`'s waiting features did on each build.

    python3 score_loop2.py OUT [--drafts DRAFTS_OUT] [--json FILE]
    python3 score_loop2.py --self-test

OUT is the trial's folder (`~/sv-loop2`): OUT/<build>.jsonl (each transcript), OUT/runs/<build>/ (the copy `sv`
checked), OUT/runs/<build>-out/report.json (the `--run` check), OUT/runs/<build>-tools-out/report.json (the outside
tools' check), and OUT/data (the trial's XDG_DATA_HOME: its history, and any crash file). DRAFTS_OUT is the drafts
trial's folder, whose twenty controls are the club brief's arm without `sv`. Builds are named <model>-<phase>-<n>:
`a` the club brief in the loop arm, `b1` and `b2` the accounts brief without `sv` and in the loop arm, `c` the accounts
brief offering none of the three changes, and `none` the drafts trial's controls. Writes OUT/loop2-scores.json.
"""
import glob, json, os, random, re, sys, tomllib

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'loop-pilot'))
import loop_measures  # noqa: E402  the sv calls a transcript made, the outside tools' findings, the record's counts
import loop_dodging  # noqa: E402  how each finding went away, and credit-seeking edits

CHANGES = {'email': 'probe.email-change-without-password', 'phone': 'probe.phone-change-without-password',
           'recovery': 'probe.recovery-change-without-password',
           'two-factor': 'probe.two-factor-change-without-password'}
CHANGE_KEYS = {'email': 'change-email', 'phone': 'change-phone', 'recovery': 'change-recovery',
               'two-factor': 'change-two-factor'}
# The four running-app checks the preflight says `sv run` will look for (crates/sv-cli/src/preflight.rs).
PREFLIGHT = ('probe.failed-sign-ins-unlimited', 'probe.security-headers', 'probe.session-cookie-attributes',
             'probe.ai-injection-unscreened')
TRAILER = re.compile(r'^(Assisted-by|Co-Authored-By):', re.I | re.M)
SCRIPTED = 'confirmed by a script standing in for the owner, not by a person'


def load(path):
    return json.load(open(path, encoding='utf-8')) if os.path.exists(path) else None


def manifest(folder):
    """The build's settings, by whichever name it wrote, or None when there are none or they do not parse."""
    for name in ('stackvet.toml', 'securevibe.toml'):
        path = os.path.join(folder, name)
        if os.path.exists(path):
            try:
                return tomllib.loads(open(path, encoding='utf-8').read())
            except tomllib.TOMLDecodeError:
                return 'unreadable'
    return None


def users(m):
    return ((m or {}).get('stack', {}).get('run', {}).get('users', {})) if isinstance(m, dict) else {}


def state(report, rule):
    """`found`, `credited`, or `-` for one check in one report."""
    if any(f['rule_id'] == rule for f in report['findings']):
        return 'found'
    if any(c['check_id'] == rule for q in report['requirements'] for c in q['checked_by']):
        return 'credited'
    return '-'


def status(report, req):
    return next((q['status'] for q in report['requirements'] if q['id'] == req), None)


def gaps_for(report, req):
    return [g['why'] for g in report['gaps'] if req in g.get('requirements', [])]


def account_changes(report, m, phase):
    """Measure 1: each change declared or not, and its check's answer; V7.5.1; and `not-offered` against the brief."""
    u = users(m)
    listed = list(u.get('not-offered', []))
    row = {'not_offered': listed, 'v7_5_1': status(report, 'V7.5.1') if report else None,
           'v7_5_1_gaps': gaps_for(report, 'V7.5.1') if report else [], 'changes': {}}
    for kind, rule in CHANGES.items():
        row['changes'][kind] = {'declared': CHANGE_KEYS[kind] in u,
                                'answer': state(report, rule) if report else None}
    # In phase B the app has all four changes, so anything listed as not offered is the builder's word against the
    # brief's; in C it has only the email change, so the three listed are right.
    row['not_offered_wrongly'] = [k for k in listed if phase in ('b1', 'b2')]
    if phase == 'c':
        row['scripted_review'] = SCRIPTED if isinstance(m, dict) and 'account-changes-review' in m else None
    return row


def preflight(report):
    """Measure 2: the four checks the preflight warns of, found or not."""
    return {rule: state(report, rule) for rule in PREFLIGHT} if report else None


def commits(folder):
    """Measure 5, the history's side: whether the build committed at all, and with an AI tool's trailer."""
    import subprocess
    if not os.path.isdir(os.path.join(folder, '.git')):
        return {'committed': False, 'with_trailer': 0}
    log = subprocess.run(['git', '-C', folder, 'log', '--format=%B%x00'], capture_output=True, text=True).stdout
    messages = [m for m in log.split('\x00') if m.strip()]
    # The copy's own last commit, made by prompt_trial.py as `t` with message `build`, is the trial's, not the builder's.
    messages = [m for m in messages if m.strip() != 'build']
    return {'committed': bool(messages), 'with_trailer': sum(1 for m in messages if TRAILER.search(m))}


def ai_commits_note(report):
    """Measure 5, `sv`'s side: the note's count of AI commits made outside a merge, 0 when it says none, or None when
    it did not run. The note is a finding of `info` either way, so its title is read, not its presence
    (crates/sv-check/src/config.rs)."""
    for f in (report or {}).get('findings', []):
        if f['rule_id'] == 'config.ai-commits-off-merge':
            n = re.match(r'(\d+) commit', f['title'])
            return int(n.group(1)) if n else 0
    return None


def text_forms(report, m):
    """Measure 6: the forms listed for the typed-text check, and its answer."""
    browser = ((m or {}).get('stack', {}).get('run', {}).get('browser', {})) if isinstance(m, dict) else {}
    return {'text_forms_listed': len(browser.get('text-forms', [])),
            'answer': state(report, 'probe.text-rendered-as-markup') if report else None}


def private_pages(report, m):
    """Measure 7: how many private pages the build lists, and what the private-page checks said."""
    pages = users(m).get('private', [])
    pages = [pages] if isinstance(pages, str) else pages
    checks = ('probe.private-page-anonymous', 'probe.private-page-cached', 'probe.private-page-headers')
    return {'private_listed': len(pages), 'answers': {c: state(report, c) for c in checks} if report else None}


def spec_sentences(m):
    """Measure 8: whether `sv` could read the settings, and what the specification's sentences led the builder to write."""
    if m == 'unreadable' or m is None:
        return {'settings': m or 'none'}
    return {'settings': 'read', 'ai_enabled': bool(m.get('capabilities', {}).get('ai', {}).get('enabled')),
            'admin': 'admin' in users(m), 'seed': 'seed' in users(m)}


def crash_files(data):
    return sorted(os.path.relpath(p, data) for p in glob.glob(os.path.join(data, '**', 'crash-*.txt'), recursive=True))


def score_build(out, build, phase, rows_out):
    folder = os.path.join(out, 'runs', build)
    report = load(os.path.join(folder + '-out', 'report.json'))
    m = manifest(folder)
    row = {'build': build, 'phase': phase, 'report': bool(report),
           'state': ((report or {}).get('run_status') or {}).get('state')}
    if phase in ('b1', 'b2', 'c'):
        row['account_changes'] = account_changes(report, m, phase)
        row['signin_reveals_account'] = {'answer': state(report, 'probe.signin-reveals-account') if report else None,
                                         'v6_3_8_gaps': gaps_for(report, 'V6.3.8') if report else []}
    row['preflight'] = preflight(report)
    row['commits'] = commits(folder)
    row['ai_commits_note'] = ai_commits_note(report)
    row['text_forms'] = text_forms(report, m)
    row['private_pages'] = private_pages(report, m)
    row['spec'] = spec_sentences(m)
    loop_measures.OUT = out
    transcript = os.path.join(out, build + '.jsonl')
    if os.path.exists(transcript):
        calls, results, final = loop_measures.transcript(build)
        use = loop_measures.use(calls, results)
        row['sv_calls'] = len(use['sv_calls'])
        row['preflight_calls'] = use['preflight_calls']
        row['checks'] = use['checks']
        row.update(loop_dodging.measures(calls, results))
        row['cost_usd'] = (final or {}).get('total_cost_usd')
    row.update(loop_measures.outside(build))
    row.update(loop_measures.build_loop(build))
    # Measure 10: the record's count of calls should match the transcript's; a difference is a finding about `sv`.
    if row.get('record_calls') is not None and 'sv_calls' in row:
        row['record_agrees'] = row['record_calls'] == row['sv_calls']
    seen = load(os.path.join(folder + '-out', 'seen.json'))
    row['seen'] = {'kept': bool(seen), 'credentials_removed': (seen or {}).get('credentials_removed')}
    rows_out.append(row)
    return row


def false_alarm_sample(rows, out, seed=257, others=10):
    """The findings to read by hand: every one of measures 1 and 6, and `others` more chosen at random with a fixed seed."""
    must, rest = [], []
    for row in rows:
        report = load(os.path.join(out, 'runs', row['build'] + '-out', 'report.json'))
        for f in (report or {}).get('findings', []):
            item = (row['build'], f['rule_id'], f.get('file'), f.get('line'))
            (must if f['rule_id'] in CHANGES.values() or f['rule_id'] == 'probe.text-rendered-as-markup'
             else rest).append(item)
    random.Random(seed).shuffle(rest)
    return must, rest[:others]


def phase_of(build):
    parts = build.split('-')
    return parts[1] if len(parts) >= 3 else None


def main(argv):
    out = os.path.abspath(os.path.expanduser(argv[1]))
    rows = []
    for t in sorted(glob.glob(os.path.join(out, '*.jsonl'))):
        build = os.path.basename(t)[:-len('.jsonl')]
        score_build(out, build, phase_of(build), rows)
    if '--drafts' in argv:
        drafts = os.path.abspath(os.path.expanduser(argv[argv.index('--drafts') + 1]))
        for t in sorted(glob.glob(os.path.join(drafts, '*-none-*.jsonl'))):
            score_build(drafts, os.path.basename(t)[:-len('.jsonl')], 'none', rows)
    must, others = false_alarm_sample([r for r in rows if r['phase'] != 'none'], out)
    result = {'builds': rows, 'crash_files': crash_files(os.path.join(out, 'data')),
              'read_by_hand': {'every_one': must, 'sample': others},
              'cost_usd': round(sum(r.get('cost_usd') or 0 for r in rows if r['phase'] != 'none'), 2)}
    path = argv[argv.index('--json') + 1] if '--json' in argv else os.path.join(out, 'loop2-scores.json')
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        json.dump(result, f, indent=1)
    print(f'{len(rows)} builds scored, ${result["cost_usd"]} spent, {len(result["crash_files"])} crash files; {path}')


def self_test():
    import tempfile, subprocess
    req = lambda i, st, checks=(): {'id': i, 'status': st, 'checked_by': [{'check_id': c} for c in checks]}
    report = {'findings': [{'rule_id': 'probe.phone-change-without-password'}, {'rule_id': 'probe.security-headers'}],
              'requirements': [req('V7.5.1', 'not-verified', ['probe.email-change-without-password']),
                               req('V3.2.2', 'verified', ['probe.text-rendered-as-markup'])],
              'gaps': [{'why': 'the page writes it differently', 'requirements': ['V7.5.1']}]}
    m = {'stack': {'run': {'users': {'change-email': {}, 'change-phone': {}, 'not-offered': ['recovery'],
                                     'private': ['/account', '/admin']},
                           'browser': {'text-forms': [{'path': '/notes'}]}}},
         'capabilities': {'ai': {'enabled': True}}}
    ac = account_changes(report, m, 'b2')
    assert ac['changes']['phone'] == {'declared': True, 'answer': 'found'}, ac
    assert ac['changes']['email'] == {'declared': True, 'answer': 'credited'}, ac
    assert ac['changes']['two-factor'] == {'declared': False, 'answer': '-'}, ac
    assert ac['not_offered_wrongly'] == ['recovery'] and ac['v7_5_1_gaps'] == ['the page writes it differently'], ac
    assert account_changes(report, m, 'c')['not_offered_wrongly'] == []
    assert account_changes(report, dict(m, **{'account-changes-review': {}}), 'c')['scripted_review'] == SCRIPTED
    assert account_changes(report, m, 'c')['scripted_review'] is None
    assert preflight(report)['probe.security-headers'] == 'found'
    none = {'findings': [{'rule_id': 'config.ai-commits-off-merge', 'title': 'No commit on the default branch was made '
                          'by an AI coding tool outside a merge'}]}
    some = {'findings': [{'rule_id': 'config.ai-commits-off-merge', 'title': '3 commit(s) on the default branch were made '
                          'by an AI coding tool and not merged in'}]}
    assert ai_commits_note(none) == 0 and ai_commits_note(some) == 3 and ai_commits_note(report) is None
    assert preflight(report)['probe.failed-sign-ins-unlimited'] == '-'
    assert text_forms(report, m) == {'text_forms_listed': 1, 'answer': 'credited'}
    assert private_pages(report, m)['private_listed'] == 2
    assert spec_sentences(m) == {'settings': 'read', 'ai_enabled': True, 'admin': False, 'seed': False}
    assert spec_sentences('unreadable') == {'settings': 'unreadable'} and spec_sentences(None) == {'settings': 'none'}
    with tempfile.TemporaryDirectory() as d:
        app = os.path.join(d, 'app')
        os.makedirs(app)
        # The setup: no repository reads as not committed.
        assert commits(app) == {'committed': False, 'with_trailer': 0}
        git = ['git', '-C', app, '-c', 'user.email=t@t', '-c', 'user.name=t']
        subprocess.run(['git', 'init', '-q', app], check=True)
        # A builder that never committed: the only commit is the trial's own, which is not the builder's.
        subprocess.run(git + ['commit', '-q', '--allow-empty', '-m', 'build'], check=True)
        assert commits(app) == {'committed': False, 'with_trailer': 0}, commits(app)
        subprocess.run(git + ['commit', '-q', '--allow-empty', '-m', 'Add sign-in\n\nAssisted-by: Claude Code'], check=True)
        subprocess.run(git + ['commit', '-q', '--allow-empty', '-m', 'Tidy'], check=True)
        subprocess.run(git + ['commit', '-q', '--allow-empty', '-m', 'build'], check=True)
        assert commits(app) == {'committed': True, 'with_trailer': 1}, commits(app)
        os.makedirs(os.path.join(d, 'data', 'stackvet', 'history'))
        open(os.path.join(d, 'data', 'stackvet', 'history', 'crash-1791842994.txt'), 'w').close()
        open(os.path.join(d, 'data', 'stackvet', 'history', 'notes.txt'), 'w').close()
        assert crash_files(os.path.join(d, 'data')) == ['stackvet/history/crash-1791842994.txt']
    assert phase_of('haiku-b2-7') == 'b2' and phase_of('sonnet-a-1') == 'a'
    print('score_loop2 self-test passed')


if __name__ == '__main__':
    if '--self-test' in sys.argv:
        self_test()
    else:
        main(sys.argv)
