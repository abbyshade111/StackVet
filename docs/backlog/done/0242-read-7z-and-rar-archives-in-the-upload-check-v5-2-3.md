# Read 7z and rar archives in the upload check (V5.2.3)

**Status:** done, 11 October 2026

From the open part of 0029, part 15. Only zip and gzip are read. 7z and rar each need a reader of their own, and rar's format is proprietary, so this may stay a known limit. Decide first whether either is worth reading; the tar item comes before it.

## The decision, 11 October 2026 (for the owner)

The tar item (0241) is done. What the check does today: it writes its own zip, gzip, and tar files, with no library,
and reads them back with Python's standard library (`zipfile`, `gzip`, `tarfile`). Each format it sends must be
written by hand here and checked by a reader that is not `sv`'s own.

- **rar:** no open writer exists, and the format is proprietary. The check cannot write a rar that unpacks to a stated
  size. Recommendation: keep rar a known limit, named in the report as not sent.
- **7z:** the format is open, so a file can be written by hand. Two things make it harder than tar: a 7z that unpacks
  to one gigabyte must be compressed (a stored 7z of that size is too big to send), so it needs a hand-written LZMA2
  encoder for runs of zeros, as the deflate writer already does; and no reader is in Python's standard library, so
  the test would need one.

The owner's choice, one of:

- **Option A, keep both as known limits**, closing 0242 with a note. No code, no new tool.
- **Option B, build 7z only**, with a reader for tests. The reader is a new tool, either a Python package (`py7zr`) or the
   `7z` program. Tests only, not shipped with `sv`, but a dependency all the same: it needs its own decision record,
   and the choice of reader needs the owner's yes.
- **Option C, build 7z with a hand-written reader check**, no tool. Possible for the stored and zeros cases, but the test would
   then only prove what the writer itself says.

Recommendation: option A for rar, and for 7z option B only if the owner wants 7z in the report's claims; otherwise
option A for both.

**The owner's choice, 11 October 2026:** option A for rar (a known limit, said in the report), and option B for 7z.
Asked which reader the 7z test uses, the owner chose `py7zr`, a Python package, for the tests only: it is not shipped
with `sv`, and it reads the files `sv` writes without sharing code with the writer. Its use needs its own decision
record and a CI install step, both in the pull request that builds it. Sent by a stored 7z (no compression), capped
the way tar is (ADR-046), so a 7z holding more files than one is not tested for `max-files`, as tar's is not.
 Until the owner chooses, nothing here is built.

Seen again on 12 October 2026: Windows, runs https://github.com/abbyshade111/StackVet/actions/runs/38165003104 (commit
c00ed36, pull request #1451, the 7z build, which brought it), 38168920237 (648ada8), 38171180441 (a608c66), and
https://github.com/abbyshade111/StackVet/actions/runs/38172814367 (commit 8b1bc14), 11 October 18:43 to 20:34 UTC. One
test failed in each, `signed_in::archives::tests::each_stored_7z_unpacks_to_exactly_what_it_says_in_py7zr`
(2954 passed, 1 failed in the last): "python3 refused the archive: ModuleNotFoundError: No module named 'py7zr'".
Windows CI had no `py7zr` installed. Mended by pull request #1452 (`690f75a`, the Windows install step for `py7zr`),
merged 20:35 UTC; every Windows run on `main` since has passed (3035 passed, 0 failed on `ae95f06`). Nothing left to do.
