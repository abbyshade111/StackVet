# Credit V1.1.2 in part when the template engine escapes by itself and nothing read turns it off

**Status:** open

Split from backlog 0488 (V1.1.2), whose finding half is built: the rules that read a template engine's escaping switched
off (`autoescape=False`, `mark_safe`, `Markup`, `|safe`, `{% autoescape off %}`, EJS `<%-`, Mustache and Pug's
unescaped forms, Vue's `v-html`, React's `dangerouslySetInnerHTML`, Go's `text/template`) now name V1.1.2 beside V1.2.1,
only ever as a finding (ADR-018, Later, 15 October 2026).

What is left is the credit. V1.1.2 asks that output encoding happen as the final step before the interpreter, or in
the interpreter itself. A template engine that escapes by default does the second. So `sv` may credit V1.1.2 *in
part* (ADR-053) when all of these hold:

- the app's packages show an engine that escapes by default (Django templates, Jinja2 under Flask, React, Vue, EJS's
  `<%=`, Handlebars, Go's `html/template`), and none that does not (Mako, Jinja2 used alone with no `autoescape`);
- every rule above ran over the files it reads and found nothing;
- every language in the app is one `sv` reads (ADR-054), since a file it could not read may switch escaping off.

The scope shown: "the template engine escapes by itself, and nothing read turns it off". It is never *checked*:
HTML put together by hand outside the engine, and output to other interpreters (SQL, shell), are not seen by it.

Needs a decision record (what counts as evidence): a Later entry on ADR-053 or a new ADR. Tests must go red with the
engine missing from the packages, with one switch-off present, and with an unread language in the app.
