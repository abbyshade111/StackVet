# stackvet.dev's pages say Cache-Control no-transform, so Cloudflare adds no script the site's CSP would block

**Status:** partly done: after the owner deploys: check that no page carries Cloudflare's script, and measure compression against the numbers above

Filed 22 November 2026, at the owner's word ("Yes, file it and build it"), from the review of stackvet.dev's Cloudflare
settings that day.

**What is wrong.** Cloudflare adds a script of its own to every page of stackvet.dev, an inline `<script>` and
`/cdn-cgi/challenge-platform/scripts/precursor/main.js`: its JavaScript detections, part of its bot scoring. The site's
Content-Security-Policy (`tools/website.py`, `headers`) allows only the inline scripts the build wrote, by their hashes,
so browsers block it: it fails in every visitor's browser, gives Cloudflare nothing, and the pages served are not the
pages built. Cloudflare's documentation says JavaScript detections "cannot be disabled" for Bot Fight Mode customers,
and stackvet.dev's zone, on the Free plan with Bot Fight Mode off, still has it; it also says the script is not added
when the response carries `Cache-Control: no-transform`.

- **To do:** `headers()` writes `Cache-Control: public, max-age=0, must-revalidate, no-transform` for every path, keeping
  what the site is served with today and adding `no-transform`; ADR-132, a Later entry.
- **Check:** a test that `_headers` holds it; after the owner deploys, no page carries `cdn-cgi/challenge-platform`, and
  the pages are still fresh after a deploy (`max-age=0`).
- **Measure, before and after:** whether Cloudflare still compresses the files. Before, 22 November 2026, every file was
  sent brotli-compressed: the home page 7,487 bytes on the wire (21,703 raw), `/configvet` 8,013 (23,076), and
  `self-assessment/sbom.cdx.json` 6,282 (88,905). If `no-transform` stops compression, say so with the numbers, and the
  owner chooses whether the larger files are worth it.
