# cv live cloudflare reads the zone's DNS records, to tell whether the SSL/TLS mode guards a connection

**Status:** open

Filed 22 November 2026, from the first try of `cv live cloudflare` on stackvet.dev (backlog 0732; ADR-157, "As built,
part 4, Cloudflare"). The SSL/TLS mode guards only the connection from Cloudflare to a server of the app, which a proxied
DNS record sends requests on to. stackvet.dev is served by a Worker alone and has no such server, so its mode, Full,
guarded nothing, yet `cv` reported it as a finding: `cv` does not read the DNS records, so it cannot tell. Each mode
finding now says so.

- **The owner's choice first, asked with a recommendation:** reading the records needs a fourth permission, **Zone >
  DNS > Read**. It reads every record of the zone, each one's name, type, address, and whether it is proxied, which is
  more than `cv` needs and may name internal hosts. The other way is to leave it, with the finding's words as they are.
- **If chosen:** one more `GET` (`/zones/{id}/dns_records`, proxied records only if Cloudflare can filter them), and the
  mode is a finding only where a proxied A, AAAA, or CNAME record points at a server; where the zone has none, the mode
  is said to guard nothing, credited for nothing; where the records cannot be read, the words stay as now. A record's
  address is never written in the report, only how many proxied records there are.
- **Check:** tests with a zone that has proxied records, one with none (a Worker's), and one whose records were refused.
- **Records:** ADR-157, a Later entry with the owner's choice.
