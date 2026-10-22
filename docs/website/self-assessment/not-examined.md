# What was not examined: sv (StackVet)

Produced by `sv` 0.1.0 (commit 082ca7feb89f) on 2026-10-22 20:42:01 UTC, run 44f4f1614a4d.

**This file:** What this report could not look at, and why, with the outside tools' runs, from compliance.md. **For:** You, before you read anything as safe. The end of `report.html` lists every file in this folder and who each is for.

This page is one part of the report, cut whole from it: it credits nothing on its own, and the counts stay in `compliance.md`. Like every file in this folder it is sealed with the report, so `sv` can tell whether it is the page it wrote.

## What was not examined

Each of these is a limit on what the rest of this report can mean.

| not examined | why |
|---|---|
| the running app | `sv report` does not start the app unless you pass --run. Without it, nothing here has asked the app anything — what it sends to a browser, what it says when something goes wrong, which sites it accepts. |
| 1 folder left out as installed or built code | a folder named like an ecosystem's output, beside the file that makes it so, or a Python virtual environment under any name, holds code the app installed or built rather than wrote, and no check read it, which is normal: `target/` (output beside its Cargo.toml). If one holds the app's own code, move it or rename the folder. |
| what the folders named as not the app use | stackvet.toml says these folders are not the app (`[repository] not-the-app`): `crates/sv-check/tests`, `crates/sv-cli/tests`, `crates/sv-frameworks/tests`, `crates/sv-manifest/tests`, `crates/sv-report/tests`, `crates/sv-run/tests`, `crates/sv-scan/tests`, `docs`, `examples`, `tools`, holding 481 of the app's 1179 code files. Their code is still checked, and its findings count, listed with test and sample code. What is in them cannot change which requirements apply: a library an example uses is not one the app uses. If the app's own code is in one of them, take it off the list. |
| whether the app itself has `xml` | The only sign of it is docs/website/diagrams/source/make_diagrams.py, in a folder stackvet.toml says is not the app (`[repository] not-the-app`), so it is not counted as a "no": nothing else answers it, so its requirements wait on that question rather than being set aside. |
| whether the app itself has `mtls` | The only sign of it is crates/sv-scan/tests/fixtures/flask-behind-nginx/nginx.conf, in a folder stackvet.toml says is not the app (`[repository] not-the-app`), so it is not counted as a "no": nothing else answers it, so its requirements wait on that question rather than being set aside. |
| whether the app itself has `format-strings` | The only sign of it is code in python, in a folder stackvet.toml says is not the app (`[repository] not-the-app`), so it is not counted as a "no": nothing else answers it, so its requirements wait on that question rather than being set aside. |
| whether the app itself has `jwt` | The only sign of it is pyjwt, in examples/flask-booking/requirements.txt, in a folder stackvet.toml says is not the app (`[repository] not-the-app`), so it is not counted as a "no": stackvet.toml says it does not, and that answer stands; if the app itself does, change the answer. |
| whether the app itself has `self-hosted-model` | The only sign of it is crates/sv-check/tests/fixtures/live/https-none.bin, in a folder stackvet.toml says is not the app (`[repository] not-the-app`), so it is not counted as a "no": stackvet.toml says it does not, and that answer stands; if the app itself does, change the answer. |
| the check `config.rich-text-without-sanitizer` | No rich-text editor among the packages `sv` read, but it could not read Dart (pub) (`crates/sv-scan/tests/fixtures/dart-server/pubspec.yaml` declares Dart (pub) dependencies, and `sv` does not read that file, so none of its packages is listed or compared with known vulnerabilities), where one may be. |
| 6 files not read while looking for credentials | `crates/sv-check/tests/fixtures/live/https-none.bin` (not a text file), `crates/sv-check/tests/fixtures/live/https-with-ech.bin` (not a text file), `crates/sv-check/tests/fixtures/live/https-without-ech.bin` (not a text file), `exports/StackVet - AISVS 1.0 requirements.xlsx` (not a text file), `exports/StackVet - ASVS 5.0 requirements.xlsx` (not a text file), and 1 more, listed by `sv check`. A credential in a file nothing read is a credential nothing found. |
| code written in erb, html | `sv` has no parser for these, so the rules that read code did not run on them; the templates are `crates/sv-check/tests/fixtures/brakeman/app/app/views/users/show.html.erb` |
| 50 requirements that are design review, not scanning | these ask how the system was designed and how it is run — whether trust zones are enforced, whether an incident response plan is rehearsed, whether data has named owners. No check here reaches them and none ever will, so they are counted as applicable and unverified, and a person has to answer them. |
| everything Go installs | `crates/sv-scan/tests/fixtures/go-stdlib-xml/go.sum` was read and no packages could be taken from it, so nothing from Go is listed. This is not an approximate list of this app's Go dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| everything npm installs | npm is in use but nothing readable says which versions are installed, so none of its packages are listed. This is not an approximate list of this app's npm dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| everything npm installs | `crates/sv-scan/tests/fixtures/node-websockets/package-lock.json` was read and no packages could be taken from it, so nothing from npm is listed. This is not an approximate list of this app's npm dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| everything npm installs | `crates/sv-scan/tests/fixtures/pinned-clean/package-lock.json` was read and no packages could be taken from it, so nothing from npm is listed. This is not an approximate list of this app's npm dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| everything npm installs | npm is in use but nothing readable says which versions are installed, so none of its packages are listed. This is not an approximate list of this app's npm dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| part of what Python installs | `crates/sv-scan/tests/fixtures/unpinned-python/requirements.txt` has no lockfile beside it, and 2 of what it installs is asked for as a range, as any version, from an address or a folder, or from another file (flask, gunicorn), so it is not listed; a lockfile, such as one from `pip-compile --generate-hashes` or `uv pip compile`, is what `sv` reads. The list of this app's Python dependencies leaves these out, so nothing here can say whether a package with a known vulnerability is among them |
| everything npm installs | `crates/sv-scan/tests/fixtures/unreadable-language/package-lock.json` was read and no packages could be taken from it, so nothing from npm is listed. This is not an approximate list of this app's npm dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| everything Ruby installs | Ruby is in use but nothing readable says which versions are installed, so none of its packages are listed. This is not an approximate list of this app's Ruby dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| everything npm installs | npm is in use but nothing readable says which versions are installed, so none of its packages are listed. This is not an approximate list of this app's npm dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| everything Go installs | Go is in use but nothing readable says which versions are installed, so none of its packages are listed. This is not an approximate list of this app's Go dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| everything Dart (pub) installs | `crates/sv-scan/tests/fixtures/dart-server/pubspec.yaml` declares Dart (pub) dependencies, and `sv` does not read that file, so none of its packages is listed or compared with known vulnerabilities. This is not an approximate list of this app's Dart (pub) dependencies, it is an empty one: nothing here can say whether a package with a known vulnerability is among them |
| which Python versions are really installed | the 3 Python packages listed here were read from a manifest rather than a lockfile, so they are what was asked for rather than what an install resolved to |
| known vulnerabilities in the packages this app ships | `sv report` compares against known vulnerabilities only when you pass --advisories DIR. `sv` does not fetch anything, because the list of packages an app depends on is yours: download an OSV export for this app's ecosystems, unpack it, and pass its folder with --advisories. Python: https://osv-vulnerabilities.storage.googleapis.com/PyPI/all.zip. Rust: https://osv-vulnerabilities.storage.googleapis.com/crates.io/all.zip. |
| 28 requirements that ask for a written decision | No tool can answer these: they ask what your rules are, who may do what, and how long things are kept. Run `sv notes` to write security-notes.md, answer the questions in it, and they become documented. Your AI coding tool can ask you them: `sv questions` prints them for its chat. |
| 48 questions about how this app is built | No tool can settle these — whether input is validated on the server, whether the app's own services authenticate to each other. Answer them in the `[design]` section of stackvet.toml: V2.2.2, V7.2.1, V8.3.1, V15.3.1, V1.1.1, V4.1.3, V4.2.1, V8.2.3, V10.1.1, V11.2.2, V13.2.1, V13.2.2, V14.2.2, V15.3.4, V16.2.2, V16.4.3, V2.3.3, V6.8.4, V6.8.1, V10.3.4, V1.5.3, V2.3.5, V3.7.3, V4.1.5, V4.2.5, V5.2.4, V6.4.5, V6.4.6, V6.5.6, V6.5.7, V8.2.4, V8.3.3, V10.3.5, V11.1.3, V11.7.1, V11.7.2, V13.3.3, V13.3.4, V14.2.6, V14.2.7, V15.2.5, V15.4.4, V10.2.3, V11.3.5, V15.4.1, V4.2.3, V4.2.4, V16.5.4. Your AI coding tool can ask you them: `sv questions` prints them for its chat. |

