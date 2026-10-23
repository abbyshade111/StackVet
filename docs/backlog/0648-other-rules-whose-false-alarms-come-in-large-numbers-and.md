# Other rules whose false alarms come in large numbers, and whether they can be fixed as the path rule's are

**Status:** open

Filed 23 October 2026 at the owner's word, after the triage of StackVet's findings on its own repository (backlog
0633): "investigate other high volume false positives like this one to see if those can be fixed as well". The path
rule's 269 false alarms on StackVet (backlog 0647) were one rule meeting code it cannot read well; other rules may do
the same to other apps, and a list too long to read hides the real fault in it.

To do: count, across the reports `sv` has made of the trial apps and the examples, which rules fire most often, and on
what; read a sample of each against the code; and for each rule whose findings are mostly false alarms, say why (the
rule cannot see where a value came from, it reads text that cannot run, it fires on test code), what would fix it, and
what the fix would cost in real faults it might then miss. Each rule worth fixing becomes an item of its own, with a test
that it still fires where it should. After 0647, which may answer the same question for one rule first.
