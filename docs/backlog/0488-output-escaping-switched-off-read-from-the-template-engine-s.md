# Output escaping switched off, read from the template engine's own setting, and in part credited when it is on

**Status:** open

From the deep review of 13 October 2026 (backlog 0417), area: coverage, the requirements no check speaks to for a typical web app with sign-in (`auth`, `tls`, `internet`, `email`; level 2). A proposal, not a fault: how these could be assessed, in the order of preference the owner set (a static read; the running app behind the fence; an outside tool already adapted; showing the person the lines of their own code with a plain question; a hand check with a reason), each at its honest tier (ADR-022, ADR-050). Written by a read-only reviewer and read, not built, by the reviewing session; a requirement's text must be read again before any citation is written (a citation is a claim). The cross-cutting item it assumes is "A question to the owner carries the lines it rests on".

**Section: V1 Encoding and Sanitization (V1.1.1, V1.1.2, V1.3.3)**

- **V1.1.1** (L2), "decoded ... into a canonical form only once ... before processing the input further". Design
  question exists. Route 4: a locator for the decoding calls (`decodeURIComponent`, `urllib.parse.unquote`,
  `html.unescape`, `base64.b64decode`, `urldecode`, `URLDecoder.decode`, and their siblings) grouped by file, shown with
  the question "Each of these turns coded text back into plain text. Is it done before the input is checked, and only
  once?" Tier: *attested by the owner*, with the pointer. The partial-checks entry's finding-only rule for a decode
  nested in a decode is a fair addition (route 1, finding-only); the pointer is new.
- **V1.1.2** (L2), "output encoding and escaping either as a final step ... or by the interpreter itself". No
  question today. Route 1: the template engine in use and whether its own escaping is on is readable from the code:
  Jinja's `autoescape=False`, Django's `{% autoescape off %}` and `mark_safe`/`|safe`, Flask's `Markup`, EJS `<%-`,
  Handlebars `noEscape: true` and `{{{ }}}`, React `dangerouslySetInnerHTML`, Vue `v-html`, Go `text/template`
  imported where `html/template` would do. A switch-off is a finding citing V1.1.2 beside V1.2.1 (finding-only; most are
  already Semgrep rules mapped to V1.2.1 only, and `ast.html-from-value` is near it; the partial-checks entry proposes
  the map change). What is new and honest as a credit: *checked in part* when the engine found is one that escapes by
  default and no switch-off was read in the files `sv` read, with the scope "the template engine escapes by itself,
  and nothing read turns it off", and only when every language is read (ADR-054). Tier: `sv`'s own, in part.
- **V1.3.3** (L2), "data being passed to a potentially dangerous context is sanitized beforehand". Hand check exists;
  partial-checks says no useful check, and I agree: each context has its own requirement. Route 4 adds something: the
  pointer is the list of dangerous sinks the existing rules already locate (shell, SQL, HTML, path), shown as "the
  places StackVet looked for each context", so the hand check reads "for each of these, what cleans the value
  first?" No code to point at beyond that. Tier: *checked by hand*.

**The owner's decision, 13 October 2026** (asked by session stackvet-deep-review): build this first, alone, and try it on one chapter's questions before any of the fourteen proposals that assume it (0489 to 0500) is taken; what the trial shows may change the rest. The proposals wait for it.