## Outside tools

The security tools each language already has, run beside StackVet's own checks with every `sv report` at a terminal unless `--no-tools` is given. Their findings are among the things to fix above, each marked with the tool's rule; one that found the same as StackVet on the same line is counted here as agreeing.

| tool | what happened | version | found | agreed with StackVet | took |
|---|---|---|---:|---:|---:|
| Bandit | Nothing for it to read: this app has no code in python |  | 0 | 0 |  |
| gosec | Nothing for it to read: this app has no code in go |  | 0 | 0 |  |
| Brakeman | Nothing for it to read: this app has no code in ruby |  | 0 | 0 |  |
| Semgrep | Ran. | 1.176.0 | 73 | 1 | 19.7 s |

## Comments that tell a scanner to look away

Each of these comments in the app's code, or entries in a scanner's own ignore file, tells a security scanner to skip a line or a warning. An AI coding tool asked to make a finding go away may add one rather than fix the problem. Where a scanner StackVet runs can be told to look anyway, it is, so a finding a comment tried to hide is still among the things to fix when the scanner found it. Nothing here counts toward any requirement or credits anything. If you did not mean something to be hidden, take the comment out.

| file | line | scanner | what it says |
|---|---:|---|---|
| `.github/workflows/codeql.yml` | 16 | CodeQL | ``codeql[js/...]` and `// lgtm[js/...]` on the`` |
| `crates/sv-check/src/adapters.rs` | 1790 | Bandit or gosec | ``nosec` makes bandit report nothing about it, and the SARIF it writes then holds`` |
| `crates/sv-check/src/adapters.rs` | 1806 | Bandit or gosec | ``nosec`, which it skipped entirely",`` |
| `crates/sv-check/src/adapters.rs` | 1834 | Bandit or gosec | ``nosec` and a \`` |
| `crates/sv-check/src/adapters.rs` | 2476 | Semgrep | ``nosemgrep`, gosec for `#nosec` when asked to track them, and Brakeman for a warning listed in`` |
| `crates/sv-check/src/git_changes_tests.rs` | 52 | Bandit or gosec | `nosec\n").unwrap();` |
| `crates/sv-check/src/git_changes_tests.rs` | 94 | Bandit or gosec | `nosec".to_owned()))` |
| `crates/sv-check/src/look_away.rs` | 2 | Bandit or gosec | ``nosec` to the line rather than fix it: found in the`` |
| `crates/sv-check/src/look_away.rs` | 8 | Bandit or gosec | ``nosec`, gosec's `gosec:disable`, Semgrep's `nosemgrep`, CodeQL's `lgtm` and`` |
| `crates/sv-check/src/look_away.rs` | 9 | CodeQL | ``codeql[...]`, each in a comment, and the warnings listed in Brakeman's `config/brakeman.ignore`. A comment is`` |
| `crates/sv-check/src/look_away_tests.rs` | 34 | Bandit or gosec | `nosec B104\nsubprocess.call(cmd, shell=True)  # nosemgrep: python.lang.security.audit.subprocess-shell-true\n",` |
| `crates/sv-check/src/look_away_tests.rs` | 38 | Bandit or gosec | `nosec G401 -- checksum only\n\t//gosec:disable G101\n}\n",` |
| `crates/sv-check/src/look_away_tests.rs` | 42 | CodeQL | `lgtm[js/code-injection]\n/* codeql[js/xss] the page escapes it` |
| `crates/sv-check/src/look_away_tests.rs` | 46 | Semgrep | `nosemgrep` |
| `crates/sv-check/src/look_away_tests.rs` | 96 | Bandit or gosec | `nosec to app.py.\n"),` |
| `crates/sv-check/src/look_away_tests.rs` | 106 | CodeQL | `lgtm\n",` |
| `crates/sv-check/src/look_away_tests.rs` | 158 | Bandit or gosec | `nosec B105 the old key was {key}\n"),` |
| `crates/sv-check/src/review_still_reads_tests.rs` | 58 | Bandit or gosec | `nosec B104\nrun(host)\n";` |
| `crates/sv-check/src/review_still_reads_tests.rs` | 69 | Bandit or gosec | `nosec\nhost = \"0.0.0.0\"\nrun(host)\n";` |
| `crates/sv-check/tests/fixtures/suppressed/bandit-app/blanket.py` | 2 | Bandit | `nosec` |
| `crates/sv-check/tests/fixtures/suppressed/bandit-app/targeted.py` | 2 | Bandit | `nosec B608` |
| `crates/sv-check/tests/fixtures/suppressed/brakeman-app/config/brakeman.ignore` | 0 | Brakeman | `a warning, listed in crates/sv-check/tests/fixtures/suppressed/brakeman-app/config/brakeman.ignore: Checked by hand.` |
| `crates/sv-check/tests/fixtures/suppressed/codeql-app-js/app.js` | 11 | CodeQL | `lgtm[js/command-line-injection]` |
| `crates/sv-check/tests/fixtures/suppressed/codeql-app-js/app.js` | 16 | CodeQL | `codeql[js/command-line-injection]` |
| `crates/sv-check/tests/fixtures/suppressed/codeql-app-js/app.js` | 22 | CodeQL | `codeql[js/command-line-injection]` |
| `crates/sv-check/tests/fixtures/suppressed/codeql-app-py/app.py` | 15 | CodeQL | `lgtm[py/command-line-injection]` |
| `crates/sv-check/tests/fixtures/suppressed/codeql-app-py/app.py` | 21 | CodeQL | `codeql[py/command-line-injection]` |
| `crates/sv-check/tests/fixtures/suppressed/codeql-app-py/app.py` | 28 | CodeQL | `codeql[py/command-line-injection]` |
| `crates/sv-check/tests/fixtures/suppressed/gosec-app/main.go` | 9 | gosec | `nosec G202` |
| `crates/sv-check/tests/fixtures/suppressed/gosec-app/main.go` | 13 | gosec | `nosec` |
| `crates/sv-check/tests/fixtures/suppressed/semgrep-app/app.py` | 3 | Semgrep | `nosemgrep` |
| `crates/sv-check/tests/fixtures/suppressed/semgrep-app/app.py` | 5 | Semgrep | `nosemgrep: shell-true` |
| `crates/sv-check/tests/suppressed.rs` | 3 | Bandit or gosec | ``nosec` on a line makes bandit report nothing about it. Before this, `sv` saw the empty list of`` |
| `crates/sv-check/tests/suppressed.rs` | 84 | Bandit or gosec | `nosec line as a result,` |
| `crates/sv-check/tests/suppressed.rs` | 120 | Bandit or gosec | ``nosec`"), "{why}");`` |
| `crates/sv-check/tests/suppressed.rs` | 126 | Bandit or gosec | ``nosec B608` is counted apart from a bare `# nosec`, and hides just as much.`` |
| `crates/sv-check/tests/suppressed.rs` | 150 | Bandit or gosec | ``nosec`" alone would be matched by the second as well.`` |
| `crates/sv-check/tests/suppressed.rs` | 151 | Bandit or gosec | ``nosec`"), "{why}");`` |
| `crates/sv-check/tests/suppressed.rs` | 194 | Bandit or gosec | ``nosec` line out of its SARIF and says`` |
| `crates/sv-check/tests/suppressed.rs` | 212 | Bandit or gosec | `nosec injection is reported");` |
| `crates/sv-check/tests/suppressed_codeql.rs` | 1 | CodeQL | ``lgtm[...]` or `codeql[...]` comment is still a finding (backlog 0509).`` |
| `crates/sv-check/tests/suppressed_codeql.rs` | 74 | CodeQL | ``lgtm[...]` on the line, 17 has `codeql[...]` on the line before, 22 on the line.`` |
| `crates/sv-check/tests/suppressed_codeql.rs` | 85 | CodeQL | ``lgtm[...]` on the line, 22 has `codeql[...]` on the line before, 28 on the line.`` |
| `crates/sv-cli/src/build_loop/placed_tests.rs` | 140 | Bandit or gosec | `nosec\nrun(host)\n",` |
| `crates/sv-cli/src/mcp/build_loop_went_same_tests.rs` | 48 | Bandit or gosec | `nosec\n"),` |
| `crates/sv-cli/src/mcp/since_still_reads_tests.rs` | 86 | Bandit or gosec | `nosec\nrun(host)\n",` |
| `crates/sv-cli/src/mcp/since_still_reads_tests.rs` | 97 | Bandit or gosec | `nosec\n"),` |
| `crates/sv-cli/src/tool_behavior_tests.rs` | 41 | Bandit or gosec | `nosec"),` |
| `crates/sv-cli/src/tool_behavior_tests.rs` | 123 | Bandit or gosec | `nosec")],` |
| `crates/sv-cli/tests/build_prompt_rules.rs` | 81 | Bandit or gosec | ``nosec", "`# nosec`"),`` |
| `crates/sv-cli/tests/look_away.rs` | 1 | Bandit or gosec | ``nosec` on`` |
| `crates/sv-cli/tests/look_away.rs` | 51 | Bandit or gosec | `nosec B104 <ok>\n";` |
| `crates/sv-cli/tests/tool_behavior_report.rs` | 181 | Bandit or gosec | `nosec\n");` |
| `docs/paper/figure-how-caught.html` | 84 | Bandit or gosec | `nosec</code> made bandit report nothing, and <code>sv</code> credited the silence as a clean run</td><td class="src">#73 → #79</td></tr>` |
| `docs/paper/figure-tests-faults-by-period.html` | 32 | Bandit or gosec | `nosec&#x27; made bandit report nothing and sv credited a clean run, V1.2.4 included</title></rect><rect x="18" y="36" width="14" height="14" rx="2" fill="var(--… (105214 more characters)` |

