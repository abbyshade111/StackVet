# The SARIF file's notes on what was not examined are notes, not warnings, and the report says what the file holds

**Status:** claimed by securevibe-trial-prep, 13 October 2026

The owner's decision of 13 October 2026, from the Antigravity trial's `findings.sarif`. It held no results at all,
only nine "Not examined: ..." notifications at level `warning`. To the owner the file's name and those warnings read as
problems with the app: "the word finding would make me think an issue, so seeing things in there makes me think there
are issues about the app."

To build:
- **The notifications for what was not examined** go at SARIF level `note`, not `warning`. They stay in the file, said
  plainly, so an empty `results` still never reads as a clean bill of health, which is why they were added. But they no
  longer look like problems in an editor or on GitHub. A decision record: it changes how a report's limits are shown.
- **`report.html` says what the file is,** beside its name: for code editors and GitHub, how many findings it holds,
  and how many notes on what was not examined, so nobody has to open it to learn that "0 findings" is what it says.
- **Tests** for both, with a deliberate break each.
