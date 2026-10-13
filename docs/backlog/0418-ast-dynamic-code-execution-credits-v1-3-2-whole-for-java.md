# `ast.dynamic-code-execution` credits V1.3.2 whole for Java without looking for SpEL, and three more AST rules credit whole from call lists with gaps

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: requirements and tests (citations). Severity: high. Verified against the code by the reviewing session before filing. The rule data was read by the reviewing session: Java's only taught call is `^eval$`, and Java has no `nothingToFind` entry, so any Java file read credits V1.3.2 whole.

- Where:
  - `data/ast-rules.json:5` (`ast.dynamic-code-execution`, `requirementIds: ["V1.3.2"]`, no `inPart`); its
    `functionPatterns.java` is `^eval$`, and Java has no `nothingToFind` entry, so a Java file read credits.
  - `crates/sv-check/src/ast.rs:2104-2187` (`clean_rules`: a rule with no finding in every language read credits
    `requirement_ids` whole unless `inPart`).
  - `data/ast-rules.json:66` (`ast.shell-command`, V1.2.5): `functionPatterns.python`
    `^(system|popen|getoutput|getstatusoutput)$`, `ruby` `^(system|exec|spawn)$`; the shell-true rule's Python
    list is `^(run|call|check_call|check_output|Popen)$` and is findings-only.
  - `data/ast-rules.json:407` (`ast.file-path-from-value`, V5.3.2): `functionPatterns.python` `^(open|send_file|FileResponse)$`.
  - `data/ast-rules.json:713` (`ast.open-redirect`, V3.7.2): JavaScript reads `res.redirect`, `res.location`,
    `redirect()`, `NextResponse.redirect`, and the browser's `location`.
- What is wrong: V1.3.2: "Verify that the application avoids the use of eval() or other dynamic code execution
  features such as Spring Expression Language (SpEL). …" The only language SpEL exists in is taught one call name,
  `eval`, which Java code never uses; a Spring app with `parser.parseExpression(userInput).getValue()` is credited
  "V1.3.2: N java files (the calls it reads: `eval`)" and the report says *checked*. The scope is honest about
  what was read, but the status is not what the requirement asks. The same shape, weaker: V1.2.5 ("protects
  against OS command injection") is credited whole with `asyncio.create_subprocess_shell(cmd)` in neither Python
  list and Ruby's `Open3.capture2(str)` / `IO.popen(str)` in none; V5.3.2 ("file paths for file operations …
  strict validation") is credited whole with `os.remove(path)`, `os.unlink`, `shutil.rmtree`, and
  `Path(name).read_text()` unread in Python; V3.7.2 ("only automatically redirect … where the destination appears
  on an allowlist") is credited whole with `res.writeHead(302, {Location: x})` and `res.setHeader("Location", x)`
  unread. On 13 October the project put `ast.weak-hash-function`, `ast.unsafe-deserialization`, and
  `ast.weak-cipher` in part for crediting "only approved" from a deny-list (0281, 0302); these four rest on the
  same kind of list and were not re-read. (`ast.sql-built-by-hand`'s list is broad enough across every taught
  language that I do not file it; `ast.shell-command-backticks` is exact for the two languages that have
  backticks.)
- What to do: add `inPart: true` to the four rules (the field and `clean_rules` support it since 0281), and
  regenerate; or widen the lists to the named calls and keep whole credit for the languages where the list is
  known complete. For V1.3.2 specifically, either teach Java `parseExpression`/`getValue` on a SpEL parser (as a
  finding) or mark the rule in part. Must not lose: a language it is not taught stays a gap, never a credit
  (ADR-054), and the scope keeps naming the calls read.
- How verified: read (the rule data, `clean_rules`, and the Java witnesses in `crates/sv-check/src/ast/tests.rs:997-1010`,
  which exercise `executeQuery` and `readObject`, not any dynamic-code call; no test has a SpEL case).
- Overlap: 0281 and 0302 (done): the same reasoning applied to three other rules; this names four rules they did
  not. 0388 (open) is the open-redirect rule's false positive on a fixed address, the finding side; this is the
  credit side.
- Needs a decision record?: yes: ADR-053's Later line of 13 October (rules that read code can credit in part) and
  ADR-054 (governs `ast.rs`); a Later line on ADR-053 naming the four rules, as 0281's did.
