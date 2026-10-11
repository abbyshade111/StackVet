# Files of sv's own: an opt-in log and a crash file (10 October 2026)


**Decided by the owner, 10 October 2026** (observability review, part 3, I, backlog 0237): `sv` may keep files of its
own on the owner's computer, and only as the owner asks or when a failure needs them.

What was built:
- `SV_LOG`, when set to a path, is a file `sv` appends to: the time, each report stage's name, and the command's name.
  Never arguments, paths the owner typed, or what a report says. A line that cannot be written is dropped.
- A crash file, under the history folder, when a panic reaches the top of the run: the version, the command's name, and
  the place of the panic (the file and line in `sv`'s own source). Never the panic's message, which can name a path.

What was left out on purpose: the panic's message is not kept, because the message is where a path or a value can
appear. A reader who wants it has the terminal output, which the owner sees when the panic happens.
