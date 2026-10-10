# Files of sv's own: an optional log and a crash file (observability review, part 3, I)

**Status:** done, 10 October 2026

The owner's word, 10 October 2026, asked with the three other open decisions of that day: yes (part 3, I, of backlog 0226).

Files of `sv`'s own: an opt-in `SV_LOG` file of stages and tools, and a crash file under the history folder (the
version, the command's name, and the place; never arguments or paths). Changes what `sv` writes on the owner's
computer, so its record is written with the build. Small each; each part claimed on its own.

**Built 10 October 2026:** both parts. `SV_LOG` names a file that `sv` appends to, opt-in: the time, the name of each report stage, and the name of the command, never its arguments or any path (`crates/sv-cli/src/own_log.rs`). And a crash file under the history folder when a panic reaches `main`: the version, the command's name, and the place of the panic, never its message or arguments (`write_crash_file` in `crates/sv-cli/src/main.rs`). Test: `crates/sv-cli/tests/own_files.rs`, three tests, including one that puts a typed folder in the arguments and checks it does not reach the file.
