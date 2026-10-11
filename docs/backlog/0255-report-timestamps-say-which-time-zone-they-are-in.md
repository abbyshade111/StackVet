# Report timestamps say which time zone they are in

**Status:** open

Found by the feedback on the Cursor build (11 October 2026). Report files carried timestamps that did not match the local
time the person was working in, so a fresh report briefly looked stale. Nothing in the report says which time zone its
times use.

What to decide: whether report times should be written in local time with the zone shown, or in UTC with "UTC" shown in
the same place every time. Either way, the report should label the zone, so a reader can compare it with a clock.
