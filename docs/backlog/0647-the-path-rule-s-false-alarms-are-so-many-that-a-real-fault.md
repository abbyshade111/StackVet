# The path rule's false alarms are so many that a real fault among them would be missed, and test code is read unevenly

**Status:** claimed by claude/0647, 23 October 2026

From the triage of StackVet's findings on its own repository (backlog 0633, 23 October 2026). The owner's word, 23 October 2026, asked how to handle the 269 *A file is opened at a path built from a value*
findings (`ast.file-path-from-value`, V5.3.2, low confidence) on StackVet's own code: "leave them counted for now as
recommended, but add a backlog item for after this one to investigate how we can fix the excessive false positives and
issue with test code, because otherwise actual issues will get overlooked."

What is known:

- **The rule cannot tell where a path came from.** It flags a file call whose path is not written out. StackVet reads
  the folder the person points it at, so nearly every one of the 269 is its own work, and the rule cannot tell a path
  typed at a terminal from one a stranger sent. With 269 such lines, one that does come from a request would be missed.
- **Test code is read unevenly.** A finding in test code counts and is said to be in test code (ADR-023), but the
  weak-key check skips test files altogether (`weak_keys.rs`, `is_test_path`), and Rust test modules that are files of
  their own are not read as test code at all (backlog 0644).
- **The bill of materials reads test fixtures as what the app ships.** StackVet's report on itself of 23 October 2026
  says its list is not complete (`sbom.incomplete`) because it read the sample lockfiles under
  `crates/sv-scan/tests/fixtures` (a `go.sum`, several `package-lock.json`, an unpinned `requirements.txt`) as if
  StackVet shipped them. A fixture's lockfile is not what the app installs. Left counted in 0633 meanwhile.
- **The database-query rule's `{ key: value }` alternative** (`ast.sql-built-by-hand`) flags any object key named
  like a query call (`query: [...]` in `crates/sv-run/assets/oidc-provider.mjs`), whatever the value; an array cannot
  be SQL text. Set aside for the owner to confirm in 0633 meanwhile.

To do: find out what would let the path rule tell a path that comes from outside from one that does not, in the
languages it reads (a request value reaching the call, as the rule already reads for some, or a value the app built
itself), and what that would cost in findings it would then miss; settle how every check treats test code, the same
way; and bring the answers to the owner before building. Then make StackVet's report on itself again.
