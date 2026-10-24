# The version-pinning check reads a test fixture's requirements as what the app installs

**Status:** open

Found 23 October 2026, after backlog 0656 (ADR-140). With the inventory of what StackVet ships complete, its own report
still holds V15.1.2 at "needs attention", for one finding of `config.versions-pinned`:
`crates/sv-run/tests/fixtures/install-app/requirements.txt` has no lockfile. The finding is in test code, listed apart,
and still counts (ADR-023), so it outranks the inventory's credit.

ADR-140 decided that a fixture's lockfile is not what the app installs, for the inventory, while the
known-vulnerability checks still read every lockfile, since a sample's vulnerable package matters. The pinning check
asks the inventory's question (are the versions the app installs fixed?), so it may belong with the inventory; or a
fixture's unpinned file may be worth keeping as a finding, as test code's findings are. That is the owner's to decide,
since it changes what counts. To do: put the question to the owner, then build the answer, with a Later entry on
ADR-140.
