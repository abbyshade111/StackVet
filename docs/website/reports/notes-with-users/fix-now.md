# Fix now: Notes with users

Produced by `sv` 0.3.0 (commit 7cb7878b2ec7) on 2026-11-22 03:24:15 UTC, run 90bae36f9345.

**This file:** What needs attention, worst first, each with where it is and what to do: the findings of security.md that are things to fix. **For:** You or your AI coding tool, first. The end of `report.html` lists every file in this folder and who each is for.

This page is one part of the report, cut whole from it: it credits nothing on its own, and the counts stay in `compliance.md`. Like every file in this folder it is sealed with the report, so `sv` can tell whether it is the page it wrote.

## 2 things to fix

Worst first.

### [medium] A password is stored, or a key made from it, with too few rounds of work

**Where:** `app.py` line 49

*How sure: confirmed.*

*Fingerprint: `v2-9d2dd4b16e6a08fd`. A person who has looked and found it a false alarm, or a risk to live with for now, can set it aside under `[[finding-review]]` in stackvet.toml.*

**Next step.** Fix it in the code and check again. If it is a false alarm, write a `[[finding-review]]` entry for it, naming its rule, file and fingerprint, and confirm it with `sv review` in your own terminal; until then it counts.

A password is hashed for storage, or a key derived from it, with PBKDF2, and the number of rounds written into the code is below what OWASP recommends: 600,000 for SHA-256 and 210,000 for SHA-512 (SHA-1 needs 1,300,000). A count below 210,000 is reported whatever the hash; one below 600,000 where the call itself names SHA-256, and one below 1,300,000 where it names SHA-1.

**The code.**

```text
    48  def hash_password(password, salt):
>   49      return hashlib.pbkdf2_hmac("sha256", password.encode(), salt.encode(), 200_000).hex()
    50  
```

**Why it matters.** Anyone who gets hold of what the key protects, such as an encrypted backup, an export, or a stored hash, can try guesses at the password quickly on ordinary hardware, and most people's passwords fall to a fast guesser.

**What to do.** Raise the number of rounds to at least 600,000 for PBKDF2 with SHA-256 (210,000 with SHA-512, and 1,300,000 with SHA-1, which is better replaced), or use a password hashing function made for this, such as Argon2id or scrypt. Keep the count where it can be raised later without breaking what was already stored.

Evidence about: V11.4.2, V11.4.4

Known as: CWE-916

### 1 low-severity finding, by rule

Each is in full, with where and how to fix it, in `low-findings.md`.

- `config.security-contact` — 1 time, in `SECURITY.md`

