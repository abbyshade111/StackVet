# A build-loop record the AI tool cannot quietly rewrite (observability review, part 3, J), if the paper relies on it

**Status:** done, 10 October 2026

The owner's word, 10 October 2026, asked with the three other open decisions of that day: yes if the paper relies on the build-loop record (part 3, J, of backlog 0226); the paper's sessions say
whether it does before this is claimed.

A second copy of the build-loop record beside the history, outside the app folder where the AI tool works, and the
report saying whether the two agree. ADR-076 weighed and declined keeping the record only there. Medium.

**Built 10 October 2026, option 3 as the owner chose it:** each line of the build-loop record is chained to the one before it. Its `chain` is a SHA-256 over the previous line's chain and this line's own text without the chain (`crates/sv-cli/src/build_loop.rs`, `chain_of`, `check_chain`). A report says whether the chain holds, the line where it breaks if not, and the head: the last chain that checked out (`crates/sv-report/src/lib.rs`, `BuildLoop`). The limit, said in the report: a rewrite of the whole record can recompute every hash and keep a valid chain, so the head is what a report keeps for comparison with an earlier one. Tests: `crates/sv-cli/src/build_loop/chain_tests.rs`, four; broken on purpose by accepting every line, the edit and removal tests failed.
