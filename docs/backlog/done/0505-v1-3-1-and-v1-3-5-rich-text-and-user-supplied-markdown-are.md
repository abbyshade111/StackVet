# V1.3.1 and V1.3.5, rich text and user-supplied Markdown, are turned on only by an AI feature

**Status:** done, 14 October 2026

**Queued on the coordination board** (`docs/coordination/board.md`, backlog 0273): taken only through the board's `assign`, never from the roadmap.

Found 14 October 2026 by session wording, building backlog 0403. Severity: unknown until read against the requirements.

- Where: `data/knowledge/applicability.json`, the rules for `V1.3.1` and `V1.3.5`, each `"condition": "ai"` alone, and
  not replaced in `data/applicability-v2.json`.
- What is wrong, as far as read: V1.3.1 is about sanitizing HTML from a rich-text (WYSIWYG) editor, and V1.3.5 about
  Markdown, templates, or stylesheets a user supplies. Both are turned on only when the app has an AI feature, so an
  app with no AI and a rich-text editor of its own has them ruled out as not applying. Their reasons, written for
  SecureVibe v1's app, said "This app has no rich-text (WYSIWYG) editor" and "user text is always shown as plain text",
  which no condition establishes; 0403 reworded them to say what the condition does ("This app has no AI features, so
  no HTML from an AI model's answers reaches its pages"), and left the gating as it is.
- What to decide: whether these two apply to every app with sign-in or text input (`always`, or a condition of their
  own such as a rich-text editor), with `ai` as one of the rules that turns them on. That changes what applies, so it
  is a decision with a record of its own, and the citation's text should be read first.
- How verified: read the two rules and their reasons; not read against the ASVS text yet.

**The owner's decision, 14 October 2026:** option A. "A new rich-text capability in stackvet.toml, asked like the
others; unanswered, V1.3.1 and V1.3.5 are not assessed; yes, or an AI feature, makes them apply." The record is
proposed in the build (ADR-117), and the waiting file goes in the same pull request. Read against the ASVS text: V1.3.1
asks about "untrusted HTML input from WYSIWYG editors or similar", V1.3.5 about "user-supplied scriptable or expression
template language content, such as Markdown, CSS or XSL stylesheets, BBCode, or similar"; neither is about AI.
