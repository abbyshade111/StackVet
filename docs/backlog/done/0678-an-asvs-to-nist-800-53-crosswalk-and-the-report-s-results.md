# An ASVS-to-NIST 800-53 crosswalk, and the report's results exported as OSCAL assessment results, so StackVet's evidence can feed a cATO pipeline

**Status:** done, 27 October 2026

Asked for by the owner on 26 October 2026, after a conversation about fitting StackVet into a DevSecOps and
continuous-authorization (cATO) pipeline. Most of what a pipeline needs is already there: the GitHub Action
(ADR-081), the signed image and its provenance (ADR-080), sealed reports (ADR-034), `--fail-on`, `--baseline`, and
`sv compare`, and `report.json`, `stackvet.sarif`, and `sbom.cdx.json` with every report. What is missing is the bridge
to the controls an authorizing official reads, NIST SP 800-53 Rev. 5, in the form their tools read, NIST's OSCAL.
ASVS 5.0's own text cites NIST SP 800-63B and SP 800-57 but never 800-53, and nothing in `sv` maps one to the other.

The plan, and what an export may and may not conclude, is the proposed record ADR-156. The owner answered its three
questions on 27 October 2026: an export only when asked (with an input on the Action that turns it on), the newest
OSCAL 1.x release pinned when the build starts, and assessment results alone at first, with no POA&M.

1. **The crosswalk.** Each ASVS 5.0 requirement against the 800-53 Rev. 5 controls and enhancements that ask the same
   thing, chosen by reading both texts, each pair with the few words they share, every requirement listed (empty when
   nothing in 800-53 asks the same), in a new data file with its line in `data/README.md`, read through
   `sv_frameworks::data` (ADR-036). OpenCRE's links may be read as a first draft; no pair is kept without reading both
   texts. Includes reading AISVS Appendix C's three existing 800-53 mappings.
   **Part status:** done, 27 October 2026
2. **The control text from NIST, not by hand.** A maintenance script, like `tools/atlas_references.py`, that reads the
   control titles and statements from a pinned release of NIST's OSCAL catalog for 800-53 Rev. 5, names any cited
   control a newer release renames, and refuses one it withdraws. Run outside the sandbox; `sv` fetches nothing.
   **Part status:** done, 27 October 2026
3. **The guard.** The citation guard (`crates/sv-check/tests/citations.rs`) holds the new crosswalk as it holds the
   Secure by Design one: shared vocabulary with both texts, and every requirement listed. Break a pair on purpose and
   watch it go red.
   **Part status:** done, 27 October 2026
4. **The export.** Reads a sealed report and writes OSCAL assessment results, mapping each status to an observation
   method and concluding `satisfied` or `not-satisfied` only where `sv` concluded *checked* or *needs attention*
   (ADR-156, Decision 3). Validated in tests against the pinned OSCAL schema, committed so the tests run offline; one
   test exports *checked in part* as `satisfied` on purpose and must fail.
   **Part status:** done, 27 October 2026
5. **The documents.** `docs/GETTING-STARTED.md` (how a pipeline runs the export and what it may conclude),
   `docs/ARCHITECTURE.md`, `docs/COVERAGE.md` if the crosswalk's reach is counted there, and the Action's new input
   that turns the export on. ADR-156 made accepted, with where the build differs from the plan.
   **Part status:** done, 27 October 2026
