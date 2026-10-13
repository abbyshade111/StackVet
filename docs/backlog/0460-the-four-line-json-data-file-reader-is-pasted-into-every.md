# The four-line JSON data-file reader is pasted into every `load`

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: code quality and duplication. Severity: low. Verified against the code by the reviewing session before filing.

- Where: verbatim in `crates/sv-check/src/human.rs:46-50`, `design.rs:111-115`, `adapters.rs:327-331`,
  `secrets.rs:88-92`, `coding_rules.rs:82-86`; the same shape (`read_to_string` with "reading {path}", then
  `serde_json::from_str` with `sv_frameworks::data::not_understood(path)`) in the ten other files that call
  `not_understood` (`notes.rs`, `prompts.rs`, `ast.rs`, `sv-scan/src/lib.rs`, `jvm.rs`, `brief.rs`, `explain.rs`,
  `threats.rs`, `applicability.rs`, `data.rs`).
- What is wrong: fifteen copies of the one idiom ADR-036 exists to centralize; the "reading {}" wording and the
  `not_understood` call are the two halves of one message kept apart.
- What to do: `sv_frameworks::data::read_json::<T: DeserializeOwned>(path) -> anyhow::Result<T>` beside `file`, used
  by every loader; the validation after it stays where it is. ADR-036's Governs list already names `data.rs`.
- How verified: read.
- Overlap: 0291 (done; requirements.txt readers), 0293 (done; compiled-in files). Different files.
- Needs a decision record?: no ("ADR-036: unchanged, because the read still goes through data.rs").
