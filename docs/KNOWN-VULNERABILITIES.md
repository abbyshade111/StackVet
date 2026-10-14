# Checking the packages against known vulnerabilities

Whether any package the app uses has a published vulnerability is *not assessed* until you give
StackVet a copy of the list of known ones. It never downloads that list itself: the names of the
packages your app depends on are yours, and a check that quietly sends them somewhere is one you did not
agree to. The list comes from OSV, a free public database, as one zip file per kind of package:

| Kind of package | Download |
|---|---|
| JavaScript (npm) | https://osv-vulnerabilities.storage.googleapis.com/npm/all.zip |
| Python | https://osv-vulnerabilities.storage.googleapis.com/PyPI/all.zip |
| Rust | https://osv-vulnerabilities.storage.googleapis.com/crates.io/all.zip |
| Ruby | https://osv-vulnerabilities.storage.googleapis.com/RubyGems/all.zip |
| PHP | https://osv-vulnerabilities.storage.googleapis.com/Packagist/all.zip |
| Go | https://osv-vulnerabilities.storage.googleapis.com/Go/all.zip |

You do not need to work out which ones your app needs: run `sv audit .` without anything more and it
names the kinds your app uses, with the address of each download. Make a folder called `osv` beside
the app, unpack each download into a folder of its own inside it (for example `osv/PyPI`), and run
`sv audit . --advisories ./osv`, or `sv report . --advisories ./osv` to put the result in the report.
The list grows every day, so download it again before a check you rely on; a copy from last month says
nothing about what was found since. A kind of package with no download here is reported as not assessed.
