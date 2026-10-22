# Fix now: sv (StackVet)

Produced by `sv` 0.1.0 (commit 082ca7feb89f) on 2026-10-22 20:42:01 UTC, run 44f4f1614a4d.

**This file:** What needs attention, worst first, each with where it is and what to do: the findings of security.md that are things to fix. **For:** You or your AI coding tool, first. The end of `report.html` lists every file in this folder and who each is for.

This page is one part of the report, cut whole from it: it credits nothing on its own, and the counts stay in `compliance.md`. Like every file in this folder it is sealed with the report, so `sv` can tell whether it is the page it wrote.

## 312 things to fix

Worst first.

### [high] A shell command is built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 641

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-acfd01237534a838`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
   640  fn presence_of(adapter: &Adapter, program: &Path) -> (Presence, Option<String>) {
>  641      let mut command = Command::new(program);
   642      command.args(&adapter.version.args);
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1397

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-0a018f17af8260ce`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1396      if let Some(prepare) = &adapter.prepare {
> 1397          let mut command = Command::new(
  1398              programs
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1429

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-e4c31df1f514c767`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1428      }
> 1429      let mut command = Command::new(&programs.run);
  1430      for arg in &run_args {
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-check/src/production.rs` line 1387

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-dc441c03fcbbfcfb`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1386          let out = match run_capped(
> 1387              std::process::Command::new(curl_program()).args(self.args(&args)),
  1388              CURL_OUTPUT_LIMIT,
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-check/src/production.rs` line 1405

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-123a270b41511a06`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1404      pub fn available() -> bool {
> 1405          std::process::Command::new(curl_program())
  1406              .arg("--version")
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-check/src/production.rs` line 1434

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-450eba423a486d80`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1433          let out = match run_capped(
> 1434              std::process::Command::new(curl_program()).args(self.args(&[
  1435                  // curl refuses the handshake when no stapled status comes back, with its own exit
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-check/src/production.rs` line 1470

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-8c4c6516bf4134ea`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1469          let library = run_capped(
> 1470              std::process::Command::new(curl_program()).arg("--version"),
  1471              CURL_OUTPUT_LIMIT,
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-check/src/production.rs` line 1496

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-cdcb2b30976f9a99`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1495          match run_capped(
> 1496              std::process::Command::new(curl_program()).args(self.args(&args)),
  1497              CURL_OUTPUT_LIMIT,
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-cli/src/review.rs` line 167

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-b8255cc2791fc325`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
   166          &mut |args| {
>  167              let status = std::process::Command::new(
   168                  std::env::current_exe().context("finding this sv to run `sv report`")?,
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-run/src/docker.rs` line 415

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-2aa78fc8a25bc095`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
   414      fn docker(&self, args: &[&str]) -> Result<(i32, String), String> {
>  415          let mut c = Command::new(&self.binary);
   416          c.args(self.prepared(args));
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-run/src/docker.rs` line 456

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-ba373cca59ef9000`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
   455      ) -> Result<(i32, String), String> {
>  456          let mut c = Command::new(&self.binary);
   457          c.args(self.prepared(args));
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-run/src/docker.rs` line 480

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-af8e388e782217ad`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
   479          let cpus = self.cpus.get_or_init(|| {
>  480              let mut c = Command::new(&self.binary);
   481              c.args(["info", "--format", "{{.NCPU}}"]);
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-run/src/docker.rs` line 538

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-67b75b5dd895fb4a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
   537      fn docker_cleanup(&self, args: &[&str]) -> Result<(i32, String), String> {
>  538          let mut c = Command::new(&self.binary);
   539          c.args(args);
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-run/src/docker.rs` line 1053

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-9d5b78d479bd5495`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1052              let ran = crate::run_bounded(
> 1053                  Command::new(&self.binary).args(["exec", &app, "sh", "-c", test_command]),
  1054                  plan.test_limit,
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-run/src/docker.rs` line 1659

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-f8ca9db06120b902`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
  1658          args.extend_from_slice(command);
> 1659          let mut c = Command::new(&self.binary);
  1660          c.args(self.prepared(&args));
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A shell command is built from a value

**Where:** `crates/sv-run/src/docker/probes.rs` line 479

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-c112923bb0018fc0`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A command line is assembled from something other than fixed text and handed to the shell.

**The code.**

```text
   478              let script = crate::exec_session::loop_script(&mark);
>  479              let mut command = Command::new(&self.binary);
   480              command.args(self.prepared(&["exec", "-i", sidecar, "sh", "-c", &script]));
```

**Why it matters.** A semicolon or a backtick in the wrong place turns one command into two, and the second one is whatever the attacker wanted.

**What to do.** Pass the program and its arguments as a list instead of a string, so the shell never parses them. Where a shell is genuinely needed, validate the value against a fixed set first.

Evidence about: V1.2.5

Known as: CWE-78

### [high] A database query is built by joining text together

**Where:** `crates/sv-run/assets/oidc-provider.mjs` line 99

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-33511db0dfdd5d2a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A query is assembled from pieces rather than sent with its values kept separate.

**The code.**

```text
    98        if (requests.length < MOST_REQUESTS) {
>   99          requests.push({ method: req.method, path, query: [...new Set(url.searchParams.keys())] });
   100        } else {
```

**Why it matters.** A quote in someone's name is enough to change what the query does, and a deliberate one can read or delete any table the app can reach.

**What to do.** Send the query with placeholders and pass the values alongside it. Every database library supports this, and it is shorter than building the string.

Evidence about: V1.2.4

Known as: CWE-89

### [high] Certificate checking is switched off for every connection the app makes

**Where:** `crates/sv-check/src/cert_checks.rs` line 75

*How sure: confirmed.*

*Fingerprint: `50e82bceb57315d9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`crates/sv-check/src/cert_checks.rs` has `ssl._create_default_https_context = ssl._create_unverified_context`: Python's standard library then connects without checking certificates by default. A certificate is how the app knows it is talking to the real server and not someone in between, and this one line turns that check off for every server at once, whatever the rest of the code asks for.

**The code.**

```text
    74              r"\bssl\._create_default_https_context\s*=\s*(?:ssl\.)?_create_unverified_context\b",
>   75              "`ssl._create_default_https_context = ssl._create_unverified_context`",
    76              "Python's standard library then connects without checking certificates by default",
```

**Why it matters.** Anyone who can get between the app and a server it calls (on a shared network, a compromised router, or inside the hosting provider) can pose as that server: read the keys and data the app sends to it, and send back whatever answers they like.

**What to do.** Take the setting out. If it was added to reach one server with a certificate of its own (an internal service, a self-signed test server), trust that certificate for that server only: `NODE_EXTRA_CA_CERTS=/path/ca.pem` in Node.js, `verify="/path/ca.pem"` in Python's `requests`, or the CA added to that one client. If it is only for local development, keep it in a file named for development (`.env.development`, `Dockerfile.dev`), which `sv` leaves out.

Evidence about: V12.3.2, V12.3.4

Known as: CWE-295

### [high] Certificate checking is switched off for every connection the app makes

**Where:** `crates/sv-check/src/cert_checks.rs` line 80

*How sure: confirmed.*

*Fingerprint: `50e82bceb57315d9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`crates/sv-check/src/cert_checks.rs` has `https.globalAgent.options.rejectUnauthorized = false`: Node.js then accepts any certificate on every connection made through its default agent. A certificate is how the app knows it is talking to the real server and not someone in between, and this one line turns that check off for every server at once, whatever the rest of the code asks for.

**The code.**

```text
    79              r"\bhttps\.globalAgent\.options\.rejectUnauthorized\s*=\s*false\b",
>   80              "`https.globalAgent.options.rejectUnauthorized = false`",
    81              "Node.js then accepts any certificate on every connection made through its default agent",
```

**Why it matters.** Anyone who can get between the app and a server it calls (on a shared network, a compromised router, or inside the hosting provider) can pose as that server: read the keys and data the app sends to it, and send back whatever answers they like.

**What to do.** Take the setting out. If it was added to reach one server with a certificate of its own (an internal service, a self-signed test server), trust that certificate for that server only: `NODE_EXTRA_CA_CERTS=/path/ca.pem` in Node.js, `verify="/path/ca.pem"` in Python's `requests`, or the CA added to that one client. If it is only for local development, keep it in a file named for development (`.env.development`, `Dockerfile.dev`), which `sv` leaves out.

Evidence about: V12.3.2, V12.3.4

Known as: CWE-295

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `crates/sv-check/src/signed_in/fake_app.rs` line 3765

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-4693deda0985c0dc`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
  3764              user: "a@example.test".into(),
> 3765              password: "[redacted: Sv-0… (28 more characters)]".into(),
  3766          },
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `crates/sv-check/src/signed_in/fake_app.rs` line 3769

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-9f1834248a9e453c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
  3768              user: "b@example.test".into(),
> 3769              password: "[redacted: Sv-b… (28 more characters)]".into(),
  3770          },
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `crates/sv-check/src/signed_in/fake_app.rs` line 3773

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-c3415d2fbb3152e6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
  3772              user: "admin@example.test".into(),
> 3773              password: "[redacted: Sv-0… (28 more characters)]".into(),
  3774          }),
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `crates/sv-check/src/signed_in/fake_app.rs` line 3779

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-fae698228fee5b1e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
  3778                  user: "totp@example.test".into(),
> 3779                  password: "[redacted: Sv-7… (28 more characters)]".into(),
  3780              },
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `crates/sv-check/src/signed_in/signin.rs` line 88

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-c88e201337c61754`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
    87          user: user.to_owned(),
>   88          password: "[redacted: Wron… (28 more characters)]".to_owned(),
    89      };
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `crates/sv-check/src/signed_in/signin.rs` line 391

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-57c1cc31a3bfed11`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
   390              user: format!("nobody.{}", accounts.a.user),
