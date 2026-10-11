# sv review and the report disagree: 'nothing waiting' beside a needs-attention item with no review path

**Status:** claimed by securevibe-trial-prep, 11 October 2026

Found by the feedback on the Cursor build (11 October 2026; design entry 0368). The owner's `sv review` run said nothing
was waiting, while the report still listed the f-string finding as needing attention. `sv review` goes through the
entries in `stackvet.toml` not yet sealed (set-asides and confirmations); a finding with no such entry is not offered at
all. The agent first advised recording it as a false alarm, which would have been wrong, and was then corrected.

What to decide: whether `sv review` should say, beside "nothing waiting", how many report items still need attention and
why none of them has a review path; and whether a finding with no review path should name its next step in the report
(fix in code, or an owner decision). The guide already says a real finding is fixed in code (section 5), so this item is
about what `sv` says, not what the guide says. Only the owner can set aside a finding, so the fix must not offer a
path that lets the tool do it.
