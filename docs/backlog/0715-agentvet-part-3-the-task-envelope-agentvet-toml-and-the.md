# AgentVet part 3: the task envelope, agentvet.toml, and the detectors that hold a tool to it

**Status:** claimed by Claude Code session (AgentVet), 21 November 2026

Asked for by the owner on 21 November 2026 ("start the task envelope and detectors once 0713 lands"). Step 2 of the
build order in the design entry "AgentVet, a companion that vets the agent: what it did, what it said, and where the
two part (21 November 2026)", and ADR-159's decision 8, with the owner's answer that the envelope is a file of its own,
`agentvet.toml`. Each part lands on its own, and ADR-159's "As built" grows with each.

1. **The envelope's format and its reader.** `agentvet.toml`, beside the app: the task in a sentence; the folders the
   tool may write; the commands it may run; the hosts it may reach; the files that guard oversight, which it may not
   change (its own instruction and permission files, hooks, CI workflows, the envelope itself, with a default list
   when none is given); and the actions that must ask first. A line left out is a question in the report, never a
   pass. A file `av` cannot read is said, with its line, and nothing is judged against it. Who wrote the envelope is
   not settled by the file: until the owner's word is recorded, the report says it may be the tool's. `av` writes
   nothing; an example to copy is in the documents. The format's choices go into ADR-159 as built, and any that is
   the owner's to make is asked first.
   **Part status:** done, 21 November 2026
2. **Outside the envelope, and tampering with oversight.** Over what `av report` already reads (git's history, and the
   hook record when asked for): a file the tool wrote or a commit of its changed outside the folders allowed; a
   command it ran that the envelope does not allow; a file that guards oversight changed. Each is listed with where
   it was seen and how far that record can be relied on. What cannot be held to the envelope from these records
   (the hosts reached, which no record here shows) is said every time. Each has a fixture that trips it and one that
   does not, and a test that fails with the detector taken out.
   **Part status:** done, 21 November 2026
3. **The rest of the six groups, from the records that exist.** Speaking as the owner (an entry or a seal line written
   in the owner's name by a commit of the tool's); trying again after a refusal, as far as the build-loop record's
   refused calls show it; and the two groups the records already feed (a claim against the record, gaming a check)
   gathered under their group's name in the report, so the report reads as the design's table does. A group no
   record can show yet says so.
   **Part status:** open
