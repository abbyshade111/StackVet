# The shell-command rule flags Rust's Command::new and Go's exec.Command, which start a program with no shell

**Status:** claimed by claude/0633-triage, 23 October 2026

From the triage of StackVet's findings on its own repository (backlog 0633, 23 October 2026). 16 of the 312 were *A shell command is built from a value* (`ast.shell-command`, V1.2.5, CWE-78), every
one at a Rust `Command::new(value)`: `adapters.rs`, `production.rs`, `review.rs`, `docker.rs`, and `docker/probes.rs`.

Rust's `std::process::Command::new(program)` starts `program` with its arguments as a list, and no shell reads them,
so a semicolon or a backtick in a value is one more character in one argument, not a second command. That is the
parameterized call V1.2.5 asks for ("operating system calls use parameterized OS queries"), not a failure of it. Go's
`exec.Command(name, args...)` is the same, and the rule's Go query flags it the same way.

The rule's Rust and Go queries each have two other alternatives, which match a shell named in the code
(`Command::new("sh").arg("-c")...`, `exec.Command("bash", "-c", value)`), and those stay. To do: in
`data/ast-rules.json`, stop the plain `Command::new(value)` and `exec.Command(value)` from matching, keeping the
shell alternatives; say in the rule's `looksForIn` for Rust and Go what it reads; and test that the shell forms still
fire and the plain ones do not, in both languages. ADR-135, proposed.
