# Say exactly which data made a report (observability review, part 3, B)

**Status:** done, 10 October 2026

The owner's word, 10 October 2026, asked with the three other open decisions of that day: yes (part 3, B, of "A deep review of sv for observability", backlog 0226).

Say exactly which data made a report: a hash per data file beside the one over the folder that ADR-083 already
keeps, and a mark when `sv` was built from source with changes not committed. It changes what a report claims about
where it came from, so its record is written with the build. Small.

**Built 10 October 2026:** the run record now holds a SHA-256 for each data file, as a list of file and hash beside the folder's own (`sv_data_files`, `crates/sv-report/src/lib.rs`, from `crates/sv-cli/src/report_lock.rs`), and a `made_by` mark, `uncommitted_changes`, set when the checkout had changes to tracked files not committed (`crates/sv-cli/build.rs`). Test: `crates/sv-cli/tests/history_inputs.rs`; broken on purpose, it failed.