>  391              password: "[redacted: this… (23 more characters)]".to_owned(),
   392          },
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `crates/sv-check/src/signed_in/signin.rs` line 398

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-b6d0df2846532909`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
   397          user: target.user.clone(),
>  398          password: "[redacted: Wron… (28 more characters)]".to_owned(),
   399      };
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `crates/sv-check/src/signed_in/signin.rs` line 734

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-4e5d85e2bc877583`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
   733              user: format!("sv-log-nobody-{tag}@example.test"),
>  734              password: "[redacted: Sv-L… (32 more characters)]".to_owned(),
   735          };
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] A value that looks like a credential is written into the code (`password`)

**Where:** `data/breached-password-evidence.json` line 3

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-789e8ddd626edc20`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`password` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder.

**The code.**

```text
     2    "_comment": "Why the V6.2.12 sign-up probe may call its password breached. `password` is BREACHED in crates/sv-check/src/signed_in/passwords.rs, and a test there fails if the two differ, so the pass… (533 more characters)
>    3    "password": "[redacted: 1qaz… (12 more characters)]",
     4    "sha1": "47DFD61B81026A5065A72623EC9430A703C9A756",
```

**Why it matters.** Anyone who can read the code — or the history it is kept in — has the credential.

**What to do.** Move the value into the app's environment or secret store, and change the credential: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [high] Semgrep Finding: typescript.react.security.react-insecure-request.react-insecure-request

**Where:** `crates/sv-run/assets/browser-driver.mjs` line 45

*Found by Semgrep (1.176.0), rule `typescript.react.security.react-insecure-request.react-insecure-request`.*

[The tool's own page about this rule](https://semgrep.dev/r/typescript.react.security.react-insecure-request.react-insecure-request)

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-e3e6add3149d8ed7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Unencrypted request over HTTP detected.

**The code.**

```text
    44      try {
>   45        const v = await (await fetch(`${DEVTOOLS}/json/version`)).json();
    46        return v.webSocketDebuggerUrl.replace(/^ws:\/\/[^/]+/, 'ws://127.0.0.1:9223');
```

**Why it matters.** Reported by Semgrep, which reads code the way its own community has learned to.

**What to do.** Unencrypted request over HTTP detected.

Evidence about: V12.3.1

Known as: CWE-319: Cleartext Transmission of Sensitive Information

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1314

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-27c13fcb2cad3a77`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1313      // A list left behind by an earlier run would vouch for files this one never read.
> 1314      std::fs::remove_file(&scanned_path).ok();
  1315      let files = if names_files {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1355

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4c2ddfb69b31dade`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1354      // A database left from an earlier run would be analyzed in place of this app's code.
> 1355      std::fs::remove_dir_all(&database).ok();
  1356      // The empty settings file, written only for a tool that asks for one. Written now, before the
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1363

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-b71534eb11bff306`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1362          .any(|a| a.contains("{config}"));
> 1363      if wants_settings && let Err(e) = std::fs::write(&settings, EMPTY_SETTINGS) {
  1364          return Outcome::NotRun {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1376

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4c7291663c69aac0`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1375      // or put there by somebody else, would be read as this run's.
> 1376      std::fs::remove_file(report_path).ok();
  1377      if std::fs::symlink_metadata(report_path).is_ok() {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1418

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-70e1a1c05432ad7d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1417          if let Some(detail) = failed {
> 1418              std::fs::remove_dir_all(&database).ok();
  1419              return Outcome::NotRun {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1443

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-08bb5117f5950e6f`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1442      let output = finish(prepared(&mut command, adapter), limit);
> 1443      std::fs::remove_dir_all(&database).ok();
  1444      std::fs::remove_file(&settings).ok();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1444

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2c57cee60405b313`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1443      std::fs::remove_dir_all(&database).ok();
> 1444      std::fs::remove_file(&settings).ok();
  1445      record.exit_code = output.as_ref().ok().and_then(|ran| ran.code);
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1456

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-20cd73c1c6111925`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1455      if output.interrupted {
> 1456          std::fs::remove_file(report_path).ok();
  1457          return Outcome::NotRun {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1467

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5940cc1f59600bbd`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1466      if output.timed_out {
> 1467          std::fs::remove_file(report_path).ok();
  1468          return Outcome::NotRun {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1491

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-08c8b7423efca3a3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1490          Some(code) => {
> 1491              std::fs::remove_file(report_path).ok();
  1492              return Outcome::NotRun {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1502

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-28c856ddb69cc2b2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1501          None => {
> 1502              std::fs::remove_file(report_path).ok();
  1503              return Outcome::NotRun {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1516

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-32709c4b889be610`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1515      let Some(text) = written
> 1516          .then(|| std::fs::read_to_string(report_path).ok())
  1517          .flatten()
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1528

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f66ef0ee42268ec3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1527      record.output = Some(text.clone());
> 1528      let scanned = std::fs::read_to_string(&scanned_path).ok();
  1529      std::fs::remove_file(&scanned_path).ok();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1529

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-1626bd1bd014e825`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1528      let scanned = std::fs::read_to_string(&scanned_path).ok();
> 1529      std::fs::remove_file(&scanned_path).ok();
  1530      match parse_sarif_relative_to(adapter, &text, app_dir) {
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 1778

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-30a0810392e1d02d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  1777          }
> 1778          std::fs::remove_file(&report_path).ok();
  1779      }
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters.rs` line 2679

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-ff89c3532b5cb9ee`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
  2678      fn drop(&mut self) {
> 2679          std::fs::remove_dir_all(&self.path).ok();
  2680      }
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/fence_tests.rs` line 47

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5a32476db06016e9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
    46      std::fs::create_dir_all(&app).unwrap();
>   47      std::fs::write(app.join("a.py"), "print(1)\n").unwrap();
    48      std::fs::write(app.join("b.py"), "print(2)\n").unwrap();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/fence_tests.rs` line 48

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-99e7a3e67275e55c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
    47      std::fs::write(app.join("a.py"), "print(1)\n").unwrap();
>   48      std::fs::write(app.join("b.py"), "print(2)\n").unwrap();
    49      app
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/not_run_cause_tests.rs` line 38

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-dc7e74726986a9d6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
    37      let dir = std::env::temp_dir().join(format!("sv-not-run-{name}-{}", std::process::id()));
>   38      std::fs::remove_dir_all(&dir).ok();
    39      std::fs::create_dir_all(dir.join("app")).unwrap();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/not_run_cause_tests.rs` line 40

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-eab4e321ed68f9d2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
    39      std::fs::create_dir_all(dir.join("app")).unwrap();
>   40      std::fs::write(dir.join("app/app.py"), "print(1)\n").unwrap();
    41      let tool = dir.join("faketool");
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/not_run_cause_tests.rs` line 65

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5a52021d276e83da`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
    64      );
>   65      std::fs::remove_dir_all(&dir).ok();
    66      assert!(run.ran.is_empty(), "{run:?}");
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/program_tests.rs` line 10

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-123b6e327e31c5dc`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
     9      let dir = std::env::temp_dir().join(format!("sv-program-{name}-{}", std::process::id()));
>   10      std::fs::remove_dir_all(&dir).ok();
    11      std::fs::create_dir_all(dir.join("app")).unwrap();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 30

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-53cbdc17a9c826cb`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
    29      let file: serde_json::Value = serde_json::from_str(
>   30          &std::fs::read_to_string(
    31              Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/adapters.json"),
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 73

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-71f212f29eabbbfb`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
    72      std::fs::create_dir_all(&app).unwrap();
>   73      std::fs::write(app.join("app.py"), "print(1)\n").unwrap();
    74      run_all(
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 249

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-cc1f7fe5191d6276`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   248      std::fs::create_dir_all(app.join("vendor")).unwrap();
>  249      std::fs::write(app.join("app.py"), "print(1)\n").unwrap();
   250      std::fs::write(app.join("web.js"), "console.log(1)\n").unwrap();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 250

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-75b89492388e3687`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   249      std::fs::write(app.join("app.py"), "print(1)\n").unwrap();
>  250      std::fs::write(app.join("web.js"), "console.log(1)\n").unwrap();
   251      std::fs::write(app.join("vendor/lib.py"), "print(2)\n").unwrap();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 251

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-02fd293e8c24e60a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   250      std::fs::write(app.join("web.js"), "console.log(1)\n").unwrap();
>  251      std::fs::write(app.join("vendor/lib.py"), "print(2)\n").unwrap();
   252      // `vendor/` holds installed code beside the manifest that explains it, as in family-hub; with
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 254

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-d0eb1ab0dd6a44b2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   253      // no manifest it would be the app's own (H6).
>  254      std::fs::write(app.join("requirements.txt"), "flask\n").unwrap();
   255      std::fs::write(dir.join("outside.py"), "print('outside the app')\n").unwrap();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 255

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e80223b6bb0fd877`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   254      std::fs::write(app.join("requirements.txt"), "flask\n").unwrap();
>  255      std::fs::write(dir.join("outside.py"), "print('outside the app')\n").unwrap();
   256      std::os::unix::fs::symlink(dir.join("outside.py"), app.join("linked.py")).unwrap();
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 267

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5f4b5f2d474a0901`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   266      );
>  267      let given = std::fs::read_to_string(&seen)
   268          .unwrap_or_default()
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 272

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f22172316d7adb64`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   271          .collect();
>  272      std::fs::remove_dir_all(&dir).ok();
   273      (outcome, given)
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 549

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-bcb36fcf0b1c70bb`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   548      let source = dir.join("quoted.sarif");
>  549      std::fs::write(&source, sarif.to_string()).unwrap();
   550      let text = format!(
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/adapters/stand_in_tests.rs` line 674

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-997f13b1f5c6fbd1`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   673      std::fs::create_dir_all(&app).unwrap();
>  674      std::fs::write(app.join("app.py"), "print(1)\n").unwrap();
   675      app
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/advisories.rs` line 195

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-efda60a82e13ab83`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**The code.**

```text
   194      };
>  195      let text = std::fs::read_to_string(path)
   196          .map_err(|e| (name(), format!("it could not be read ({e})")))?;
```

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ai_tool_cursor_tests.rs` line 16

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5407d2ebc661c3d2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ai_tool_cursor_tests.rs` line 24

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-fcfa5d3ab29a00ea`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ast/also_cites_tests.rs` line 100

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-db89bf3a2695ebda`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ast/also_cites_tests.rs` line 110

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-9f4636b5b40e76c0`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ast/call_list_in_part_tests.rs` line 25

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8331f09faa0690a4`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ast/call_list_in_part_tests.rs` line 27

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c8d2c4496e8e843d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ast/callers_tests.rs` line 15

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-cc96bceda4207ae2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ast/callers_tests.rs` line 18

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f08e5d622fadf405`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/ast/callers_tests.rs` line 21

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-0d2d04fcd9d6e505`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/bundled.rs` line 140

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2838599f1b041589`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config.rs` line 610

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-03d00d1b4d70060a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_ai_quote_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5cac838c824ebadb`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_ai_tests.rs` line 30

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c6b36e0bea2c874c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_ai_tests.rs` line 37

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8e94bb7edc41535b`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_no_commits_tests.rs` line 12

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-1386b51a83251c5c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_npmrc_tests.rs` line 27

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-85633a7d43e6cfd5`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_npmrc_tests.rs` line 31

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-7a454ade8c1f31e7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_pin_title_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-95de3ef5a50c5fad`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_pin_title_tests.rs` line 13

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-1658bf046cae8f72`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/config_pin_title_tests.rs` line 16

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-772493c48c2a0501`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/finding.rs` line 393

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3391f5f605cd03a8`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git.rs` line 225

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-26703bd686e92b82`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_ai_tests.rs` line 27

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-510e9330e8f17693`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_ai_tests.rs` line 45

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-eb1822ee14f581e3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_changes_tests.rs` line 39

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-fe226efb9bc89a56`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_changes_tests.rs` line 42

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-73277326b79dbe87`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_changes_tests.rs` line 43

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-fd6c435c2a53aae7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_changes_tests.rs` line 51

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3b73b837de5e0f84`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_changes_tests.rs` line 52

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-0b647f1153681ef3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_changes_tests.rs` line 53

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5f29661af909f9f3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_changes_tests.rs` line 63

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-aea8013cdcbe8a98`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_separator_tests.rs` line 28

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-6b53cfe5508707d6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/git_separator_tests.rs` line 31

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c60ffeed9b3d4073`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/level_hints.rs` line 30

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-47232d1bc4760f43`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/algorithm_tests.rs` line 17

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4169049df66b01f1`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/availability_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-fcb4ff797b58e997`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/encoding_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-d2cf53bf0e25e0a1`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/examples_tests.rs` line 137

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-1d7c6fc1119f7840`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/field_lists_tests.rs` line 18

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-7678a374ca51da5f`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/identity_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3d697e04eea78b5a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/level_three_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a8ce52f92c780a82`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/locate_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-6ebfd5c0a86a7aef`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/logging_tests.rs` line 18

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-b473759ea6fd1b7e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/memcache_media_tests.rs` line 20

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-456239f0e0b5c014`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/outside_tests.rs` line 17

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-36faa19e2a845841`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/session_lifetime_tests.rs` line 17

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-cca2497b97c85065`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/several_writes_tests.rs` line 17

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-6f756c7512315322`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/v6_tests.rs` line 18

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5dd838deac699993`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/locate/validation_tests.rs` line 20

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-bfe481383499bd67`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/logged_ids.rs` line 45

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-97b595bf81432d39`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/look_away_tests.rs` line 15

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-52f2f52b24ca03d3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/look_away_tests.rs` line 18

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-14ca98a7d90612b9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/model_files.rs` line 80

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a6831d073b1fecc6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/model_files.rs` line 90

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4eec4fb8e7c80234`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/model_files.rs` line 127

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-72f12ae8c9602665`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/outbound_rules.rs` line 71

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-7a58c99c737a877e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/review.rs` line 258

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-6d2144498dc98b26`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/review.rs` line 442

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-ce4a0e5d6cf0f6a1`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/review_renamed_title_tests.rs` line 35

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-fe6423bf17c01250`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/review_renamed_title_tests.rs` line 53

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2bf5cd0b02dc0386`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/review_still_reads_tests.rs` line 39

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-36c2dca090bdb5df`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/review_still_reads_tests.rs` line 42

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e2f40e58c30de3a3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/review_still_reads_tests.rs` line 44

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2fc9445972d31997`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/rich_text.rs` line 186

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e5a8b0185bd79e72`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom.rs` line 296

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-0b1f24d0c841cbb6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom.rs` line 403

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-0b9b0acab5ad85d8`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom.rs` line 601

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a3299120daa7e3b1`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom/not_compared_tests.rs` line 15

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-85121712e5d08fe9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom/not_compared_tests.rs` line 17

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2f20b2f3385697e2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom/not_compared_tests.rs` line 18

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-6375e3c1b648c334`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom/runs_on.rs` line 96

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-794c42fe3c585d36`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom/runs_on.rs` line 205

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-04cddcb4f252a93d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom/runs_on_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-044a58f707340c95`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/sbom/runs_on_tests.rs` line 13

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-110a2598c6f5ae94`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/seal.rs` line 153

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-096f91bd34172492`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/seal.rs` line 221

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f6523e5be0ae5371`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/security_contact.rs` line 32

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c610760d2ad1f0ed`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/security_contact_tests.rs` line 14

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e726e5bbd4a59bda`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/security_contact_tests.rs` line 24

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4c1146606d505a68`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/security_contact_tests.rs` line 26

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-509f35aba35babdd`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/security_contact_unreachable_tests.rs` line 12

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-7363464cc202e676`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/security_contact_unreachable_tests.rs` line 14

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-af8071b018805ce4`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/security_contact_unreachable_tests.rs` line 16

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-0f33fc8fc60e66bb`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/signed.rs` line 107

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-da9f496fdde35eb5`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/signed.rs` line 136

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-60c66638312019af`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/signed.rs` line 422

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-130cc384758d3c7a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/test_support.rs` line 44

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-ab95f9e456f1768e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/test_support.rs` line 67

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-9d352b4fc8497f52`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-check/src/workflows.rs` line 303

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-bfbf510d2a8caf10`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble.rs` line 285

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a8aef9a137d2fa96`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble.rs` line 1018

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-ef4c377d19d582b1`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble.rs` line 2100

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2cdd3c506ec9aa53`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble.rs` line 2143

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c51c577b9af02721`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble.rs` line 2981

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-022c205756eeaa81`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble.rs` line 3281

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-59477f3522673251`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble_env_file_tests.rs` line 10

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-402655e2f84f636c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble_env_file_tests.rs` line 15

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-624e49351697fdff`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble_own_tests_tests.rs` line 10

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8175f21567f40f3a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/assemble_own_tests_tests.rs` line 12

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-77080110395ab164`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/baseline.rs` line 49

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3ab3275f81a68eb9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/before_live.rs` line 41

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8d3970274b65fdc3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/board_link.rs` line 18

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a34f036bc60abc5d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop.rs` line 434

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8722e5b0e67b9dd3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop.rs` line 571

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-cff12c27be431ecb`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop.rs` line 692

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4cab01123db296cc`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop/chain_tests.rs` line 11

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-64a5e22310bd0a09`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop/chain_tests.rs` line 13

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5eeb77f4317ee626`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop/kept_apart_tests.rs` line 10

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-865e172f63dd4ab8`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop/limit_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4c02c1127cb0fcef`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop/limit_tests.rs` line 11

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-97a63ec3abe9555e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop/placed_tests.rs` line 109

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-b1733dea5084ce24`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop/placed_tests.rs` line 126

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3ea6b90fda1c2ad6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/build_loop/placed_tests.rs` line 130

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e7dc09551ba86107`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/bundle.rs` line 201

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-b77c9308736f8821`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/bundle.rs` line 334

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-15764a73bbaa4399`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/code_lines.rs` line 369

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-51c75102ee5e2747`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/code_lines/audience_question_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-145cd7b799976e92`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/code_lines/audience_question_tests.rs` line 11

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c333b1174623a390`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/code_lines/audience_question_tests.rs` line 12

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-89da567392d5f602`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/code_lines/audience_question_tests.rs` line 24

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-cab3516de4ab3e76`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/code_lines/code_lines_tests.rs` line 40

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-9336f788d7182ed4`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/compare.rs` line 39

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-37b8ede9a8638377`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/crash_notes.rs` line 39

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-1ac8b2dc604de472`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/doctor/unreachable_tests.rs` line 21

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-109358fc270a62fa`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/doctor/windows_tests.rs` line 23

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-aa4e053b5b88e101`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/explain.rs` line 386

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-efbe8de25a4f383e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/history.rs` line 142

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f3d6ff61f6341b70`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/history.rs` line 159

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f9c0bde3170cd838`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/history.rs` line 193

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4bbc6f7f94cc8b9c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/history.rs` line 224

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-1579083610ee1e5e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/history.rs` line 247

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2f7f6a9d24a41cd4`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/history.rs` line 257

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e671b0e7528f407e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/hook_record.rs` line 462

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-b8d0e639a7df4138`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/hook_record.rs` line 637

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-6901818687b86705`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/hook_record_claims_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-da83c23fbd635760`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/hook_record_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-bbcace7e9ca13a8b`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/lib.rs` line 1299

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2ba487c89a3fd3b6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/lib.rs` line 1389

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-41c84a459f24f47c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/lib.rs` line 1407

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2db02a5fa4bc403e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/lib.rs` line 1626

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-fefc11e098729860`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/lib.rs` line 1856

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-0e990910f08c3cf7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/lib.rs` line 1913

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-effd0822238b73d4`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/lib.rs` line 1936

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-494c0cf7a2ede163`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/main.rs` line 137

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-bae8edcc348235cf`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/main.rs` line 709

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8e43e78cbf71ebfc`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/main.rs` line 980

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-9b6b4d673694a42f`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/main.rs` line 1762

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e7261a78bb35b03a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/main.rs` line 1811

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-b0eff5b5cd48c7a0`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/main.rs` line 1875

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-b8e2fd06f4bdce38`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/build_loop_fingerprint_tests.rs` line 15

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c3d251e44c3d4f45`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/build_loop_first_calls_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-018d43c36b31331c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/build_loop_limit_tests.rs` line 11

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-754695a94372a2bc`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/build_loop_outcome_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-102f645a5e67e845`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/build_loop_outcome_tests.rs` line 17

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-01f0a5785b7b7baf`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/build_loop_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-cec0ecce4d22d8ec`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/build_loop_tests.rs` line 13

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3f2c48b745d87b27`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/build_loop_went_same_tests.rs` line 13

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c9a52d72e91711e2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/catalog.rs` line 831

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-b6bd93966396145d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/fifo_tests.rs` line 44

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-7a625483c0c879dc`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/flow_text_tests.rs` line 61

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a8a08f5b7c32799d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/marker_tests.rs` line 12

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f0507aebb8694ee9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/marker_tests.rs` line 13

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-dd9b8394a3f38123`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/marker_tests.rs` line 14

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-47106e99df43311d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/resources.rs` line 133

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3ff800a27ebc31ee`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/since_all_gone_tests.rs` line 11

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8528446a9e18196b`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/since_tests.rs` line 39

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8cadbbb8c6a71c8a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/since_tests.rs` line 41

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f1058da5e49eb553`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/since_tests.rs` line 54

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f0eb1cdbda685f6c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/mcp/terminal_words_tests.rs` line 10

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4e35048928811cbc`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/preflight.rs` line 733

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4404e67d14e1652a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/preflight/test_runner.rs` line 60

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3a4760ab1638ccf7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/preflight/test_runner.rs` line 107

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-11da0405e6adf9e2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/preflight/test_runner.rs` line 111

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-283c3f0055365f54`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/preflight/test_runner.rs` line 161

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4ae7f56375144f7c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/preflight/test_runner_tests.rs` line 13

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-608062f991f729fe`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/preflight/test_runner_tests.rs` line 16

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a0ad430eab05d7b5`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/preflight/test_runner_tests.rs` line 30

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-832d8000faa4b0e7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_again.rs` line 111

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-dd27b879f2a45086`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock.rs` line 86

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-5ab51a70c01249a1`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock.rs` line 119

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-9de6c36855734e5b`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock.rs` line 157

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-6b75ca58accecfe6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock.rs` line 193

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4e57abab87acd60a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock.rs` line 212

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-eb33f4f104840f56`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock.rs` line 213

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e09ce60eaa49c0d2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock.rs` line 433

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-07a3e0d622ca25aa`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock.rs` line 597

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-06849940bf6d53c1`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_lock/holder_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3de8524a681e6171`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_seal.rs` line 98

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a149444a3d5aaa88`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_seal.rs` line 175

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-9289b1e4c4a1c06a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/report_seal.rs` line 271

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-bafe0679bbce00bf`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review.rs` line 78

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-d9a52c440f1c37b7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review.rs` line 215

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-cfd2b2051f67b56a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review.rs` line 480

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-6baf9fd1876aed41`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review.rs` line 1412

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4f7c39721b71a72c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review.rs` line 2469

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-c0e71e21c04a6608`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/capabilities_tests.rs` line 18

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-7cfef62c22a083d7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/confirm_notes_tests.rs` line 11

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-7ffe1888f942bbd6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/mine_tests.rs` line 13

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3460785fddfb27bf`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/passphrase_prompts_tests.rs` line 12

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-bbc968ed2d3f9b90`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/passphrase_prompts_tests.rs` line 35

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-d9dc416d4ae6b80c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/passphrase_tests.rs` line 12

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-9dcc7ca5e19fc40a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/passphrase_tests.rs` line 23

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8772b3e4fcb43cd7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/policy_tests.rs` line 19

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-99144eb961c3cda5`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/scope_tests.rs` line 15

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-0d07d3d54029e094`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/ticks_tests.rs` line 21

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-ca7f4e15e0089c0b`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/ticks_tests.rs` line 39

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-633d15679cceb9f3`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/ticks_tests.rs` line 40

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-860ab0e01f5c8683`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/together_tests.rs` line 20

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-731cf4c30c50119b`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review/together_tests.rs` line 31

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f45a94799081ee6f`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-cli/src/review_gap_tests.rs` line 7

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-694d111a13ff1e12`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-frameworks/src/data.rs` line 125

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e7f5c442263d4fff`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-frameworks/src/load.rs` line 133

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-52439b1d79506d99`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-frameworks/src/load.rs` line 168

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-649deaedcf651038`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-frameworks/src/load.rs` line 211

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-3573b8ab50f4eae9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-manifest/src/lib.rs` line 2049

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-2281770de46a39e2`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-report/src/mark_tests.rs` line 10

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-cb4483977fba5e06`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-report/src/sarif.rs` line 236

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-490f162a30f739de`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-run/src/docker/stand_in_ready_tests.rs` line 9

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-96b376c3a1d3a329`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-run/src/docker/stand_in_ready_tests.rs` line 29

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-87ab74a997052882`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-run/src/install.rs` line 224

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e9eb66405b9c3d59`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-run/src/install.rs` line 256

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-7be211bec879bf51`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-run/src/install.rs` line 296

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-d6e23ab382de64c7`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/deps.rs` line 39

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-855dd8764a551303`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/deps.rs` line 95

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e8b6f68ba6a1c28e`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/ecosystems.rs` line 310

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-12ee0126da53fd9d`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/ecosystems.rs` line 584

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-29e88660a78476a0`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/ecosystems.rs` line 761

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-ad970b7404f76c5b`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/files.rs` line 261

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-a6469da251289ce6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/files.rs` line 286

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f8dd0804305a4f5a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/jvm.rs` line 215

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-429d30bbe4119b39`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/jvm.rs` line 263

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-f3071a04b3feea1c`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/jvm.rs` line 440

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-e80623b1ef366ef6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/jvm.rs` line 509

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-1350e23c6f2a4045`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/jvm.rs` line 589

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-4152fa7face7891a`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] A file is opened at a path built from a value

**Where:** `crates/sv-scan/src/lib.rs` line 403

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-615b462681cfcb72`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A file path used to read, write or delete a file is built from a value rather than written out, so the file operation goes wherever that value points.

**Why it matters.** If any part of the path comes from a request, a name like ../../etc/passwd reads or overwrites a file outside the folder the app meant to use.

**What to do.** Keep user-submitted file names out of file paths: store files under names the app generates, and look them up by id. Where a submitted name has to be used, reduce it to a bare name (secure_filename in Flask, path.basename in Node) and check that the resolved path is still inside the folder it should be in.

Evidence about: V5.3.2

Known as: CWE-22, CWE-73

### [medium] Data is encrypted in a way that does not show whether it was changed

**Where:** `crates/sv-check/src/ssh_format.rs` line 344

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-960fa456a8e584c0`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Data is encrypted with a block mode that keeps it secret but cannot tell when it has been altered: CBC, CTR, CFB, or OFB. Someone who can change the encrypted data can often change what it decrypts to, without knowing the key.

**Why it matters.** Encrypted data that is stored or sent somewhere a person can reach, such as a cookie, a token, or a file, can be tampered with, and the app will decrypt and trust the altered result. With CBC, error messages about padding can also let somebody decrypt it bit by bit.

**What to do.** Use an authenticated mode: AES-GCM or ChaCha20-Poly1305, or a library that does it for you (Fernet, libsodium's secretbox, Tink). If this code already signs the encrypted data with a separate HMAC and checks it before decrypting, it is correct as it is: this rule sees one call at a time and cannot see the HMAC, which is why it is only ever a finding and never a clean result.

Evidence about: V11.3.3

Known as: CWE-353

### [medium] A disallowed hash function is used

**Where:** `crates/sv-check/src/git.rs` line 239

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-8e9fea3a655e6a84`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

MD5 or SHA-1 is used as a hash function. Both have practical collision attacks, and approved-hash rules disallow them for any cryptographic use.

**Why it matters.** Anything that relies on the hash to tell two things apart, or to prove something was not changed, can be fooled by somebody who builds a second input with the same hash. Used for passwords, both are also fast enough to guess through quickly.

**What to do.** Use SHA-256 or stronger for integrity and signatures, and a password hashing function (Argon2id, scrypt, bcrypt) for passwords. If this hash is only a cache key or a file checksum with no security purpose, say so where it is used; Python lets you pass usedforsecurity=False.

Evidence about: V11.4.1

Known as: CWE-328, CWE-327

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 33

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-51407d94f731fe24`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here an RSA key of 1024 bits is made with Node's `generateKeyPair`. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 42

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-773ff0a0a336cbb6`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here an RSA key of 1024 bits is made with `rsa.generate_private_key`. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 51

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-f4a7b851527786f9`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here an RSA key of 1024 bits is made. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 69

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-f67d7b50db729def`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here an RSA key of 1024 bits is made with `openssl genrsa`. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 77

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-f583e301ae4d3b64`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here a DSA key is made, and DSA is no longer approved for making signatures. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 78

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-81d0f8d544ac5a55`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here a DSA key is made, and DSA is no longer approved for making signatures. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 87

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-c2da007d14db6185`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here an elliptic-curve key is made on a 192-bit curve, which is no longer approved. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 90

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-89615f4064a463be`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here an elliptic-curve key is made on a 192-bit curve, which is no longer approved. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] A key is made with a method or a size that is no longer approved

**Where:** `crates/sv-check/src/weak_keys.rs` line 96

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-2a6f5bc80fbc9e62`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

Here an RSA key is made with a public exponent of 3. Keys like this are no longer approved for new use: an RSA key under 2048 bits or on a 192-bit curve is within reach of a determined attacker, a DSA signature is no longer approved to make, and an exponent of 3 makes some RSA mistakes exploitable.

**Why it matters.** Whatever the key protects, a signature, a token, an encrypted message, can be forged or read by somebody able to break the key, and that gets cheaper every year.

**What to do.** Make RSA keys of 2048 bits or more (3072 for anything meant to last), with the exponent 65537; or use an elliptic-curve key on P-256 or larger, or Ed25519. Replace DSA with ECDSA or Ed25519. Keys already made the old way have to be made again; changing the code alone does not change them.

Evidence about: V11.6.1

Known as: CWE-326

### [medium] The list of what this app ships is not complete

**Where:** `sbom.cdx.json` line 1

*How sure: confirmed.*

*Fingerprint: `aa47d83191e7cc96`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`crates/sv-scan/tests/fixtures/go-stdlib-xml/go.sum` was read and no packages could be taken from it, so nothing from Go is listed; npm is in use but nothing readable says which versions are installed, so none of its packages are listed; `crates/sv-scan/tests/fixtures/node-websockets/package-lock.json` was read and no packages could be taken from it, so nothing from npm is listed; `crates/sv-scan/tests/fixtures/pinned-clean/package-lock.json` was read and no packages could be taken from it, so nothing from npm is listed; npm is in use but nothing readable says which versions are installed, so none of its packages are listed; `crates/sv-scan/tests/fixtures/unpinned-python/requirements.txt` has no lockfile beside it, and 2 of what it installs is asked for as a range, as any version, from an address or a folder, or from another file (flask, gunicorn), so it is not listed; a lockfile, such as one from `pip-compile --generate-hashes` or `uv pip compile`, is what `sv` reads; `crates/sv-scan/tests/fixtures/unreadable-language/package-lock.json` was read and no packages could be taken from it, so nothing from npm is listed; Ruby is in use but nothing readable says which versions are installed, so none of its packages are listed; npm is in use but nothing readable says which versions are installed, so none of its packages are listed; Go is in use but nothing readable says which versions are installed, so none of its packages are listed; `crates/sv-scan/tests/fixtures/dart-server/pubspec.yaml` declares Dart (pub) dependencies, and `sv` does not read that file, so none of its packages is listed or compared with known vulnerabilities; 3 package(s) are listed at the version asked for rather than the version installed

**Why it matters.** A bill of materials is worth the completeness of its list. Asked whether a compromised version of some library is in this app, nobody could answer from this document.

**What to do.** Commit a lockfile for every ecosystem in use, and install from it.

Evidence about: V15.1.2

Known as: CWE-1104

### [medium] An invite or access code is written into the code (`INVITE_FOR_STACKVET`)

**Where:** `crates/sv-check/src/secrets.rs` line 383

*How sure: likely. Rules like this one are usually right, not always.*

*Fingerprint: `v2-f251967223d823ea`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`INVITE_FOR_STACKVET` is set to a fixed value in the code itself. The name says it is a code people are given to sign up or get in, and the value is not a placeholder: anyone who can read the code can use it.

**Why it matters.** Anyone who can read the code — or the history it is kept in — can use the code to sign up or get in.

**What to do.** Make the code when the app needs one (at random, and once per invitation) or read it from the app's environment. If only a seed script uses it for testing, move it into the test setup, where a fixed value is expected.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

### [low] A value written under a credential's name is in the code, but it reads like a sentence (`WRONG_PASSWORD`)

**Where:** `crates/sv-check/src/secrets.rs` line 986

*How sure: possible. Rules like this one often misfire, so read the code before changing anything; if it is not a problem, it is a false alarm and the code can stay.*

*Fingerprint: `v2-8f23f11c621f9947`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

`WRONG_PASSWORD` is set to a value in the code itself. The name says it holds a credential, and the value is not a placeholder, but this reads like a sentence: several ordinary words ending in a period, question mark, or exclamation mark. That is most often a message shown to people, such as an error message, and sometimes a passphrase.

**Why it matters.** If it is a passphrase, anyone who can read the code — or the history it is kept in — has it. If it is a message, there is no harm.

**What to do.** This reads like a sentence, so read the line first. If it is a message shown to people, nothing needs changing: record it as a false alarm. If it is a passphrase, move it into the app's environment or secret store and change it: anything committed should be treated as known.

Evidence about: V13.3.1, SBD-AC-05

Known as: CWE-798, CWE-259

