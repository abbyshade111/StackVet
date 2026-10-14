# The roadmap's status checked against the code (11 October 2026)

Moved here word for word from `docs/BACKLOG.md` on 14 October 2026 (backlog 0486), when the roadmap became a table whose
status column is read from each item's own status line; it was written there under the heading "Status checked against
the code, 11 October 2026", and is a dated record, so it belongs here.

Every item that was open, partly done, or claimed on 11 October was read against the code on `main` by a read-only
review, one batch at a time. Those reviews were a check of the notes, not a test run: where the code could not settle a
part, it is said to be unclear. Where a status line in an item file is stale, the review says so and the item needs its
line fixed, which is separate work. Phase 2 and phase 3 were not part of this pass.

**Phase 1, item 1 (the medium and low findings of 8 October): closed.** Every part has its done note, checked on 9 October.

**Phase 1, item 2 (the documentation review of 6 October): closed.** Items 1 and 3 to 7 are done; items 2, 4 and 8 were
closed earlier by their own notes.

**Phase 1, item 3 (the smaller open parts of older reviews):** the review of 27 September (0065) is done, and so are the
prompt-library blind spots (0021), the cato limits (0061), and the MCP server (0058). The deep review's parts 2 and 3
(0023 and 0024) are done in the code except two points: a test is needed to show that range events are read in version
order (0023, H18), and the per-run marker and the stdin seeding for one improvement (0024, improvement 5) are not
confirmed. The running-app checks (0029) are partly done: the tar archive read is now built (0241), and what remains is
7z and rar (0242), the container's CPU, memory, disk, and network readings (0243), and a real email change in the example
app (0244).

**Phase 4 (the product), in its order:**
- *The loop trials and the MCP server* (item 1): not part of this pass.
- *The prompt library* (item 2): the blind spots are closed (0021). The open parts of the library were not checked here.
- *Design-time help and prompts* (item 3): 0034 is partly done. Parts 1 to 4, 7 to 9 are in the code. Part 5 is open: the
  design prompts never ask for a planned feature. Part 6 is closed by the owner's decision not to build it (ADR-028).
- *The gap analysis of 7 October* (item 4): 0006 is partly done. Open: the audit of how much each credit rests on one
  sample (part 4), the outside-tool run for loop dodging (part 21). Unclear, and needs a look before the next pass: part 13(a),
  which one review says is built (`burst.rs`) and another says is open; parts 18 and 22(d) and (e). The partial checks (0085)
  are done except the error-handler signals (V16.5.4), and its status line still says two parts are open when the code shows
  them done. The gap analysis's open parts 1 to 33 otherwise were not checked here.
- *The dashboard* (item 5): 0227 is claimed by another session and partly built (ADR-086). Still open: the durations and tool
  versions for each run, the per-requirement drill-down, the running-app section, the build loop's calls and outcomes, and
  the data files that did not load.
- *Missing reviews of the new tools* (0147): open. The owner's CI results (reading an owner's own SARIF file) were agreed on
  9 October and are not claimed. V4.2.1 (request smuggling) is not built and not decided. The mock OIDC lacks V6.8.1,
  V10.5.3 and V6.8.4. The flow entry has not been run in a container.
- *Semgrep's pack* (0169): steps 1 to 3 are done. The template fixes on the `v1` branch and the coverage figure are
  unclear and need confirming.
- *The walk-through and the packaging* (0119, 0120): 0120's Homebrew route is done in this repository, but its formula
  lives in another repository, which was not checked. Open in 0119: a native download, the `--root` folder step in the
  guide, a plainer Docker line, and Cursor, which is still untried. The Windows install route waits for signed releases
  (0191 part 4).
- *Shorter CI and the merge queue* (0192): part 1, a shorter CI run, is open. Part 2, the merge queue, is the owner's
  hold of 9 October. Part 3, the nightly routine, is done.
- *Helping a person who is not technical* (0217): parts 1, 3, 5 and 7 are built and untried. Part 2 was replaced by the
  print-only `sv connect`. Part 4 is open until a trial measures it. Part 6 is 0120's. Part 8 is the owner's to arrange.
- *The logo* (0219): parts 1 and 4 are done; part 5 is approved and not built; parts 2 and 3 are the owner's.
- *Not started:* 0222 (a Cursor trial), 0224 (website mock-ups, which live outside the repository, so they cannot be
  checked from here), 0232 (more samples, groups A to H, none built yet), 0242 to 0244 (above). The claim on 0223 is not
  visible in the repository, and its routine runs first on 12 October, so it is unclear until then.
